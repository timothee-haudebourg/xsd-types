use crate::XsdError;
use crate::lexical::{
	XsdDecimalStr, XsdDecimalString, XsdIntegerStr, XsdIntegerString, XsdLexicalForm,
	XsdNonNegativeIntegerStr, lexical_form,
};

use super::{XsdNonPositiveIntegerStr, XsdNonPositiveIntegerString, XsdOverflow};
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use str_newtype::StrNewType;

/// Negative integer number.
///
/// `negativeInteger` has no numbered grammar production of its own in
/// the XSD 1.1 Datatypes specification: it is defined by restricting
/// [`XsdNonPositiveIntegerStr`]'s lexical space with a `maxInclusive` of -1.
/// See: <https://www.w3.org/TR/xmlschema11-2/#negativeInteger>.
///
/// ```abnf
/// negativeInteger = "-" *"0" NZDIGIT *DIGIT
/// ```
#[derive(Validate, StrNewType)]
#[automaton(crate::lexical::grammar::NegativeInteger)]
#[newtype(owned(XsdNegativeIntegerString, derive(PartialEq, Eq)))]
pub struct XsdNegativeIntegerStr(str);

impl XsdLexicalForm for XsdNegativeIntegerStr {
	type Value = crate::XsdNegativeInteger;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdNegativeIntegerStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdNegativeIntegerStr::new(value)
			.map_err(|_| XsdError::lexical_form(InvalidXsdNegativeIntegerStr(value)))
	}
}

lexical_form! {
	ty: XsdNegativeIntegerStr,
	buffer: XsdNegativeIntegerString,
	value: crate::XsdNegativeInteger,
	error: InvalidXsdNegativeIntegerStr,
	parent_forms: {
		as_non_positive_integer: XsdNonPositiveIntegerStr, XsdNonPositiveIntegerString,
		as_integer: XsdIntegerStr, XsdIntegerString,
		as_decimal: XsdDecimalStr, XsdDecimalString
	}
}

impl XsdNegativeIntegerStr {
	/// Returns the canonical form of the absolute value of `self` (without leading zeros).
	pub fn abs(&self) -> &XsdNonNegativeIntegerStr {
		let mut last_zero = 0;
		for (i, c) in self.0.as_bytes().iter().enumerate() {
			match c {
				b'-' => (),
				b'0' => last_zero = i,
				_ => return unsafe { XsdNonNegativeIntegerStr::new_unchecked(&self.0[i..]) },
			}
		}

		unsafe { XsdNonNegativeIntegerStr::new_unchecked(&self.0[last_zero..]) }
	}

	fn as_canonical_str(&self) -> &str {
		self.abs().as_str()
	}

	#[inline(always)]
	pub fn value(&self) -> crate::XsdNegativeInteger {
		use num_bigint::BigInt;
		unsafe {
			crate::XsdNegativeInteger::new_unchecked(BigInt::from_str(self.as_str()).unwrap())
		}
	}
}

impl PartialEq for XsdNegativeIntegerStr {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for XsdNegativeIntegerStr {}

impl Hash for XsdNegativeIntegerStr {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for XsdNegativeIntegerStr {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_canonical_str()
			.cmp(other.as_canonical_str())
			.reverse()
	}
}

impl PartialOrd for XsdNegativeIntegerStr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl XsdNegativeIntegerString {
	pub fn minus_one() -> Self {
		unsafe { Self::new_unchecked("-1".to_string()) }
	}
}

impl PartialOrd for XsdNegativeIntegerString {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for XsdNegativeIntegerString {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for XsdNegativeIntegerString {
				fn from(i: $ty) -> Self {
					unsafe { XsdNegativeIntegerString::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a XsdNegativeIntegerStr> for $ty {
				type Error = XsdOverflow;

				fn try_from(i: &'a XsdNegativeIntegerStr) -> Result<Self, XsdOverflow> {
					i.as_str().parse().map_err(|_| XsdOverflow)
				}
			}

			impl TryFrom<XsdNegativeIntegerString> for $ty {
				type Error = XsdOverflow;

				fn try_from(i: XsdNegativeIntegerString) -> Result<Self, XsdOverflow> {
					i.as_str().parse().map_err(|_| XsdOverflow)
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
	fn parse_negative() {
		XsdNegativeIntegerStr::new("-1").unwrap();
		XsdNegativeIntegerStr::new("-042").unwrap();
	}

	#[test]
	fn parse_zero_rejected() {
		// A negative integer's value must be strictly negative: `-0` and `0`
		// are not valid `negativeInteger` lexical representations.
		XsdNegativeIntegerStr::new("-0").unwrap_err();
		XsdNegativeIntegerStr::new("0").unwrap_err();
	}

	#[test]
	fn as_non_positive_integer_preserves_value() {
		let n = XsdNegativeIntegerStr::new("-5").unwrap();
		assert_eq!(n.as_non_positive_integer().as_str(), "-5");
	}
}
