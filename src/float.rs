use super::{Decimal, DecimalBuf, Integer, IntegerBuf, Overflow};
use std::borrow::{Borrow, ToOwned};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct InvalidFloat;

pub const NAN: &Float = unsafe { Float::new_unchecked_from_slice(b"NaN") };
pub const POSITIVE_INFINITY: &Float = unsafe { Float::new_unchecked_from_slice(b"INF") };
pub const NEGATIVE_INFINITY: &Float = unsafe { Float::new_unchecked_from_slice(b"-INF") };

/// Float number.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#float>
#[derive(PartialEq, Eq, Hash)]
pub struct Float([u8]);

impl Float {
	/// Creates a new `Float` from a string.
	///
	/// If the input string is ot a [valid XSD float](https://www.w3.org/TR/xmlschema-2/#float),
	/// an [`InvalidFloat`] error is returned.
	#[inline(always)]
	pub fn new<S: ?Sized + AsRef<[u8]>>(s: &S) -> Result<&Self, InvalidFloat> {
		if check(s.as_ref()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err(InvalidFloat)
		}
	}

	/// Creates a new `Float` from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD float](https://www.w3.org/TR/xmlschema-2/#float).
	#[inline(always)]
	pub unsafe fn new_unchecked<S: ?Sized + AsRef<[u8]>>(s: &S) -> &Self {
		std::mem::transmute(s.as_ref())
	}

	/// Creates a new `Float` from a byte slice without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD float](https://www.w3.org/TR/xmlschema-2/#float).
	#[inline(always)]
	pub const unsafe fn new_unchecked_from_slice(s: &[u8]) -> &Self {
		std::mem::transmute(s)
	}

	#[inline(always)]
	pub fn as_str(&self) -> &str {
		unsafe { core::str::from_utf8_unchecked(&self.0) }
	}

	#[inline(always)]
	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}

	pub fn is_infinite(&self) -> bool {
		matches!(&self.0, b"INF" | b"-INF")
	}

	pub fn is_finite(&self) -> bool {
		!matches!(&self.0, b"INF" | b"-INF" | b"NaN")
	}

	pub fn is_nan(&self) -> bool {
		&self.0 == b"NaN"
	}

	fn exponent_separator_index(&self) -> Option<usize> {
		for (i, c) in self.0.iter().enumerate() {
			if matches!(c, b'e' | b'E') {
				return Some(i);
			}
		}

		None
	}

	pub fn mantissa(&self) -> Option<&Decimal> {
		if self.is_finite() {
			Some(match self.exponent_separator_index() {
				Some(e) => unsafe { Decimal::new_unchecked(&self[..e]) },
				None => unsafe { Decimal::new_unchecked(self) },
			})
		} else {
			None
		}
	}

	pub fn exponent(&self) -> Option<&Integer> {
		if self.is_finite() {
			self.exponent_separator_index()
				.map(|e| unsafe { Integer::new_unchecked(&self[(e + 1)..]) })
		} else {
			None
		}
	}
}

