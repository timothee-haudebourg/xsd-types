use super::{XsdDecimalStr, XsdFloatStr, XsdFloatString, XsdIntegerStr, XsdOverflow, lexical_form};
use crate::XsdError;
use crate::lexical::XsdLexicalForm;
use static_automata::Validate;
use std::hash::Hash;
use str_newtype::StrNewType;

/// XsdDoubleStr number.
///
/// This is the `doubleRep` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-doubleRep>.
/// It is identical to the `floatRep` production used by [`XsdFloatStr`]; only
/// the value spaces (precision) of `float` and `double` differ.
///
/// ```abnf
/// doubleRep = noDecimalPtNumeral / decimalPtNumeral / scientificNotationNumeral / numericalSpecialRep
///
/// scientificNotationNumeral = [ ("+" / "-") ] (unsignedNoDecimalPtNumeral / unsignedDecimalPtNumeral) "e" noDecimalPtNumeral
///
/// numericalSpecialRep = %s"+INF" / %s"INF" / %s"-INF" / %s"NaN"
/// ```
#[derive(Validate, StrNewType)]
#[automaton(crate::lexical::grammar::Double)]
#[newtype(owned(XsdDoubleString, derive(PartialEq, Eq)))]
pub struct XsdDoubleStr(str);

impl XsdLexicalForm for XsdDoubleStr {
	type Value = crate::XsdDouble;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdDoubleStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdDoubleStr::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdDoubleStr(value)))
	}
}

lexical_form! {
	ty: XsdDoubleStr,
	buffer: XsdDoubleString,
	value: crate::XsdDouble,
	error: InvalidXsdDoubleStr,
	parent_forms: {}
}

pub const XSD_DOUBLE_NAN: &XsdDoubleStr = unsafe { XsdDoubleStr::new_unchecked_from_bytes(b"NaN") };
pub const XSD_DOUBLE_POSITIVE_INFINITY: &XsdDoubleStr =
	unsafe { XsdDoubleStr::new_unchecked_from_bytes(b"INF") };
pub const XSD_DOUBLE_NEGATIVE_INFINITY: &XsdDoubleStr =
	unsafe { XsdDoubleStr::new_unchecked_from_bytes(b"-INF") };

impl XsdDoubleStr {
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

	pub fn value(&self) -> crate::XsdDouble {
		self.into()
	}
}

impl PartialEq for XsdDoubleStr {
	fn eq(&self, other: &Self) -> bool {
		self.as_bytes() == other.as_bytes()
	}
}

impl Eq for XsdDoubleStr {}

impl Hash for XsdDoubleStr {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.as_bytes().hash(state)
	}
}

