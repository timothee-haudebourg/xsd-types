use crate::lexical::{lexical_form, Lexical};

use super::{Decimal, DecimalBuf, Integer, IntegerBuf, NonNegativeInteger, Overflow, Sign};
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use str_newtype::StrNewType;

mod negative_integer;

pub use negative_integer::*;

/// Non positive integer number.
///
/// `nonPositiveInteger` has no numbered grammar production of its own
/// in the XSD 1.1 Datatypes specification: it is defined by
/// restricting [`Integer`]'s lexical space with a `maxInclusive` of 0.
/// See: <https://www.w3.org/TR/xmlschema11-2/#nonPositiveInteger>.
///
/// ```abnf
/// nonPositiveInteger = "+" 1*"0"
///                     / 1*"0"
///                     / "-" 1*DIGIT
/// ```
#[derive(Validate, StrNewType)]
#[automaton(crate::lexical::grammar::NonPositiveInteger)]
#[newtype(owned(NonPositiveIntegerBuf, derive(PartialEq, Eq)))]
pub struct NonPositiveInteger(str);

impl Lexical for NonPositiveInteger {
	type Error = InvalidNonPositiveInteger<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidNonPositiveInteger(value.to_owned()))
	}
}

lexical_form! {
	ty: NonPositiveInteger,
	buffer: NonPositiveIntegerBuf,
	value: crate::NonPositiveInteger,
	error: InvalidNonPositiveInteger,
	parent_forms: {
		as_integer: Integer, IntegerBuf,
		as_decimal: Decimal, DecimalBuf
	}
}

impl NonPositiveInteger {
	/// Returns `true` if `self` is negative
	/// and `false` is the number is zero.
	pub fn is_negative(&self) -> bool {
		for c in self.0.as_bytes() {
			match c {
				b'+' | b'-' | b'0' => (),
				_ => return true,
			}
		}

		false
	}

	/// Returns `true` if `self` is zero
	/// and `false` otherwise.
	pub fn is_zero(&self) -> bool {
		for c in self.0.as_bytes() {
			if !matches!(c, b'+' | b'-' | b'0') {
				return false;
			}
		}

		true
	}

	pub fn sign(&self) -> Sign {
		for c in self.0.as_bytes() {
			match c {
				b'+' | b'-' | b'0' => (),
				_ => return Sign::Negative,
			}
		}

		Sign::Zero
	}

	/// Returns the absolute value of `self`.
	///
	/// The returned integer is in canonical form (without leading zeros).
	pub fn abs(&self) -> &NonNegativeInteger {
		let mut last_zero = 0;
		for (i, c) in self.0.as_bytes().iter().enumerate() {
			match c {
				b'+' | b'-' => (),
				b'0' => last_zero = i,
				_ => return unsafe { NonNegativeInteger::new_unchecked(&self.0[i..]) },
			}
		}

		unsafe { NonNegativeInteger::new_unchecked(&self.0[last_zero..]) }
	}

	#[inline(always)]
	pub fn value(&self) -> crate::NonPositiveInteger {
		use num_bigint::BigInt;
		unsafe {
			crate::NonPositiveInteger::new_unchecked(BigInt::from_str(self.as_str()).unwrap())
		}
	}
}

impl PartialEq for NonPositiveInteger {
	fn eq(&self, other: &Self) -> bool {
		self.sign() == other.sign() && self.abs() == other.abs()
	}
}

impl Eq for NonPositiveInteger {}

impl Hash for NonPositiveInteger {
	fn hash<H: Hasher>(&self, h: &mut H) {
		match self.sign() {
			Sign::Zero => 0.hash(h),
			sign => {
				sign.hash(h);
				self.abs().hash(h)
			}
		}
	}
}

impl Ord for NonPositiveInteger {
	fn cmp(&self, other: &Self) -> Ordering {
		let sign = self.sign();
		let other_sign = other.sign();
		match sign.cmp(&other_sign) {
			Ordering::Equal => {
				let a = &self.abs();
				let b = &other.abs();

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

impl PartialOrd for NonPositiveInteger {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl NonPositiveIntegerBuf {
	pub fn zero() -> Self {
		unsafe { Self::new_unchecked("0".to_string()) }
	}

	pub fn minus_one() -> Self {
		unsafe { Self::new_unchecked("-1".to_string()) }
	}
}

impl Default for NonPositiveIntegerBuf {
	fn default() -> Self {
		Self::zero()
	}
}

impl PartialOrd for NonPositiveIntegerBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for NonPositiveIntegerBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for NonPositiveIntegerBuf {
				fn from(i: $ty) -> Self {
					unsafe { NonPositiveIntegerBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a NonPositiveInteger> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a NonPositiveInteger) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<NonPositiveIntegerBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: NonPositiveIntegerBuf) -> Result<Self, Overflow> {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_zero_variants() {
		// `0` and `+0` are valid (if unusual, for the latter) lexical
		// representations of `0`, which is itself non-positive.
		NonPositiveInteger::new("0").unwrap();
		NonPositiveInteger::new("+0").unwrap();
		NonPositiveInteger::new("+000").unwrap();
	}

	#[test]
	fn parse_positive_nonzero_rejected() {
		NonPositiveInteger::new("+1").unwrap_err();
	}

	#[test]
	fn positive_zero_is_zero() {
		let n = NonPositiveInteger::new("+0").unwrap();
		assert!(n.is_zero());
		assert!(!n.is_negative());
		assert_eq!(n.sign(), Sign::Zero);
	}

	#[test]
	fn positive_zero_equals_zero() {
		assert_eq!(
			NonPositiveInteger::new("+0").unwrap(),
			NonPositiveInteger::new("0").unwrap()
		);
	}
}