impl AsRef<[u8]> for Float {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl AsRef<str> for Float {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

macro_rules! integer_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for FloatBuf {
				fn from(i: $ty) -> Self {
					unsafe { FloatBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a Float> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a Float) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<FloatBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: FloatBuf) -> Result<Self, Overflow> {
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

const DTOA_CONFIG: pretty_dtoa::FmtFloatConfig = pretty_dtoa::FmtFloatConfig::default();

impl From<f32> for FloatBuf {
	fn from(i: f32) -> Self {
		if i.is_finite() {
			unsafe { FloatBuf::new_unchecked(pretty_dtoa::ftoa(i, DTOA_CONFIG)) }
		} else if i.is_nan() {
			FloatBuf::nan()
		} else if i.is_sign_positive() {
			FloatBuf::positive_infinity()
		} else {
			FloatBuf::negative_infinity()
		}
	}
}

impl<'a> TryFrom<&'a Float> for f32 {
	type Error = <f32 as std::str::FromStr>::Err;

	fn try_from(i: &'a Float) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl TryFrom<FloatBuf> for f32 {
	type Error = <f32 as std::str::FromStr>::Err;

	fn try_from(i: FloatBuf) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl From<f64> for FloatBuf {
	fn from(i: f64) -> Self {
		if i.is_finite() {
			unsafe { FloatBuf::new_unchecked(pretty_dtoa::dtoa(i, DTOA_CONFIG)) }
		} else if i.is_nan() {
			FloatBuf::nan()
		} else if i.is_sign_positive() {
			FloatBuf::positive_infinity()
		} else {
			FloatBuf::negative_infinity()
		}
	}
}

impl<'a> TryFrom<&'a Float> for f64 {
	type Error = <f64 as std::str::FromStr>::Err;

	fn try_from(i: &'a Float) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl TryFrom<FloatBuf> for f64 {
	type Error = <f64 as std::str::FromStr>::Err;

	fn try_from(i: FloatBuf) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl Deref for Float {
	type Target = str;

	#[inline(always)]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

impl ToOwned for Float {
	type Owned = FloatBuf;

	#[inline(always)]
	fn to_owned(&self) -> FloatBuf {
		unsafe { FloatBuf::new_unchecked(self.as_str().to_owned()) }
	}
}

impl fmt::Display for Float {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_str().fmt(f)
	}
}

impl fmt::Debug for Float {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_str().fmt(f)
	}
}

impl<'a> From<&'a Integer> for &'a Float {
	#[inline(always)]
	fn from(d: &'a Integer) -> Self {
		unsafe { Float::new_unchecked(d) }
	}
}

impl<'a> From<&'a Decimal> for &'a Float {
	#[inline(always)]
	fn from(d: &'a Decimal) -> Self {
		unsafe { Float::new_unchecked(d) }
	}
}

impl<'a> From<&'a IntegerBuf> for &'a Float {
	#[inline(always)]
	fn from(d: &'a IntegerBuf) -> Self {
		d.as_ref()
	}
}

impl<'a> From<&'a DecimalBuf> for &'a Float {
	#[inline(always)]
	fn from(d: &'a DecimalBuf) -> Self {
		d.as_ref()
	}
}

impl<'a> From<&'a FloatBuf> for &'a Float {
	#[inline(always)]
	fn from(b: &'a FloatBuf) -> Self {
		b.as_ref()
	}
}

/// Owned float number.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#float>
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FloatBuf(Vec<u8>);

impl FloatBuf {
	/// Creates a new `FloatBuf` from a `String`.
	///
	/// If the input string is ot a [valid XSD float](https://www.w3.org/TR/xmlschema-2/#float),
	/// an [`InvalidFloat`] error is returned along with the input string.
	#[inline(always)]
	pub fn new<S: AsRef<[u8]> + Into<Vec<u8>>>(s: S) -> Result<Self, (InvalidFloat, S)> {
		if check(s.as_ref()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err((InvalidFloat, s))
		}
	}

	/// Creates a new `FloatBuf` from a `String` without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD float](https://www.w3.org/TR/xmlschema-2/#float).
	#[inline(always)]
	pub unsafe fn new_unchecked(s: impl Into<Vec<u8>>) -> Self {
		std::mem::transmute(s.into())
	}

	#[inline(always)]
	pub fn nan() -> Self {
		NAN.to_owned()
	}

	#[inline(always)]
	pub fn positive_infinity() -> Self {
		POSITIVE_INFINITY.to_owned()
	}

	#[inline(always)]
	pub fn negative_infinity() -> Self {
		NEGATIVE_INFINITY.to_owned()
	}

	#[inline(always)]
	pub fn into_string(mut self) -> String {
		let buf = self.0.as_mut_ptr();
		let len = self.0.len();
		let capacity = self.0.capacity();
		core::mem::forget(self);
		unsafe {
			String::from_raw_parts(buf, len, capacity)
		}
	}
}

impl FromStr for FloatBuf {
	type Err = InvalidFloat;

	fn from_str(s: &str) -> Result<Self, InvalidFloat> {
		Self::new(s.to_owned()).map_err(|(e, _)| e)
	}
}

impl Deref for FloatBuf {
	type Target = Float;

	#[inline(always)]
	fn deref(&self) -> &Float {
		unsafe { Float::new_unchecked(&self.0) }
	}
}

impl AsRef<Float> for FloatBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Float {
		unsafe { Float::new_unchecked(&self.0) }
	}
}

impl Borrow<Float> for FloatBuf {
	#[inline(always)]
	fn borrow(&self) -> &Float {
		unsafe { Float::new_unchecked(&self.0) }
	}
}

impl fmt::Display for FloatBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_str().fmt(f)
	}
}

