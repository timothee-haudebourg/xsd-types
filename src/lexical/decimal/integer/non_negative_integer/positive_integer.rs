use crate::XsdError;
use crate::lexical::{
	XsdDecimalStr, XsdDecimalString, XsdIntegerStr, XsdIntegerString, XsdLexicalForm,
	XsdNonNegativeIntegerStr, lexical_form,
};

use super::{XsdNonNegativeIntegerString, XsdOverflow};
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use str_newtype::StrNewType;

/// Positive integer number.
///
/// `positiveInteger` has no numbered grammar production of its own in
/// the XSD 1.1 Datatypes specification: it is defined by restricting
/// [`XsdNonNegativeIntegerStr`]'s lexical space with a `minInclusive` of 1.
/// See: <https://www.w3.org/TR/xmlschema11-2/#positiveInteger>.
///
/// ```abnf
/// positiveInteger = [ "+" ] *"0" NZDIGIT *DIGIT
/// ```
#[derive(Validate, StrNewType)]
#[automaton(crate::lexical::grammar::PositiveInteger)]
#[newtype(owned(XsdPositiveIntegerString, derive(PartialEq, Eq)))]
pub struct XsdPositiveIntegerStr(str);

impl XsdLexicalForm for XsdPositiveIntegerStr {
	type Value = crate::XsdPositiveInteger;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdPositiveIntegerStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdPositiveIntegerStr::new(value)
			.map_err(|_| XsdError::lexical_form(InvalidXsdPositiveIntegerStr(value)))
	}
}

lexical_form! {
	ty: XsdPositiveIntegerStr,
	buffer: XsdPositiveIntegerString,
	value: crate::XsdPositiveInteger,
	error: InvalidXsdPositiveIntegerStr,
	parent_forms: {
		as_non_negative_integer: XsdNonNegativeIntegerStr, XsdNonNegativeIntegerString,
		as_integer: XsdIntegerStr, XsdIntegerString,
		as_decimal: XsdDecimalStr, XsdDecimalString
	}
}

impl XsdPositiveIntegerStr {
	/// Returns the canonical form of `self` (without leading zeros).
	pub fn canonical(&self) -> &Self {
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

	fn as_canonical_str(&self) -> &str {
		self.canonical().as_str()
	}

	#[inline(always)]
	pub fn value(&self) -> crate::XsdPositiveInteger {
		use num_bigint::BigInt;
		unsafe {
			crate::XsdPositiveInteger::new_unchecked(BigInt::from_str(self.as_str()).unwrap())
		}
	}
}

impl PartialEq for XsdPositiveIntegerStr {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for XsdPositiveIntegerStr {}

impl Hash for XsdPositiveIntegerStr {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for XsdPositiveIntegerStr {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_canonical_str().cmp(other.as_canonical_str())
	}
}

impl PartialOrd for XsdPositiveIntegerStr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl XsdPositiveIntegerString {
	pub fn one() -> Self {
		unsafe { Self::new_unchecked("1".to_string()) }
	}
}

impl PartialOrd for XsdPositiveIntegerString {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for XsdPositiveIntegerString {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for XsdPositiveIntegerString {
				fn from(i: $ty) -> Self {
					unsafe { XsdPositiveIntegerString::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a XsdPositiveIntegerStr> for $ty {
				type Error = XsdOverflow;

				fn try_from(i: &'a XsdPositiveIntegerStr) -> Result<Self, XsdOverflow> {
					i.as_str().parse().map_err(|_| XsdOverflow)
				}
			}

			impl TryFrom<XsdPositiveIntegerString> for $ty {
				type Error = XsdOverflow;

				fn try_from(i: XsdPositiveIntegerString) -> Result<Self, XsdOverflow> {
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
