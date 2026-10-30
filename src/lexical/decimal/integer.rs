use crate::XsdError;
use crate::lexical::{XsdLexicalForm, lexical_form};

use super::{Overflow, Sign, XsdDecimalStr, XsdDecimalString};
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use str_newtype::StrNewType;

mod non_negative_integer;
mod non_positive_integer;

pub use non_negative_integer::*;
pub use non_positive_integer::*;

/// XsdIntegerStr number.
///
/// `integer` has no numbered grammar production of its own in the XSD
/// 1.1 Datatypes specification: it is defined by restricting
/// `decimal`'s lexical space (see [`XsdDecimalStr`]) to `noDecimalPtNumeral`
/// forms, i.e. disallowing a decimal point.
/// See: <https://www.w3.org/TR/xmlschema11-2/#integer>.
///
/// ```abnf
/// integer = noDecimalPtNumeral
/// ```
#[derive(Validate, StrNewType)]
#[automaton(crate::lexical::grammar::Integer)]
#[newtype(owned(XsdIntegerString, derive(PartialEq, Eq)))]
pub struct XsdIntegerStr(str);

impl XsdLexicalForm for XsdIntegerStr {
	type Value = crate::XsdInteger;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdIntegerStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdIntegerStr::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdIntegerStr(value)))
	}
}

lexical_form! {
	ty: XsdIntegerStr,
	buffer: XsdIntegerString,
	value: crate::XsdInteger,
	error: InvalidXsdIntegerStr,
	parent_forms: {
		as_decimal: XsdDecimalStr, XsdDecimalString
	}
}

impl XsdIntegerStr {
	/// Returns `true` if `self` is positive
	/// and `false` is the number is zero or negative.
	pub fn is_positive(&self) -> bool {
		let mut sign_positive = true;
		for c in self.0.as_bytes() {
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
		self.sign().is_negative()
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

	/// Returns `true` if `self` is positive or zero
	/// and `false` is negative.
	pub fn is_non_negative(&self) -> bool {
		self.0.as_bytes()[0] != b'-'
	}

	pub fn sign(&self) -> Sign {
		let mut sign_positive = true;
		for c in self.0.as_bytes() {
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

	/// Returns the canonical form of `self` (without leading zeros).
	pub fn canonical(&self) -> &Self {
		if self.is_zero() {
			unsafe { Self::new_unchecked(&self.0[self.0.len() - 1..]) }
		} else {
			let mut last_zero = 0;
			for (i, c) in self.0.as_bytes().iter().enumerate() {
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
	pub fn value(&self) -> crate::XsdInteger {
		crate::XsdInteger::from_str(self.as_str()).unwrap()
	}
}

impl PartialEq for XsdIntegerStr {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for XsdIntegerStr {}

impl Hash for XsdIntegerStr {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for XsdIntegerStr {
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

impl PartialOrd for XsdIntegerStr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl XsdIntegerString {
	pub fn zero() -> Self {
		unsafe { Self::new_unchecked("0".to_string()) }
	}

	pub fn one() -> Self {
		unsafe { Self::new_unchecked("1".to_string()) }
	}
}

impl Default for XsdIntegerString {
	fn default() -> Self {
		Self::zero()
	}
}

impl PartialOrd for XsdIntegerString {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for XsdIntegerString {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_xsd_integer_str().cmp(other.as_xsd_integer_str())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for XsdIntegerString {
				fn from(i: $ty) -> Self {
					unsafe { XsdIntegerString::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a XsdIntegerStr> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a XsdIntegerStr) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<XsdIntegerString> for $ty {
				type Error = Overflow;

				fn try_from(i: XsdIntegerString) -> Result<Self, Overflow> {
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
	fn sign_predicates() {
		for (input, negative, positive, zero) in [
			("42", false, true, false),
			("+42", false, true, false),
			("-42", true, false, false),
			("0", false, false, true),
			("-0", false, false, true),
		] {
			let i = XsdIntegerStr::new(input).unwrap();
			assert_eq!(i.is_negative(), negative, "is_negative({input})");
			assert_eq!(i.is_positive(), positive, "is_positive({input})");
			assert_eq!(i.is_zero(), zero, "is_zero({input})");
		}
	}

	#[test]
	fn parse_01() {
		XsdIntegerStr::new("0").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_02() {
		XsdIntegerStr::new("+").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_03() {
		XsdIntegerStr::new("-").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_04() {
		XsdIntegerStr::new("012+").unwrap();
	}

	#[test]
	fn parse_05() {
		XsdIntegerStr::new("+42").unwrap();
	}

	#[test]
	fn parse_06() {
		XsdIntegerStr::new("-42").unwrap();
	}

	#[test]
	fn abs_01() {
		assert_eq!(XsdIntegerStr::new("01").unwrap().abs().as_str(), "1")
	}

	#[test]
	fn abs_02() {
		assert_eq!(XsdIntegerStr::new("00").unwrap().abs().as_str(), "0")
	}

	#[test]
	fn abs_03() {
		assert_eq!(XsdIntegerStr::new("+00000").unwrap().abs().as_str(), "0")
	}

	#[test]
	fn abs_04() {
		assert_eq!(XsdIntegerStr::new("-00000").unwrap().abs().as_str(), "0")
	}

	#[test]
	fn abs_05() {
		assert_eq!(XsdIntegerStr::new("-00100").unwrap().abs().as_str(), "100")
	}

	#[test]
	fn eq_01() {
		assert_eq!(
			XsdIntegerStr::new("+001").unwrap(),
			XsdIntegerStr::new("1").unwrap()
		)
	}

	#[test]
	fn eq_02() {
		assert_ne!(
			XsdIntegerStr::new("-001").unwrap(),
			XsdIntegerStr::new("1").unwrap()
		)
	}

	#[test]
	fn eq_03() {
		assert_eq!(
			XsdIntegerStr::new("-000").unwrap(),
			XsdIntegerStr::new("+0").unwrap()
		)
	}

	#[test]
	fn cmp_01() {
		assert!(XsdIntegerStr::new("123").unwrap() < XsdIntegerStr::new("456").unwrap())
	}

	#[test]
	fn cmp_02() {
		assert!(XsdIntegerStr::new("1230").unwrap() > XsdIntegerStr::new("456").unwrap())
	}

	#[test]
	fn cmp_03() {
		assert!(XsdIntegerStr::new("-1230").unwrap() < XsdIntegerStr::new("456").unwrap())
	}

	#[test]
	fn cmp_04() {
		assert!(XsdIntegerStr::new("-1230").unwrap() < XsdIntegerStr::new("-456").unwrap())
	}

	#[test]
	fn cmp_05() {
		assert!(XsdIntegerStr::new("-123").unwrap() > XsdIntegerStr::new("-456").unwrap())
	}

	#[test]
	fn cmp_06() {
		assert_eq!(
			XsdIntegerStr::new("+123456")
				.unwrap()
				.cmp(XsdIntegerStr::new("0000123456").unwrap()),
			Ordering::Equal
		)
	}
}
