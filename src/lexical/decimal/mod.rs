use super::lexical_form;
use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Numeric sign.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Sign {
	Negative,
	Zero,
	Positive,
}

impl Sign {
	pub fn is_positive(&self) -> bool {
		matches!(self, Self::Positive)
	}

	pub fn is_negative(&self) -> bool {
		matches!(self, Self::Negative)
	}

	pub fn is_zero(&self) -> bool {
		matches!(self, Self::Zero)
	}
}

/// Error thrown when a conversion function overflowed.
pub struct Overflow;

mod integer;

pub use integer::*;

lexical_form! {
	/// Decimal number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#decimal>
	ty: Decimal,

	/// Owned decimal number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#decimal>
	buffer: DecimalBuf,

	/// Creates a new decimal from a string.
	///
	/// If the input string is ot a [valid XSD decimal](https://www.w3.org/TR/xmlschema-2/#decimal),
	/// an [`InvalidDecimal`] error is returned.
	new,

	/// Creates a new decimal from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD decimal](https://www.w3.org/TR/xmlschema-2/#decimal).
	new_unchecked,

	value: crate::Decimal,
	error: InvalidDecimal,
	as_ref: as_decimal,
	parent_forms: {}
}

impl Decimal {
	/// Returns `true` if `self` is positive
	/// and `false` is the number is zero or negative.
	pub fn is_positive(&self) -> bool {
		let mut sign_positive = true;
		for c in &self.0 {
			match c {
				b'+' | b'0' | b'.' => (),
				b'-' => sign_positive = false,
				_ => return sign_positive,
			}
		}

		false
	}

	/// Returns `true` if `self` is negative
	/// and `false` is the number is zero or positive.
	pub fn is_negative(&self) -> bool {
		let mut sign_negative = true;
		for c in &self.0 {
			match c {
				b'-' | b'0' | b'.' => (),
				b'+' => sign_negative = false,
				_ => return sign_negative,
			}
		}

		false
	}

	/// Returns `true` if `self` is zero
	/// and `false` otherwise.
	pub fn is_zero(&self) -> bool {
		for c in &self.0 {
			if !matches!(c, b'+' | b'-' | b'0' | b'.') {
				return false;
			}
		}

		true
	}

	pub fn sign(&self) -> Sign {
		let mut sign_positive = true;
		for c in &self.0 {
			match c {
				b'+' | b'0' | b'.' => (),
				b'-' => sign_positive = false,
				_ => {
					if sign_positive {
						return Sign::Positive;
					} else {
						return Sign::Negative;
					}
				}
			}
		}

		Sign::Zero
	}

	#[inline(always)]
	pub fn integer_part(&self) -> &Integer {
		match self.split_once('.') {
			Some((integer_part, _)) => unsafe { Integer::new_unchecked(integer_part) },
			None => unsafe { Integer::new_unchecked(self) },
		}
	}

	#[inline(always)]
	pub fn fractional_part(&self) -> Option<&FractionalPart> {
		self.split_once('.')
			.map(|(_, fractional_part)| unsafe { FractionalPart::new_unchecked(fractional_part) })
	}

	#[inline(always)]
	pub fn trimmed_fractional_part(&self) -> Option<&FractionalPart> {
		self.split_once('.').and_then(|(_, fractional_part)| {
			let f = unsafe { FractionalPart::new_unchecked(fractional_part) }.trimmed();
			if f.is_empty() {
				None
			} else {
				Some(f)
			}
		})
	}

	#[inline(always)]
	pub fn parts(&self) -> (&Integer, Option<&FractionalPart>) {
		match self.split_once('.') {
			Some((i, f)) => unsafe {
				(
					Integer::new_unchecked(i),
					Some(FractionalPart::new_unchecked(f)),
				)
			},
			None => unsafe { (Integer::new_unchecked(self), None) },
		}
	}

	pub fn value(&self) -> crate::Decimal {
		self.to_owned().into()
	}
}

impl PartialEq for Decimal {
	fn eq(&self, other: &Self) -> bool {
		self.integer_part() == other.integer_part()
			&& self.fractional_part() == other.fractional_part()
	}
}

impl Eq for Decimal {}

impl Hash for Decimal {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.integer_part().hash(h);
		match self.fractional_part() {
			Some(f) => f.hash(h),
			None => FractionalPart::empty().hash(h),
		}
	}
}

impl Ord for Decimal {
	fn cmp(&self, other: &Self) -> Ordering {
		let sign = self.sign();
		match sign.cmp(&other.sign()) {
			Ordering::Equal => {
				let (integer_part, fractional_part) = self.parts();
				let (other_integer_part, other_fractional_part) = other.parts();
				match integer_part.cmp(other_integer_part) {
					Ordering::Equal => {
						let fractional_part = fractional_part.unwrap_or_else(FractionalPart::empty);
						let other_fractional_part =
							other_fractional_part.unwrap_or_else(FractionalPart::empty);
						if sign.is_negative() {
							fractional_part.cmp(other_fractional_part).reverse()
						} else {
							fractional_part.cmp(other_fractional_part)
						}
					}
					other => {
						if sign.is_negative() {
							other.reverse()
						} else {
							other
						}
					}
				}
			}
			other => other,
		}
	}
}

impl PartialOrd for Decimal {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

macro_rules! integer_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for DecimalBuf {
				fn from(i: $ty) -> Self {
					unsafe { DecimalBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a Decimal> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a Decimal) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<DecimalBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: DecimalBuf) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}
		)*
	};
}

integer_conversion! {
	u8,
	i8,
	u16,
	i16,
	u32,
	i32,
	u64,
	i64,
	usize,
	isize
}

