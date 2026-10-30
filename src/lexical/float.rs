use super::{
	Overflow, XsdDecimalStr, XsdIntegerStr, XsdNonNegativeIntegerStr, XsdNonPositiveIntegerStr,
	lexical_form,
};
use crate::XsdError;
use crate::lexical::XsdLexicalForm;
use static_automata::Validate;
use std::hash::Hash;
use str_newtype::StrNewType;

/// XsdFloatStr number.
///
/// This is the `floatRep` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-floatRep>.
/// It is identical to the `doubleRep` production used by [`XsdDoubleStr`](crate::lexical::XsdDoubleStr);
/// only the value spaces (precision) of `float` and `double` differ.
///
/// ```abnf
/// floatRep = noDecimalPtNumeral / decimalPtNumeral / scientificNotationNumeral / numericalSpecialRep
///
/// scientificNotationNumeral = [ ("+" / "-") ] (unsignedNoDecimalPtNumeral / unsignedDecimalPtNumeral) "e" noDecimalPtNumeral
///
/// numericalSpecialRep = %s"+INF" / %s"INF" / %s"-INF" / %s"NaN"
/// ```
#[derive(Validate, StrNewType)]
#[automaton(crate::lexical::grammar::Float)]
#[newtype(owned(XsdFloatString, derive(PartialEq, Eq)))]
pub struct XsdFloatStr(str);

impl XsdLexicalForm for XsdFloatStr {
	type Value = crate::XsdFloat;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdFloatStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdFloatStr::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdFloatStr(value)))
	}
}

lexical_form! {
	ty: XsdFloatStr,
	buffer: XsdFloatString,
	value: crate::XsdFloat,
	error: InvalidXsdFloatStr,
	parent_forms: {}
}

pub const NAN: &XsdFloatStr = unsafe { XsdFloatStr::new_unchecked_from_bytes(b"NaN") };
pub const POSITIVE_INFINITY: &XsdFloatStr =
	unsafe { XsdFloatStr::new_unchecked_from_bytes(b"INF") };
pub const NEGATIVE_INFINITY: &XsdFloatStr =
	unsafe { XsdFloatStr::new_unchecked_from_bytes(b"-INF") };

impl XsdFloatStr {
	pub fn is_infinite(&self) -> bool {
		matches!(self.as_str(), "INF" | "+INF" | "-INF")
	}

	pub fn is_finite(&self) -> bool {
		!matches!(self.as_str(), "INF" | "+INF" | "-INF" | "NaN")
	}

	pub fn is_nan(&self) -> bool {
		self.as_str() == "NaN"
	}

	fn exponent_separator_index(&self) -> Option<usize> {
		for (i, c) in self.0.as_bytes().iter().enumerate() {
			if matches!(c, b'e' | b'E') {
				return Some(i);
			}
		}

		None
	}

	pub fn mantissa(&self) -> Option<&XsdDecimalStr> {
		if self.is_finite() {
			Some(match self.exponent_separator_index() {
				Some(e) => unsafe { XsdDecimalStr::new_unchecked(&self.0[..e]) },
				None => unsafe { XsdDecimalStr::new_unchecked(&self.0) },
			})
		} else {
			None
		}
	}

	pub fn exponent(&self) -> Option<&XsdIntegerStr> {
		if self.is_finite() {
			self.exponent_separator_index()
				.map(|e| unsafe { XsdIntegerStr::new_unchecked(&self.0[(e + 1)..]) })
		} else {
			None
		}
	}

	pub fn value(&self) -> crate::XsdFloat {
		self.into()
	}
}

impl PartialEq for XsdFloatStr {
	fn eq(&self, other: &Self) -> bool {
		self.as_bytes() == other.as_bytes()
	}
}

impl Eq for XsdFloatStr {}

impl Hash for XsdFloatStr {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.as_bytes().hash(state)
	}
}

