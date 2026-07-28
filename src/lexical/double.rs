use super::{lexical_form, Decimal, Float, FloatBuf, Integer, Overflow};
use crate::lexical::Lexical;
use static_automata::Validate;
use std::hash::Hash;
use str_newtype::StrNewType;

/// Double number.
///
/// This is the `doubleRep` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-doubleRep>.
/// It is identical to the `floatRep` production used by [`Float`]; only
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
#[newtype(owned(DoubleBuf, derive(PartialEq, Eq)))]
pub struct Double(str);

impl Lexical for Double {
	type Error = InvalidDouble<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidDouble(value.to_owned()))
	}
}

lexical_form! {
	ty: Double,
	buffer: DoubleBuf,
	value: crate::Double,
	error: InvalidDouble,
	parent_forms: {}
}

pub const NAN: &Double = unsafe { Double::new_unchecked_from_bytes(b"NaN") };
pub const POSITIVE_INFINITY: &Double = unsafe { Double::new_unchecked_from_bytes(b"INF") };
pub const NEGATIVE_INFINITY: &Double = unsafe { Double::new_unchecked_from_bytes(b"-INF") };

impl Double {
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

	pub fn mantissa(&self) -> Option<&Decimal> {
		if self.is_finite() {
			Some(match self.exponent_separator_index() {
				Some(e) => unsafe { Decimal::new_unchecked(&self.0[..e]) },
				None => unsafe { Decimal::new_unchecked(&self.0) },
			})
		} else {
			None
		}
	}

	pub fn exponent(&self) -> Option<&Integer> {
		if self.is_finite() {
			self.exponent_separator_index()
				.map(|e| unsafe { Integer::new_unchecked(&self.0[(e + 1)..]) })
		} else {
			None
		}
	}

	pub fn value(&self) -> crate::Double {
		self.into()
	}
}

impl PartialEq for Double {
	fn eq(&self, other: &Self) -> bool {
		self.as_bytes() == other.as_bytes()
	}
}

impl Eq for Double {}

impl Hash for Double {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.as_bytes().hash(state)
	}
}

macro_rules! integer_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for DoubleBuf {
				fn from(i: $ty) -> Self {
					unsafe { DoubleBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a Double> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a Double) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<DoubleBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: DoubleBuf) -> Result<Self, Overflow> {
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

impl From<f32> for DoubleBuf {
	fn from(i: f32) -> Self {
		if i.is_finite() {
			unsafe { DoubleBuf::new_unchecked(pretty_dtoa::ftoa(i, DTOA_CONFIG)) }
		} else if i.is_nan() {
			DoubleBuf::nan()
		} else if i.is_sign_positive() {
			DoubleBuf::positive_infinity()
		} else {
			DoubleBuf::negative_infinity()
		}
	}
}

impl<'a> From<&'a Double> for f64 {
	fn from(i: &'a Double) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl From<DoubleBuf> for f64 {
	fn from(i: DoubleBuf) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl<'a> From<&'a Float> for f64 {
	fn from(i: &'a Float) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl From<FloatBuf> for f64 {
	fn from(i: FloatBuf) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl From<f64> for DoubleBuf {
	fn from(i: f64) -> Self {
		if i.is_finite() {
			unsafe { DoubleBuf::new_unchecked(pretty_dtoa::dtoa(i, DTOA_CONFIG)) }
		} else if i.is_nan() {
			DoubleBuf::nan()
		} else if i.is_sign_positive() {
			DoubleBuf::positive_infinity()
		} else {
			DoubleBuf::negative_infinity()
		}
	}
}

impl<'a> From<&'a Decimal> for &'a Double {
	#[inline(always)]
	fn from(d: &'a Decimal) -> Self {
		unsafe { Double::new_unchecked(d.as_str()) }
	}
}

impl DoubleBuf {
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
		Double::new("0").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_02() {
		Double::new("+").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_03() {
		Double::new("-").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_04() {
		Double::new("012+").unwrap();
	}

	#[test]
	fn parse_05() {
		Double::new("+42").unwrap();
	}

	#[test]
	fn parse_06() {
		Double::new("-42").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_07() {
		Double::new(".").unwrap();
	}

	#[test]
	fn parse_08() {
		Double::new(".0").unwrap();
	}

	#[test]
	fn parse_09() {
		Double::new("0.").unwrap();
	}

	#[test]
	fn parse_10() {
		Double::new("42.0").unwrap();
	}

	#[test]
	fn parse_11() {
		Double::new("INF").unwrap();
	}

	#[test]
	fn parse_12() {
		Double::new("-INF").unwrap();
	}

	#[test]
	fn parse_13() {
		Double::new("NaN").unwrap();
	}

	#[test]
	fn parse_14() {
		Double::new(".0e1").unwrap();
	}

	#[test]
	fn parse_15() {
		Double::new("0.e1").unwrap();
	}

	#[test]
	fn parse_16() {
		Double::new("42E10").unwrap();
	}

	#[test]
	fn parse_17() {
		Double::new("-42E+10").unwrap();
	}

	#[test]
	fn parse_18() {
		Double::new("-42E-10").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_19() {
		Double::new("+42E-10e").unwrap();
	}

	#[test]
	fn parse_20() {
		Double::new("+42E-10").unwrap();
	}

	#[test]
	fn parse_21() {
		let d = Double::new("+01234E-56789").unwrap();
		assert_eq!(d.mantissa(), Some(Decimal::new("+01234").unwrap()));
		assert_eq!(d.exponent(), Some(Integer::new("-56789").unwrap()));
	}

	#[test]
	fn parse_22() {
		let a = DoubleBuf::new("+01234E-56789".to_string()).unwrap();
		let b = Double::new("+01234E-56789").unwrap();
		assert_eq!(a, b)
	}

	#[test]
	fn format_01() {
		assert_eq!(DoubleBuf::from(1.0e10f64).to_string(), "1.0e10")
	}

	#[test]
	fn parse_signed_empty_rejected() {
		Double::new("+.").unwrap_err();
		Double::new("-.e5").unwrap_err();
	}
}
