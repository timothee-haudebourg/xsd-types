use crate::XsdError;
use crate::lexical::{XsdLexicalForm, lexical_form};

use super::{Overflow, Sign, XsdDecimalStr, XsdDecimalString, XsdIntegerStr, XsdIntegerString};
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use str_newtype::StrNewType;

mod positive_integer;

pub use positive_integer::*;

/// Non negative integer number.
///
/// `nonNegativeInteger` has no numbered grammar production of its own
/// in the XSD 1.1 Datatypes specification: it is defined by
/// restricting [`XsdIntegerStr`]'s lexical space with a `minInclusive` of 0.
/// See: <https://www.w3.org/TR/xmlschema11-2/#nonNegativeInteger>.
///
/// ```abnf
/// nonNegativeInteger = "-" 1*"0"
///                     / [ "+" ] 1*DIGIT
/// ```
#[derive(Validate, StrNewType)]
#[automaton(crate::lexical::grammar::NonNegativeInteger)]
#[newtype(owned(XsdNonNegativeIntegerString, derive(PartialEq, Eq)))]
pub struct XsdNonNegativeIntegerStr(str);

impl XsdLexicalForm for XsdNonNegativeIntegerStr {
	type Value = crate::XsdNonNegativeInteger;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdNonNegativeIntegerStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdNonNegativeIntegerStr::new(value)
			.map_err(|_| XsdError::lexical_form(InvalidXsdNonNegativeIntegerStr(value)))
	}
}

lexical_form! {
	ty: XsdNonNegativeIntegerStr,
	buffer: XsdNonNegativeIntegerString,
	value: crate::XsdNonNegativeInteger,
	error: InvalidXsdNonNegativeIntegerStr,
	parent_forms: {
		as_integer: XsdIntegerStr, XsdIntegerString,
		as_decimal: XsdDecimalStr, XsdDecimalString
	}
}

impl XsdNonNegativeIntegerStr {
	/// Returns `true` if `self` is positive
	/// and `false` is the number is zero.
	pub fn is_positive(&self) -> bool {
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
				_ => return Sign::Positive,
			}
		}

		Sign::Zero
	}

	/// Returns the canonical form of `self` (without leading zeros).
	///
	/// The only valid lexical representation involving a leading `-` sign is
	/// a string of zeros (e.g. `-0`), since the represented value can never
	/// be negative. Such forms are canonicalized to `0`, just like their
	/// unsigned or `+`-prefixed counterparts.
	pub fn canonical(&self) -> &Self {
		let mut last_zero = 0;
		for (i, c) in self.0.as_bytes().iter().enumerate() {
			match c {
				b'+' | b'-' => (),
				b'0' => last_zero = i,
				_ => return unsafe { Self::new_unchecked(&self.0[i..]) },
			}
		}

		unsafe { Self::new_unchecked(&self.0[last_zero..]) }
	}

	#[inline(always)]
	fn as_canonical_str(&self) -> &str {
		self.canonical().as_str()
	}

	#[inline(always)]
	pub fn value(&self) -> crate::XsdNonNegativeInteger {
		use num_bigint::BigInt;
		unsafe {
			crate::XsdNonNegativeInteger::new_unchecked(BigInt::from_str(self.as_str()).unwrap())
		}
	}
}

impl PartialEq for XsdNonNegativeIntegerStr {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for XsdNonNegativeIntegerStr {}

impl Hash for XsdNonNegativeIntegerStr {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for XsdNonNegativeIntegerStr {
	fn cmp(&self, other: &Self) -> Ordering {
		let sign = self.sign();
		let other_sign = other.sign();
		match sign.cmp(&other_sign) {
			Ordering::Equal => {
				let a = &self.canonical().0;
				let b = &other.canonical().0;

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

impl PartialOrd for XsdNonNegativeIntegerStr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl XsdNonNegativeIntegerString {
	pub fn zero() -> Self {
		unsafe { Self::new_unchecked("0".to_string()) }
	}

	pub fn one() -> Self {
		unsafe { Self::new_unchecked("1".to_string()) }
	}
}

impl Default for XsdNonNegativeIntegerString {
	fn default() -> Self {
		Self::zero()
	}
}

impl PartialOrd for XsdNonNegativeIntegerString {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for XsdNonNegativeIntegerString {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for XsdNonNegativeIntegerString {
				fn from(i: $ty) -> Self {
					unsafe { XsdNonNegativeIntegerString::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a XsdNonNegativeIntegerStr> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a XsdNonNegativeIntegerStr) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<XsdNonNegativeIntegerString> for $ty {
				type Error = Overflow;

				fn try_from(i: XsdNonNegativeIntegerString) -> Result<Self, Overflow> {
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
	fn parse_negative_zero() {
		// `-0` is a valid (if unusual) lexical representation of `0`, which
		// is itself non-negative.
		XsdNonNegativeIntegerStr::new("-0").unwrap();
		XsdNonNegativeIntegerStr::new("-000").unwrap();
	}

	#[test]
	fn parse_negative_nonzero_rejected() {
		XsdNonNegativeIntegerStr::new("-1").unwrap_err();
	}

	#[test]
	fn negative_zero_is_zero() {
		let n = XsdNonNegativeIntegerStr::new("-0").unwrap();
		assert!(n.is_zero());
		assert!(!n.is_positive());
		assert_eq!(n.sign(), Sign::Zero);
	}

	#[test]
	fn negative_zero_canonical_form() {
		assert_eq!(
			XsdNonNegativeIntegerStr::new("-00")
				.unwrap()
				.canonical()
				.as_str(),
			"0"
		);
	}

	#[test]
	fn negative_zero_equals_zero() {
		assert_eq!(
			XsdNonNegativeIntegerStr::new("-0").unwrap(),
			XsdNonNegativeIntegerStr::new("0").unwrap()
		);
	}
}