const DTOA_CONFIG: pretty_dtoa::FmtFloatConfig =
	pretty_dtoa::FmtFloatConfig::default().force_no_e_notation();

impl From<f32> for DecimalBuf {
	fn from(i: f32) -> Self {
		unsafe { DecimalBuf::new_unchecked(pretty_dtoa::ftoa(i, DTOA_CONFIG)) }
	}
}

impl<'a> TryFrom<&'a Decimal> for f32 {
	type Error = <f32 as std::str::FromStr>::Err;

	fn try_from(i: &'a Decimal) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl TryFrom<DecimalBuf> for f32 {
	type Error = <f32 as std::str::FromStr>::Err;

	fn try_from(i: DecimalBuf) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl From<f64> for DecimalBuf {
	fn from(i: f64) -> Self {
		unsafe { DecimalBuf::new_unchecked(pretty_dtoa::dtoa(i, DTOA_CONFIG)) }
	}
}

impl<'a> TryFrom<&'a Decimal> for f64 {
	type Error = <f64 as std::str::FromStr>::Err;

	fn try_from(i: &'a Decimal) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl TryFrom<DecimalBuf> for f64 {
	type Error = <f64 as std::str::FromStr>::Err;

	fn try_from(i: DecimalBuf) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

pub struct FractionalPart([u8]);

impl FractionalPart {
	/// Creates a new fractional part from a byte slice.
	///
	/// # Safety
	///
	/// The input byte slice must be a valid fractional part lexical
	/// representation.
	#[inline(always)]
	pub unsafe fn new_unchecked<S: ?Sized + AsRef<[u8]>>(s: &S) -> &Self {
		std::mem::transmute(s.as_ref())
	}

	#[inline(always)]
	pub fn empty<'a>() -> &'a Self {
		unsafe { Self::new_unchecked(b"") }
	}

	#[inline(always)]
	pub fn as_str(&self) -> &str {
		unsafe { core::str::from_utf8_unchecked(&self.0) }
	}

	#[inline(always)]
	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}

	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	/// Returns the fractional part without the trailing zeros.
	///
	/// The returned fractional part may be empty.
	pub fn trimmed(&self) -> &FractionalPart {
		let mut end = 0;
		for (i, &c) in self.0.iter().enumerate() {
			if c != b'0' {
				end = i + 1
			}
		}

		unsafe { Self::new_unchecked(&self.0[0..end]) }
	}
}

impl PartialEq for FractionalPart {
	fn eq(&self, other: &Self) -> bool {
		self.trimmed().0 == other.trimmed().0
	}
}

impl Eq for FractionalPart {}

impl Hash for FractionalPart {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.trimmed().0.hash(h)
	}
}

impl Ord for FractionalPart {
	fn cmp(&self, other: &Self) -> Ordering {
		self.trimmed().0.cmp(&other.trimmed().0)
	}
}

impl PartialOrd for FractionalPart {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
impl AsRef<[u8]> for FractionalPart {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl AsRef<str> for FractionalPart {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

fn check_bytes(s: &[u8]) -> bool {
	check(s.iter().copied())
}

fn check<C: Iterator<Item = u8>>(mut chars: C) -> bool {
	enum State {
		Initial,
		NonEmptyInteger,
		Integer,
		NonEmptyDecimal,
		Decimal,
	}

	let mut state = State::Initial;

	loop {
		state = match state {
			State::Initial => match chars.next() {
				Some(b'+') => State::NonEmptyInteger,
				Some(b'-') => State::NonEmptyInteger,
				Some(b'.') => State::NonEmptyDecimal,
				Some(b'0'..=b'9') => State::Integer,
				_ => break false,
			},
			State::NonEmptyInteger => match chars.next() {
				Some(b'0'..=b'9') => State::Integer,
				Some(b'.') => State::Decimal,
				_ => break false,
			},
			State::Integer => match chars.next() {
				Some(b'0'..=b'9') => State::Integer,
				Some(b'.') => State::Decimal,
				Some(_) => break false,
				None => break true,
			},
			State::NonEmptyDecimal => match chars.next() {
				Some(b'0'..=b'9') => State::Decimal,
				_ => break false,
			},
			State::Decimal => match chars.next() {
				Some(b'0'..=b'9') => State::Decimal,
				Some(_) => break false,
				None => break true,
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_01() {
		Decimal::new("0").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_02() {
		Decimal::new("+").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_03() {
		Decimal::new("-").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_04() {
		Decimal::new("012+").unwrap();
	}

	#[test]
	fn parse_05() {
		Decimal::new("+42").unwrap();
	}

	#[test]
	fn parse_06() {
		Decimal::new("-42").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_07() {
		Decimal::new(".").unwrap();
	}

	#[test]
	fn parse_08() {
		Decimal::new(".0").unwrap();
	}

	#[test]
	fn parse_09() {
		Decimal::new("0.").unwrap();
	}

	#[test]
	fn parse_10() {
		Decimal::new("42.0").unwrap();
	}

	#[test]
	fn format_01() {
		assert_eq!(DecimalBuf::from(1.0e10f32).to_string(), "10000000000.0")
	}

	#[test]
	fn cmp_01() {
		assert!(Decimal::new("0.123").unwrap() < Decimal::new("1.123").unwrap())
	}

	#[test]
	fn cmp_02() {
		assert!(Decimal::new("0.123").unwrap() < Decimal::new("0.1234").unwrap())
	}

	#[test]
	fn cmp_03() {
		assert!(Decimal::new("0.123").unwrap() > Decimal::new("-0.123").unwrap())
	}

	#[test]
	fn cmp_04() {
		assert!(Decimal::new("-0.123").unwrap() > Decimal::new("-0.1234").unwrap())
	}
}