impl fmt::Debug for FloatBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_str().fmt(f)
	}
}

fn check(s: &[u8]) -> bool {
	s == b"INF" || s == b"-INF" || s == b"NaN" || check_normal(s.iter().cloned())
}

fn check_normal<C: Iterator<Item = u8>>(mut chars: C) -> bool {
	enum State {
		Initial,
		NonEmptyInteger,
		Integer,
		NonEmptyDecimal,
		Decimal,
		ExponentSign,
		NonEmptyExponent,
		Exponent,
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
				Some(b'e' | b'E') => State::ExponentSign,
				Some(_) => break false,
				None => break true,
			},
			State::NonEmptyDecimal => match chars.next() {
				Some(b'0'..=b'9') => State::Decimal,
				_ => break false,
			},
			State::Decimal => match chars.next() {
				Some(b'0'..=b'9') => State::Decimal,
				Some(b'e' | b'E') => State::ExponentSign,
				Some(_) => break false,
				None => break true,
			},
			State::ExponentSign => match chars.next() {
				Some(b'+' | b'-') => State::NonEmptyExponent,
				Some(b'0'..=b'9') => State::Exponent,
				_ => break false,
			},
			State::NonEmptyExponent => match chars.next() {
				Some(b'0'..=b'9') => State::Exponent,
				_ => break false,
			},
			State::Exponent => match chars.next() {
				Some(b'0'..=b'9') => State::Exponent,
				Some(_) => break false,
				None => break true,
			},
		}
	}
}

impl PartialEq<Float> for FloatBuf {
	#[inline(always)]
	fn eq(&self, other: &Float) -> bool {
		self.as_ref() == other
	}
}

impl<'a> PartialEq<&'a Float> for FloatBuf {
	#[inline(always)]
	fn eq(&self, other: &&'a Float) -> bool {
		self.as_ref() == *other
	}
}

impl PartialEq<FloatBuf> for Float {
	#[inline(always)]
	fn eq(&self, other: &FloatBuf) -> bool {
		self == other.as_ref()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_01() {
		Float::new("0").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_02() {
		Float::new("+").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_03() {
		Float::new("-").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_04() {
		Float::new("012+").unwrap();
	}

	#[test]
	fn parse_05() {
		Float::new("+42").unwrap();
	}

	#[test]
	fn parse_06() {
		Float::new("-42").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_07() {
		Float::new(".").unwrap();
	}

	#[test]
	fn parse_08() {
		Float::new(".0").unwrap();
	}

	#[test]
	fn parse_09() {
		Float::new("0.").unwrap();
	}

	#[test]
	fn parse_10() {
		Float::new("42.0").unwrap();
	}

	#[test]
	fn parse_11() {
		Float::new("INF").unwrap();
	}

	#[test]
	fn parse_12() {
		Float::new("-INF").unwrap();
	}

	#[test]
	fn parse_13() {
		Float::new("NaN").unwrap();
	}

	#[test]
	fn parse_14() {
		Float::new(".0e1").unwrap();
	}

	#[test]
	fn parse_15() {
		Float::new("0.e1").unwrap();
	}

	#[test]
	fn parse_16() {
		Float::new("42E10").unwrap();
	}

	#[test]
	fn parse_17() {
		Float::new("-42E+10").unwrap();
	}

	#[test]
	fn parse_18() {
		Float::new("-42E-10").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_19() {
		Float::new("+42E-10e").unwrap();
	}

	#[test]
	fn parse_20() {
		Float::new("+42E-10").unwrap();
	}

	#[test]
	fn parse_21() {
		let d = Float::new("+01234E-56789").unwrap();
		assert_eq!(d.mantissa(), Some(Decimal::new("+01234").unwrap()));
		assert_eq!(d.exponent(), Some(Integer::new("-56789").unwrap()));
	}

	#[test]
	fn parse_22() {
		let a = FloatBuf::new("+01234E-56789".to_string()).unwrap();
		let b = Float::new("+01234E-56789").unwrap();
		assert_eq!(a, b)
	}

	#[test]
	fn format_01() {
		assert_eq!(FloatBuf::from(1.0e10f32).to_string(), "1.0e10")
	}
}
