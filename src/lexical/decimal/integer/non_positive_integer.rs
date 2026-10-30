use crate::XsdError;
use crate::lexical::{XsdLexicalForm, lexical_form};

use super::{
	Overflow, Sign, XsdDecimalStr, XsdDecimalString, XsdIntegerStr, XsdIntegerString,
	XsdNonNegativeIntegerStr,
};
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
/// restricting [`XsdIntegerStr`]'s lexical space with a `maxInclusive` of 0.
/// See: <https://www.w3.org/TR/xmlschema11-2/#nonPositiveInteger>.
///
/// ```abnf
/// nonPositiveInteger = "+" 1*"0"
///                     / 1*"0"
///                     / "-" 1*DIGIT
/// ```
#[derive(Validate, StrNewType)]
#[automaton(crate::lexical::grammar::NonPositiveInteger)]
#[newtype(owned(XsdNonPositiveIntegerString, derive(PartialEq, Eq)))]
pub struct XsdNonPositiveIntegerStr(str);

impl XsdLexicalForm for XsdNonPositiveIntegerStr {
	type Value = crate::XsdNonPositiveInteger;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdNonPositiveIntegerStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdNonPositiveIntegerStr::new(value)
			.map_err(|_| XsdError::lexical_form(InvalidXsdNonPositiveIntegerStr(value)))
	}
}

lexical_form! {
	ty: XsdNonPositiveIntegerStr,
	buffer: XsdNonPositiveIntegerString,
	value: crate::XsdNonPositiveInteger,
	error: InvalidXsdNonPositiveIntegerStr,
	parent_forms: {
		as_integer: XsdIntegerStr, XsdIntegerString,
		as_decimal: XsdDecimalStr, XsdDecimalString
	}
}

impl XsdNonPositiveIntegerStr {
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
	pub fn abs(&self) -> &XsdNonNegativeIntegerStr {
		let mut last_zero = 0;
		for (i, c) in self.0.as_bytes().iter().enumerate() {
			match c {
				b'+' | b'-' => (),
				b'0' => last_zero = i,
				_ => return unsafe { XsdNonNegativeIntegerStr::new_unchecked(&self.0[i..]) },
			}
		}

		unsafe { XsdNonNegativeIntegerStr::new_unchecked(&self.0[last_zero..]) }
	}

	#[inline(always)]
	pub fn value(&self) -> crate::XsdNonPositiveInteger {
		use num_bigint::BigInt;
		unsafe {
			crate::XsdNonPositiveInteger::new_unchecked(BigInt::from_str(self.as_str()).unwrap())
		}
	}
}

impl PartialEq for XsdNonPositiveIntegerStr {
	fn eq(&self, other: &Self) -> bool {
		self.sign() == other.sign() && self.abs() == other.abs()
	}
}

impl Eq for XsdNonPositiveIntegerStr {}

impl Hash for XsdNonPositiveIntegerStr {
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

impl Ord for XsdNonPositiveIntegerStr {
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

impl PartialOrd for XsdNonPositiveIntegerStr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl XsdNonPositiveIntegerString {
	pub fn zero() -> Self {
		unsafe { Self::new_unchecked("0".to_string()) }
	}

	pub fn minus_one() -> Self {
		unsafe { Self::new_unchecked("-1".to_string()) }
	}
}

impl Default for XsdNonPositiveIntegerString {
	fn default() -> Self {
		Self::zero()
	}
}

impl PartialOrd for XsdNonPositiveIntegerString {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for XsdNonPositiveIntegerString {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for XsdNonPositiveIntegerString {
				fn from(i: $ty) -> Self {
					unsafe { XsdNonPositiveIntegerString::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a XsdNonPositiveIntegerStr> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a XsdNonPositiveIntegerStr) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<XsdNonPositiveIntegerString> for $ty {
				type Error = Overflow;

				fn try_from(i: XsdNonPositiveIntegerString) -> Result<Self, Overflow> {
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
		XsdNonPositiveIntegerStr::new("0").unwrap();
		XsdNonPositiveIntegerStr::new("+0").unwrap();
		XsdNonPositiveIntegerStr::new("+000").unwrap();
	}

	#[test]
	fn parse_positive_nonzero_rejected() {
		XsdNonPositiveIntegerStr::new("+1").unwrap_err();
	}

	#[test]
	fn positive_zero_is_zero() {
		let n = XsdNonPositiveIntegerStr::new("+0").unwrap();
		assert!(n.is_zero());
		assert!(!n.is_negative());
		assert_eq!(n.sign(), Sign::Zero);
	}

	#[test]
	fn positive_zero_equals_zero() {
		assert_eq!(
			XsdNonPositiveIntegerStr::new("+0").unwrap(),
			XsdNonPositiveIntegerStr::new("0").unwrap()
		);
	}
}