macro_rules! integer_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for XsdDoubleString {
				fn from(i: $ty) -> Self {
					unsafe { XsdDoubleString::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a XsdDoubleStr> for $ty {
				type Error = XsdOverflow;

				fn try_from(i: &'a XsdDoubleStr) -> Result<Self, XsdOverflow> {
					i.as_str().parse().map_err(|_| XsdOverflow)
				}
			}

			impl TryFrom<XsdDoubleString> for $ty {
				type Error = XsdOverflow;

				fn try_from(i: XsdDoubleString) -> Result<Self, XsdOverflow> {
					i.as_str().parse().map_err(|_| XsdOverflow)
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

impl From<f32> for XsdDoubleString {
	fn from(i: f32) -> Self {
		if i.is_finite() {
			unsafe { XsdDoubleString::new_unchecked(pretty_dtoa::ftoa(i, DTOA_CONFIG)) }
		} else if i.is_nan() {
			XsdDoubleString::nan()
		} else if i.is_sign_positive() {
			XsdDoubleString::positive_infinity()
		} else {
			XsdDoubleString::negative_infinity()
		}
	}
}

impl<'a> From<&'a XsdDoubleStr> for f64 {
	fn from(i: &'a XsdDoubleStr) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl From<XsdDoubleString> for f64 {
	fn from(i: XsdDoubleString) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl<'a> From<&'a XsdFloatStr> for f64 {
	fn from(i: &'a XsdFloatStr) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl From<XsdFloatString> for f64 {
	fn from(i: XsdFloatString) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl From<f64> for XsdDoubleString {
	fn from(i: f64) -> Self {
		if i.is_finite() {
			unsafe { XsdDoubleString::new_unchecked(pretty_dtoa::dtoa(i, DTOA_CONFIG)) }
		} else if i.is_nan() {
			XsdDoubleString::nan()
		} else if i.is_sign_positive() {
			XsdDoubleString::positive_infinity()
		} else {
			XsdDoubleString::negative_infinity()
		}
	}
}

impl<'a> From<&'a XsdDecimalStr> for &'a XsdDoubleStr {
	#[inline(always)]
	fn from(d: &'a XsdDecimalStr) -> Self {
		unsafe { XsdDoubleStr::new_unchecked(d.as_str()) }
	}
}

impl XsdDoubleString {
	#[inline(always)]
	pub fn nan() -> Self {
		XSD_DOUBLE_NAN.to_owned()
	}

	#[inline(always)]
	pub fn positive_infinity() -> Self {
		XSD_DOUBLE_POSITIVE_INFINITY.to_owned()
	}

	#[inline(always)]
	pub fn negative_infinity() -> Self {
		XSD_DOUBLE_NEGATIVE_INFINITY.to_owned()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_01() {
		XsdDoubleStr::new("0").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_02() {
		XsdDoubleStr::new("+").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_03() {
		XsdDoubleStr::new("-").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_04() {
		XsdDoubleStr::new("012+").unwrap();
	}

	#[test]
	fn parse_05() {
		XsdDoubleStr::new("+42").unwrap();
	}

	#[test]
	fn parse_06() {
		XsdDoubleStr::new("-42").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_07() {
		XsdDoubleStr::new(".").unwrap();
	}

	#[test]
	fn parse_08() {
		XsdDoubleStr::new(".0").unwrap();
	}

	#[test]
	fn parse_09() {
		XsdDoubleStr::new("0.").unwrap();
	}

	#[test]
	fn parse_10() {
		XsdDoubleStr::new("42.0").unwrap();
	}

	#[test]
	fn parse_11() {
		XsdDoubleStr::new("INF").unwrap();
	}

	#[test]
	fn parse_12() {
		XsdDoubleStr::new("-INF").unwrap();
	}

	#[test]
	fn parse_13() {
		XsdDoubleStr::new("NaN").unwrap();
	}

	#[test]
	fn parse_14() {
		XsdDoubleStr::new(".0e1").unwrap();
	}

	#[test]
	fn parse_15() {
		XsdDoubleStr::new("0.e1").unwrap();
	}

	#[test]
	fn parse_16() {
		XsdDoubleStr::new("42E10").unwrap();
	}

	#[test]
	fn parse_17() {
		XsdDoubleStr::new("-42E+10").unwrap();
	}

	#[test]
	fn parse_18() {
		XsdDoubleStr::new("-42E-10").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_19() {
		XsdDoubleStr::new("+42E-10e").unwrap();
	}

	#[test]
	fn parse_20() {
		XsdDoubleStr::new("+42E-10").unwrap();
	}

	#[test]
	fn parse_21() {
		let d = XsdDoubleStr::new("+01234E-56789").unwrap();
		assert_eq!(d.mantissa(), Some(XsdDecimalStr::new("+01234").unwrap()));
		assert_eq!(d.exponent(), Some(XsdIntegerStr::new("-56789").unwrap()));
	}

	#[test]
	fn parse_22() {
		let a = XsdDoubleString::new("+01234E-56789".to_string()).unwrap();
		let b = XsdDoubleStr::new("+01234E-56789").unwrap();
		assert_eq!(a, b)
	}

	#[test]
	fn format_01() {
		assert_eq!(XsdDoubleString::from(1.0e10f64).to_string(), "1.0e10")
	}

	#[test]
	fn parse_signed_empty_rejected() {
		XsdDoubleStr::new("+.").unwrap_err();
		XsdDoubleStr::new("-.e5").unwrap_err();
	}
}
