use crate::lexical::lexical_form;

use super::{Decimal, DecimalBuf, Overflow, Sign};
use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

mod non_negative_integer;
mod non_positive_integer;

pub use non_negative_integer::*;
pub use non_positive_integer::*;

lexical_form! {
	/// Integer number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#integer>
	ty: Integer,

	/// Owned integer number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#integer>
	buffer: IntegerBuf,

	/// Creates a new integer from a string.
	///
	/// If the input string is ot a [valid XSD integer](https://www.w3.org/TR/xmlschema-2/#integer),
	/// an [`InvalidInteger`] error is returned.
	new,

	/// Creates a new integer from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD integer](https://www.w3.org/TR/xmlschema-2/#integer).
	new_unchecked,

	value: crate::Integer,
	error: InvalidInteger,
	as_ref: as_integer,
	parent_forms: {
		as_decimal: Decimal, DecimalBuf
	}
}

impl Integer {
	/// Returns `true` if `self` is positive
	/// and `false` is the number is zero or negative.
	pub fn is_positive(&self) -> bool {
		let mut sign_positive = true;
		for c in &self.0 {
			match c {
				b'+' | b'0' => (),
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
				b'-' | b'0' => (),
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
			if !matches!(c, b'+' | b'-' | b'0') {
				return false;
			}
		}

		true
	}

	/// Returns `true` if `self` is positive or zero
	/// and `false` is negative.
	pub fn is_non_negative(&self) -> bool {
		self.0[0] != b'-'
	}

	pub fn sign(&self) -> Sign {
		let mut sign_positive = true;
		for c in &self.0 {
			match c {
				b'+' | b'0' => (),
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

	/// Returns the absolute value of `self`.
	///
	/// The returned integer is in canonical form (without leading zeros).
	pub fn abs(&self) -> &NonNegativeInteger {
		let mut last_zero = 0;
		for (i, c) in self.0.iter().enumerate() {
			match c {
				b'+' | b'-' => (),
				b'0' => last_zero = i,
				_ => return unsafe { NonNegativeInteger::new_unchecked(&self.0[i..]) },
			}
		}

		unsafe { NonNegativeInteger::new_unchecked(&self.0[last_zero..]) }
	}

	/// Returns the canonical form of `self` (without leading zeros).
	pub fn canonical(&self) -> &Self {
		if self.is_zero() {
			unsafe { Self::new_unchecked(&self.0[self.0.len() - 1..]) }
		} else {
			let mut last_zero = 0;
			for (i, c) in self.0.iter().enumerate() {
				match c {
					b'+' => (),
					b'0' => last_zero = i,
					_ => return unsafe { Self::new_unchecked(&self.0[i..]) },
				}
			}

			unsafe { Self::new_unchecked(&self.0[last_zero..]) }
		}
	}

	#[inline(always)]
	fn as_canonical_str(&self) -> &str {
		self.canonical().as_str()
	}

	#[inline(always)]
	pub fn value(&self) -> crate::Integer {
		crate::Integer::from_str(self.as_str()).unwrap()
	}
}

impl PartialEq for Integer {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for Integer {}

impl Hash for Integer {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for Integer {
	fn cmp(&self, other: &Self) -> Ordering {
		let sign = self.sign();
		let other_sign = other.sign();
		match sign.cmp(&other_sign) {
			Ordering::Equal => {
				let a = self.abs().as_bytes();
				let b = other.abs().as_bytes();

				match a.len().cmp(&b.len()) {
					Ordering::Equal => {
						if sign.is_negative() {
							a.cmp(b).reverse()
						} else {
							a.cmp(b)
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

impl PartialOrd for Integer {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl IntegerBuf {
	pub fn zero() -> Self {
		unsafe { Self::new_unchecked("0".to_string()) }
	}

	pub fn one() -> Self {
		unsafe { Self::new_unchecked("1".to_string()) }
	}
}

impl Default for IntegerBuf {
	fn default() -> Self {
		Self::zero()
	}
}

impl PartialOrd for IntegerBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for IntegerBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for IntegerBuf {
				fn from(i: $ty) -> Self {
					unsafe { IntegerBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a Integer> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a Integer) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<IntegerBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: IntegerBuf) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}
		)*
	};
}

number_conversion! {
	u8,
	i8,
	u16,
	i16,
	u32,
	i32,
	u64,
	i64,
	u128,
	i128,
	usize,
	isize
}

fn check_bytes(s: &[u8]) -> bool {
	check(s.iter().copied())
}

fn check<C: Iterator<Item = u8>>(mut chars: C) -> bool {
	enum State {
		Initial,
		NonEmptyInteger,
		Integer,
	}

	let mut state = State::Initial;

	loop {
		state = match state {
			State::Initial => match chars.next() {
				Some(b'+') => State::NonEmptyInteger,
				Some(b'-') => State::NonEmptyInteger,
				Some(b'0'..=b'9') => State::Integer,
				_ => break false,
			},
			State::NonEmptyInteger => match chars.next() {
				Some(b'0'..=b'9') => State::Integer,
				_ => break false,
			},
			State::Integer => match chars.next() {
				Some(b'0'..=b'9') => State::Integer,
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
		Integer::new("0").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_02() {
		Integer::new("+").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_03() {
		Integer::new("-").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_04() {
		Integer::new("012+").unwrap();
	}

	#[test]
	fn parse_05() {
		Integer::new("+42").unwrap();
	}

	#[test]
	fn parse_06() {
		Integer::new("-42").unwrap();
	}

	#[test]
	fn abs_01() {
		assert_eq!(Integer::new("01").unwrap().abs().as_str(), "1")
	}

	#[test]
	fn abs_02() {
		assert_eq!(Integer::new("00").unwrap().abs().as_str(), "0")
	}

	#[test]
	fn abs_03() {
		assert_eq!(Integer::new("+00000").unwrap().abs().as_str(), "0")
	}

	#[test]
	fn abs_04() {
		assert_eq!(Integer::new("-00000").unwrap().abs().as_str(), "0")
	}

	#[test]
	fn abs_05() {
		assert_eq!(Integer::new("-00100").unwrap().abs().as_str(), "100")
	}

	#[test]
	fn eq_01() {
		assert_eq!(Integer::new("+001").unwrap(), Integer::new("1").unwrap())
	}

	#[test]
	fn eq_02() {
		assert_ne!(Integer::new("-001").unwrap(), Integer::new("1").unwrap())
	}

	#[test]
	fn eq_03() {
		assert_eq!(Integer::new("-000").unwrap(), Integer::new("+0").unwrap())
	}

	#[test]
	fn cmp_01() {
		assert!(Integer::new("123").unwrap() < Integer::new("456").unwrap())
	}

	#[test]
	fn cmp_02() {
		assert!(Integer::new("1230").unwrap() > Integer::new("456").unwrap())
	}

	#[test]
	fn cmp_03() {
		assert!(Integer::new("-1230").unwrap() < Integer::new("456").unwrap())
	}

	#[test]
	fn cmp_04() {
		assert!(Integer::new("-1230").unwrap() < Integer::new("-456").unwrap())
	}

	#[test]
	fn cmp_05() {
		assert!(Integer::new("-123").unwrap() > Integer::new("-456").unwrap())
	}

	#[test]
	fn cmp_06() {
		assert_eq!(
			Integer::new("+123456")
				.unwrap()
				.cmp(Integer::new("0000123456").unwrap()),
			Ordering::Equal
		)
	}
}