macro_rules! integer_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for XsdFloatString {
				fn from(i: $ty) -> Self {
					unsafe { XsdFloatString::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a XsdFloatStr> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a XsdFloatStr) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<XsdFloatString> for $ty {
				type Error = Overflow;

				fn try_from(i: XsdFloatString) -> Result<Self, Overflow> {
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

impl From<f32> for XsdFloatString {
	fn from(i: f32) -> Self {
		if i.is_finite() {
			unsafe { XsdFloatString::new_unchecked(pretty_dtoa::ftoa(i, DTOA_CONFIG)) }
		} else if i.is_nan() {
			XsdFloatString::nan()
		} else if i.is_sign_positive() {
			XsdFloatString::positive_infinity()
		} else {
			XsdFloatString::negative_infinity()
		}
	}
}

impl<'a> From<&'a XsdFloatStr> for f32 {
	fn from(i: &'a XsdFloatStr) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl From<XsdFloatString> for f32 {
	fn from(i: XsdFloatString) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl From<f64> for XsdFloatString {
	fn from(i: f64) -> Self {
		if i.is_finite() {
			unsafe { XsdFloatString::new_unchecked(pretty_dtoa::dtoa(i, DTOA_CONFIG)) }
		} else if i.is_nan() {
			XsdFloatString::nan()
		} else if i.is_sign_positive() {
			XsdFloatString::positive_infinity()
		} else {
			XsdFloatString::negative_infinity()
		}
	}
}

impl<'a> From<&'a XsdIntegerStr> for &'a XsdFloatStr {
	#[inline(always)]
	fn from(d: &'a XsdIntegerStr) -> Self {
		unsafe { XsdFloatStr::new_unchecked(d.as_str()) }
	}
}

impl<'a> From<&'a XsdNonNegativeIntegerStr> for &'a XsdFloatStr {
	#[inline(always)]
	fn from(d: &'a XsdNonNegativeIntegerStr) -> Self {
		unsafe { XsdFloatStr::new_unchecked(d.as_str()) }
	}
}

impl<'a> From<&'a XsdNonPositiveIntegerStr> for &'a XsdFloatStr {
	#[inline(always)]
	fn from(d: &'a XsdNonPositiveIntegerStr) -> Self {
		unsafe { XsdFloatStr::new_unchecked(d.as_str()) }
	}
}

impl XsdFloatString {
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
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_01() {
		XsdFloatStr::new("0").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_02() {
		XsdFloatStr::new("+").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_03() {
		XsdFloatStr::new("-").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_04() {
		XsdFloatStr::new("012+").unwrap();
	}

	#[test]
	fn parse_05() {
		XsdFloatStr::new("+42").unwrap();
	}

	#[test]
	fn parse_06() {
		XsdFloatStr::new("-42").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_07() {
		XsdFloatStr::new(".").unwrap();
	}

	#[test]
	fn parse_08() {
		XsdFloatStr::new(".0").unwrap();
	}

	#[test]
	fn parse_09() {
		XsdFloatStr::new("0.").unwrap();
	}

	#[test]
	fn parse_10() {
		XsdFloatStr::new("42.0").unwrap();
	}

	#[test]
	fn parse_11() {
		XsdFloatStr::new("INF").unwrap();
	}

	#[test]
	fn parse_12() {
		XsdFloatStr::new("-INF").unwrap();
	}

	#[test]
	fn parse_13() {
		XsdFloatStr::new("NaN").unwrap();
	}

	#[test]
	fn parse_14() {
		XsdFloatStr::new(".0e1").unwrap();
	}

	#[test]
	fn parse_15() {
		XsdFloatStr::new("0.e1").unwrap();
	}

	#[test]
	fn parse_16() {
		XsdFloatStr::new("42E10").unwrap();
	}

	#[test]
	fn parse_17() {
		XsdFloatStr::new("-42E+10").unwrap();
	}

	#[test]
	fn parse_18() {
		XsdFloatStr::new("-42E-10").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_19() {
		XsdFloatStr::new("+42E-10e").unwrap();
	}

	#[test]
	fn parse_20() {
		XsdFloatStr::new("+42E-10").unwrap();
	}

	#[test]
	fn parse_21() {
		let d = XsdFloatStr::new("+01234E-56789").unwrap();
		assert_eq!(d.mantissa(), Some(XsdDecimalStr::new("+01234").unwrap()));
		assert_eq!(d.exponent(), Some(XsdIntegerStr::new("-56789").unwrap()));
	}

	#[test]
	fn parse_22() {
		let a = XsdFloatString::new("+01234E-56789".to_string()).unwrap();
		let b = XsdFloatStr::new("+01234E-56789").unwrap();
		assert_eq!(a, b)
	}

	#[test]
	fn format_01() {
		assert_eq!(XsdFloatString::from(1.0e10f32).to_string(), "1.0e10")
	}

	#[test]
	fn parse_signed_empty_rejected() {
		XsdFloatStr::new("+.").unwrap_err();
		XsdFloatStr::new("-.e5").unwrap_err();
	}
}
