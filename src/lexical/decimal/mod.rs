use super::lexical_form;
use crate::XsdError;
use crate::lexical::XsdLexicalForm;
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use str_newtype::StrNewType;

/// Numeric sign, re-exported from [`num_bigint`].
pub use num_bigint::Sign;

/// Error thrown when a conversion function overflowed.
pub struct Overflow;

mod integer;

pub use integer::*;

/// XsdDecimalStr number.
///
/// This is the `decimalLexicalRep` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-decimalRep>.
///
/// ```abnf
/// decimalLexicalRep = decimalPtNumeral / noDecimalPtNumeral
///
/// decimalPtNumeral = [ ("+" / "-") ] unsignedDecimalPtNumeral
///
/// unsignedDecimalPtNumeral = (unsignedNoDecimalPtNumeral "." [ 1*DIGIT ]) / ("." 1*DIGIT)
///
/// noDecimalPtNumeral = [ ("+" / "-") ] unsignedNoDecimalPtNumeral
///
/// unsignedNoDecimalPtNumeral = 1*DIGIT
/// ```
#[derive(Validate, StrNewType)]
#[automaton(crate::lexical::grammar::Decimal)]
#[newtype(owned(XsdDecimalString, derive(PartialEq, Eq)))]
pub struct XsdDecimalStr(str);

impl XsdLexicalForm for XsdDecimalStr {
	type Value = crate::XsdDecimal;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdDecimalStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdDecimalStr::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdDecimalStr(value)))
	}
}

lexical_form! {
	ty: XsdDecimalStr,
	buffer: XsdDecimalString,
	value: crate::XsdDecimal,
	error: InvalidXsdDecimalStr,
	parent_forms: {}
}

impl XsdDecimalStr {
	/// Returns `true` if `self` is positive
	/// and `false` is the number is zero or negative.
	pub fn is_positive(&self) -> bool {
		let mut sign_positive = true;
		for c in self.0.as_bytes() {
			match c {
				b'+' | b'0' | b'.' => (),
				b'-' => sign_positive = false,
				_ => return sign_positive,
			}
		}

		false
	}

	/// Returns `true` if `self` is negative
	/// and `false` is the number is zero or positive.
	pub fn is_negative(&self) -> bool {
		self.sign() == Sign::Minus
	}

	/// Returns `true` if `self` is zero
	/// and `false` otherwise.
	pub fn is_zero(&self) -> bool {
		for c in self.0.as_bytes() {
			if !matches!(c, b'+' | b'-' | b'0' | b'.') {
				return false;
			}
		}

		true
	}

	pub fn sign(&self) -> Sign {
		let mut sign_positive = true;
		for c in self.0.as_bytes() {
			match c {
				b'+' | b'0' | b'.' => (),
				b'-' => sign_positive = false,
				_ => {
					if sign_positive {
						return Sign::Plus;
					} else {
						return Sign::Minus;
					}
				}
			}
		}

		Sign::NoSign
	}

	#[inline(always)]
	pub fn integer_part(&self) -> &XsdIntegerStr {
		match self.split_once('.') {
			Some((integer_part, _)) => unsafe { XsdIntegerStr::new_unchecked(integer_part) },
			None => unsafe { XsdIntegerStr::new_unchecked(self) },
		}
	}

	#[inline(always)]
	pub fn fractional_part(&self) -> Option<&FractionalPart> {
		self.split_once('.')
			.map(|(_, fractional_part)| unsafe { FractionalPart::new_unchecked(fractional_part) })
	}

	#[inline(always)]
	pub fn trimmed_fractional_part(&self) -> Option<&FractionalPart> {
		self.split_once('.').and_then(|(_, fractional_part)| {
			let f = unsafe { FractionalPart::new_unchecked(fractional_part) }.trimmed();
			if f.is_empty() { None } else { Some(f) }
		})
	}

