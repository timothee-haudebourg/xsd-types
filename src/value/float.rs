use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Div, Mul, Sub},
	str::FromStr,
};

use crate::{Datatype, XsdFromStr, XsdTyped, XsdValue, lexical};

/// Value space of the XSD `float` datatype.
///
/// This is the decoded counterpart of [`lexical::XsdFloatStr`]: it stores the
/// parsed IEEE 754 single-precision floating point value rather than its
/// lexical (string) representation.
/// See: <https://www.w3.org/TR/xmlschema11-2/#float>.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XsdFloat(pub f32);

impl XsdFloat {
	/// Negative infinity.
	pub const NEG_INFINITY: Self = Self(f32::NEG_INFINITY);

	/// Positive infinity.
	pub const INFINITY: Self = Self(f32::INFINITY);

	/// Smallest finite `float` value.
	pub const MIN: Self = Self(f32::MIN);

	/// Largest finite `float` value.
	pub const MAX: Self = Self(f32::MAX);

	/// Not a Number (NaN).
	pub const NAN: Self = Self(f32::NAN);

	/// Creates a new `float` value from the given `f32`.
	#[inline(always)]
	pub fn new(f: f32) -> Self {
		Self(f)
	}

	/// Returns `true` if this value is NaN.
	#[inline(always)]
	pub fn is_nan(&self) -> bool {
		self.0.is_nan()
	}

	/// Returns `true` if this number is neither infinite nor NaN.
	#[inline(always)]
	pub fn is_finite(&self) -> bool {
		self.0.is_finite()
	}

	/// Returns `true` if this value is positive infinity or negative infinity, and `false` otherwise.
	#[inline(always)]
	pub fn is_infinite(&self) -> bool {
		self.0.is_infinite()
	}

	/// Returns `true` if `self` has a positive sign, including +0.0, NaNs with
	/// positive sign bit and positive infinity.
	///
	/// Note that IEEE 754 doesn't assign any meaning to the sign bit in case
	/// of a NaN, and as Rust doesn't guarantee that the bit pattern of NaNs
	/// are conserved over arithmetic operations, the result of
	/// `is_positive` on a NaN might produce an unexpected result in some
	/// cases.
	/// See [explanation of NaN as a special value](https://doc.rust-lang.org/nightly/core/primitive.f32.html)
	/// for more info.
	#[inline(always)]
	pub fn is_positive(&self) -> bool {
		self.0.is_sign_positive()
	}

	/// Returns `false` if `self` has a negative sign, including -0.0, NaNs with
	/// negative sign bit and negative infinity.
	///
	/// Note that IEEE 754 doesn't assign any meaning to the sign bit in case
	/// of a NaN, and as Rust doesn't guarantee that the bit pattern of NaNs
	/// are conserved over arithmetic operations, the result of
	/// `is_negative` on a NaN might produce an unexpected result in some
	/// cases.
	/// See [explanation of NaN as a special value](https://doc.rust-lang.org/nightly/core/primitive.f32.html)
	/// for more info.
	#[inline(always)]
	pub fn is_negative(&self) -> bool {
		self.0.is_sign_negative()
	}

	/// Converts this value into a `f32`.
	#[inline(always)]
	pub const fn into_f32(self) -> f32 {
		self.0
	}
}

// <https://www.w3.org/TR/xmlschema11-2/#f-doubleLexmap>
//
// This configuration only covers finite values. `pretty_dtoa` hardcodes the
// special values as `inf`/`-inf` (not configurable), which are not valid XSD
// lexical forms (`INF`/`-INF`), so infinities are written by `Display`
// directly. `NaN` is already written correctly.
const XSD_CANONICAL_FLOAT: pretty_dtoa::FmtFloatConfig = pretty_dtoa::FmtFloatConfig::default()
	.force_e_notation()
	.capitalize_e(true);

/// Writes the canonical lexical form of the value.
///
/// Special values are written as `NaN`, `INF` and `-INF`.
impl fmt::Display for XsdFloat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.0.is_infinite() {
			// `pretty_dtoa` hardcodes `inf`/`-inf`, which are not valid
			// lexical forms. It writes `NaN` correctly.
			f.write_str(if self.0.is_sign_positive() {
				"INF"
			} else {
				"-INF"
			})
		} else {
			pretty_dtoa::ftoa(self.0, XSD_CANONICAL_FLOAT).fmt(f)
		}
	}
}

impl XsdTyped for XsdFloat {
	#[inline(always)]
	fn datatype(&self) -> Datatype {
		Datatype::Float
	}
}

impl XsdValue for XsdFloat {
	type LexicalForm = lexical::XsdFloatStr;

	fn to_lexical_form(&self) -> lexical::XsdFloatString {
		// `Display` emits the canonical lexical form by contract.
		lexical::XsdFloatString::new(self.to_string())
			.expect("Display emits a valid float lexical form")
	}
}

impl<'a> XsdFromStr<'a> for XsdFloat {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdFloatStr, Self>(lexical_value)
	}
}

impl<'a> From<&'a lexical::XsdFloatStr> for XsdFloat {
	fn from(value: &'a lexical::XsdFloatStr) -> Self {
		Self::new(value.into())
	}
}

impl From<lexical::XsdFloatString> for XsdFloat {
	fn from(value: lexical::XsdFloatString) -> Self {
		Self::new(value.into())
	}
}

impl FromStr for XsdFloat {
	type Err = lexical::InvalidXsdFloatStr<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l =
			lexical::XsdFloatStr::new(s).map_err(|_| lexical::InvalidXsdFloatStr(s.to_owned()))?;
		Ok(l.into())
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdFloat {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdFloat {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdFloat;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#float")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				v.parse().map_err(|e| E::custom(e))
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}

impl From<f32> for XsdFloat {
	#[inline(always)]
	fn from(value: f32) -> Self {
		Self(value)
	}
}

impl From<XsdFloat> for f32 {
	#[inline(always)]
	fn from(value: XsdFloat) -> Self {
		value.0
	}
}

impl From<XsdFloat> for f64 {
	#[inline(always)]
	fn from(value: XsdFloat) -> Self {
		value.0 as f64
	}
}

impl AsRef<f32> for XsdFloat {
	#[inline(always)]
	fn as_ref(&self) -> &f32 {
		&self.0
	}
}

impl Borrow<f32> for XsdFloat {
	#[inline(always)]
	fn borrow(&self) -> &f32 {
		&self.0
	}
}

impl Add for XsdFloat {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

impl Sub for XsdFloat {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		Self(self.0 - rhs.0)
	}
}

impl Mul for XsdFloat {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self::Output {
		Self(self.0 * rhs.0)
	}
}

impl Div for XsdFloat {
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: Self) -> Self::Output {
		Self(self.0 / rhs.0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// `Display` must write the canonical lexical form, including for special
	// values, which must parse back.
	#[test]
	fn display_is_canonical() {
		for (value, expected) in [
			(XsdFloat::INFINITY, "INF"),
			(XsdFloat::NEG_INFINITY, "-INF"),
			(XsdFloat::NAN, "NaN"),
			(XsdFloat(1.5), "1.5E0"),
			(XsdFloat(-2500.0), "-2.5E3"),
		] {
			assert_eq!(value.to_string(), expected);
			assert_eq!(value.to_lexical_form().as_str(), expected);
			assert!(expected.parse::<XsdFloat>().is_ok());
		}
	}

	// <https://www.w3.org/TR/xmlschema11-2/#float>: "NaN ≠ NaN", and "NaN is
	// incomparable with any value in the value space including itself".
	#[test]
	fn nan_is_not_equal_to_itself() {
		assert_ne!(XsdFloat::NAN, XsdFloat::NAN);
	}

	#[test]
	fn nan_is_incomparable() {
		assert_eq!(XsdFloat::NAN.partial_cmp(&XsdFloat::NAN), None);
		assert_eq!(XsdFloat::NAN.partial_cmp(&XsdFloat::new(0.0)), None);
	}

	// <https://www.w3.org/TR/xmlschema11-2/#float>: "0 = −0 (although they are
	// not identical)".
	#[test]
	fn positive_and_negative_zero_are_equal() {
		assert_eq!(XsdFloat::new(0.0), XsdFloat::new(-0.0));
	}

	fn assert_lexical_form_round_trip(input: &str) {
		let value: XsdFloat = input.parse().unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);

		let reparsed = XsdFloat::xsd_from_str(lexical.as_str()).unwrap();
		if value.is_nan() {
			assert!(reparsed.is_nan());
		} else {
			assert_eq!(reparsed, value);
			assert_eq!(reparsed.is_negative(), value.is_negative());
		}
	}

	#[test]
	fn to_lexical_form_round_trips_finite_values() {
		for input in ["1.0E0", "-1.5E3", "2.5E-3", "0.0E0", "-0.0E0"] {
			assert_lexical_form_round_trip(input);
		}
	}

	#[test]
	fn to_lexical_form_round_trips_special_values() {
		for input in ["INF", "-INF", "NaN"] {
			assert_lexical_form_round_trip(input);
		}
	}
}