	#[inline(always)]
	pub fn parts(&self) -> (&XsdIntegerStr, Option<&FractionalPart>) {
		match self.split_once('.') {
			Some((i, f)) => unsafe {
				(
					XsdIntegerStr::new_unchecked(i),
					Some(FractionalPart::new_unchecked(f)),
				)
			},
			None => unsafe { (XsdIntegerStr::new_unchecked(self), None) },
		}
	}

	pub fn value(&self) -> crate::XsdDecimal {
		self.to_owned().into()
	}
}

impl PartialEq for XsdDecimalStr {
	fn eq(&self, other: &Self) -> bool {
		self.integer_part() == other.integer_part()
			&& self.fractional_part() == other.fractional_part()
	}
}

impl Eq for XsdDecimalStr {}

impl Hash for XsdDecimalStr {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.integer_part().hash(h);
		match self.fractional_part() {
			Some(f) => f.hash(h),
			None => FractionalPart::empty().hash(h),
		}
	}
}

impl Ord for XsdDecimalStr {
	fn cmp(&self, other: &Self) -> Ordering {
		let sign = self.sign();
		match sign.cmp(&other.sign()) {
			Ordering::Equal => {
				let (integer_part, fractional_part) = self.parts();
				let (other_integer_part, other_fractional_part) = other.parts();
				match integer_part.cmp(other_integer_part) {
					Ordering::Equal => {
						let fractional_part = fractional_part.unwrap_or_else(FractionalPart::empty);
						let other_fractional_part =
							other_fractional_part.unwrap_or_else(FractionalPart::empty);
						if sign == Sign::Minus {
							fractional_part.cmp(other_fractional_part).reverse()
						} else {
							fractional_part.cmp(other_fractional_part)
						}
					}
					other => {
						if sign == Sign::Minus {
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

impl PartialOrd for XsdDecimalStr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

macro_rules! integer_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for XsdDecimalString {
				fn from(i: $ty) -> Self {
					unsafe { XsdDecimalString::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a XsdDecimalStr> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a XsdDecimalStr) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<XsdDecimalString> for $ty {
				type Error = Overflow;

				fn try_from(i: XsdDecimalString) -> Result<Self, Overflow> {
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

const DTOA_CONFIG: pretty_dtoa::FmtFloatConfig =
	pretty_dtoa::FmtFloatConfig::default().force_no_e_notation();

impl From<f32> for XsdDecimalString {
	fn from(i: f32) -> Self {
		unsafe { XsdDecimalString::new_unchecked(pretty_dtoa::ftoa(i, DTOA_CONFIG)) }
	}
}

impl<'a> TryFrom<&'a XsdDecimalStr> for f32 {
	type Error = <f32 as std::str::FromStr>::Err;

	fn try_from(i: &'a XsdDecimalStr) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl TryFrom<XsdDecimalString> for f32 {
	type Error = <f32 as std::str::FromStr>::Err;

	fn try_from(i: XsdDecimalString) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl From<f64> for XsdDecimalString {
	fn from(i: f64) -> Self {
		unsafe { XsdDecimalString::new_unchecked(pretty_dtoa::dtoa(i, DTOA_CONFIG)) }
	}
}

impl<'a> TryFrom<&'a XsdDecimalStr> for f64 {
	type Error = <f64 as std::str::FromStr>::Err;

	fn try_from(i: &'a XsdDecimalStr) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl TryFrom<XsdDecimalString> for f64 {
	type Error = <f64 as std::str::FromStr>::Err;

	fn try_from(i: XsdDecimalString) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

pub struct FractionalPart([u8]);

impl FractionalPart {
	/// Creates a new fractional part from a byte slice.
	///
	/// # Safety
	///
	/// The input byte slice must be a valid fractional part lexical
	/// representation.
	#[inline(always)]
	pub unsafe fn new_unchecked<S: ?Sized + AsRef<[u8]>>(s: &S) -> &Self {
		unsafe { std::mem::transmute(s.as_ref()) }
	}

	#[inline(always)]
	pub fn empty<'a>() -> &'a Self {
		unsafe { Self::new_unchecked(b"") }
	}

	#[inline(always)]
	pub fn as_str(&self) -> &str {
		unsafe { core::str::from_utf8_unchecked(&self.0) }
	}

	#[inline(always)]
	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}

	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	/// Returns the fractional part without the trailing zeros.
	///
	/// The returned fractional part may be empty.
	pub fn trimmed(&self) -> &FractionalPart {
		let mut end = 0;
		for (i, &c) in self.0.iter().enumerate() {
			if c != b'0' {
				end = i + 1
			}
		}

		unsafe { Self::new_unchecked(&self.0[0..end]) }
	}
}

impl PartialEq for FractionalPart {
	fn eq(&self, other: &Self) -> bool {
		self.trimmed().0 == other.trimmed().0
	}
}

impl Eq for FractionalPart {}

impl Hash for FractionalPart {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.trimmed().0.hash(h)
	}
}

impl Ord for FractionalPart {
	fn cmp(&self, other: &Self) -> Ordering {
		self.trimmed().0.cmp(&other.trimmed().0)
	}
}

impl PartialOrd for FractionalPart {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
impl AsRef<[u8]> for FractionalPart {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl AsRef<str> for FractionalPart {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sign_predicates() {
		for (input, negative, positive, zero) in [
			("3.14", false, true, false),
			("+3.14", false, true, false),
			("-3.14", true, false, false),
			("-0.5", true, false, false),
			("0.0", false, false, true),
			("-0.0", false, false, true),
		] {
			let d = XsdDecimalStr::new(input).unwrap();
			assert_eq!(d.is_negative(), negative, "is_negative({input})");
			assert_eq!(d.is_positive(), positive, "is_positive({input})");
			assert_eq!(d.is_zero(), zero, "is_zero({input})");
		}
	}

	#[test]
	fn parse_01() {
		XsdDecimalStr::new("0").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_02() {
		XsdDecimalStr::new("+").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_03() {
		XsdDecimalStr::new("-").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_04() {
		XsdDecimalStr::new("012+").unwrap();
	}

	#[test]
	fn parse_05() {
		XsdDecimalStr::new("+42").unwrap();
	}

	#[test]
	fn parse_06() {
		XsdDecimalStr::new("-42").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_07() {
		XsdDecimalStr::new(".").unwrap();
	}

	#[test]
	fn parse_08() {
		XsdDecimalStr::new(".0").unwrap();
	}

	#[test]
	fn parse_09() {
		XsdDecimalStr::new("0.").unwrap();
	}

	#[test]
	fn parse_signed_empty_rejected() {
		// A sign alone, or a sign followed only by a decimal point with no
		// digits at all, is not a valid decimal.
		XsdDecimalStr::new("+.").unwrap_err();
		XsdDecimalStr::new("-.").unwrap_err();
	}

	#[test]
	fn parse_10() {
		XsdDecimalStr::new("42.0").unwrap();
	}

	#[test]
	fn format_01() {
		assert_eq!(
			XsdDecimalString::from(1.0e10f32).to_string(),
			"10000000000.0"
		)
	}

	#[test]
	fn cmp_01() {
		assert!(XsdDecimalStr::new("0.123").unwrap() < XsdDecimalStr::new("1.123").unwrap())
	}

	#[test]
	fn cmp_02() {
		assert!(XsdDecimalStr::new("0.123").unwrap() < XsdDecimalStr::new("0.1234").unwrap())
	}

	#[test]
	fn cmp_03() {
		assert!(XsdDecimalStr::new("0.123").unwrap() > XsdDecimalStr::new("-0.123").unwrap())
	}

	#[test]
	fn cmp_04() {
		assert!(XsdDecimalStr::new("-0.123").unwrap() > XsdDecimalStr::new("-0.1234").unwrap())
	}
}
