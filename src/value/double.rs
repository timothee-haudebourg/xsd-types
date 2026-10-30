use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Deref, DerefMut, Div, Mul, Sub},
	str::FromStr,
};

use crate::{lexical, Datatype, XsdFromStr, XsdTyped, XsdValue};

/// Value space of the XSD `double` datatype.
///
/// This is the decoded counterpart of [`lexical::XsdDoubleStr`]: it stores the
/// parsed IEEE 754 double-precision floating point value rather than its
/// lexical (string) representation.
/// See: <https://www.w3.org/TR/xmlschema11-2/#double>.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XsdDouble(pub f64);

impl XsdDouble {
	/// Negative infinity.
	pub const NEG_INFINITY: Self = Self(f64::NEG_INFINITY);

	/// Positive infinity.
	pub const INFINITY: Self = Self(f64::INFINITY);

	/// Smallest finite `double` value.
	pub const MIN: Self = Self(f64::MIN);

	/// Largest finite `double` value.
	pub const MAX: Self = Self(f64::MAX);

	/// Not a Number (NaN).
	pub const NAN: Self = Self(f64::NAN);

	/// Creates a new `double` value from the given `f64`.
	#[inline(always)]
	pub fn new(f: f64) -> Self {
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

	/// Converts this value into a `f64`.
	#[inline(always)]
	pub const fn into_f64(self) -> f64 {
		self.0
	}
}

// <https://www.w3.org/TR/xmlschema11-2/#f-doubleLexmap>
const XSD_CANONICAL_DOUBLE: pretty_dtoa::FmtFloatConfig = pretty_dtoa::FmtFloatConfig::default()
	.force_e_notation()
	.capitalize_e(true);

impl fmt::Display for XsdDouble {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		pretty_dtoa::dtoa(self.0, XSD_CANONICAL_DOUBLE).fmt(f)
	}
}

impl XsdTyped for XsdDouble {
	fn datatype(&self) -> Datatype {
		Datatype::Double
	}
}

impl XsdValue for XsdDouble {
	type LexicalForm = lexical::XsdDoubleStr;

	fn to_lexical_form(&self) -> lexical::XsdDoubleString {
		if self.0.is_finite() {
			// `Display` emits the canonical lexical form of finite values by
			// contract, so it is always valid and this cannot panic.
			lexical::XsdDoubleString::new(self.to_string())
				.expect("Display emits a valid double lexical form for finite values")
		} else {
			// `Display` emits `inf`/`-inf`, which are not valid lexical forms,
			// so special values go through the infallible conversion instead
			// (`NaN`, `INF`, `-INF`).
			lexical::XsdDoubleString::from(self.0)
		}
	}
}

impl<'a> XsdFromStr<'a> for XsdDouble {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdDoubleStr, Self>(lexical_value)
	}
}

impl<'a> From<&'a lexical::XsdDoubleStr> for XsdDouble {
	fn from(value: &'a lexical::XsdDoubleStr) -> Self {
		Self::new(value.into())
	}
}

impl From<lexical::XsdDoubleString> for XsdDouble {
	fn from(value: lexical::XsdDoubleString) -> Self {
		Self::new(value.into())
	}
}

impl FromStr for XsdDouble {
	type Err = lexical::InvalidXsdDoubleStr<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::XsdDoubleStr::new(s)
			.map_err(|_| lexical::InvalidXsdDoubleStr(s.to_owned()))?;
		Ok(l.into())
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdDouble {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdDouble {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdDouble;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#double")
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

impl From<f32> for XsdDouble {
	fn from(value: f32) -> Self {
		Self(value as f64)
	}
}

impl From<f64> for XsdDouble {
	fn from(value: f64) -> Self {
		Self(value)
	}
}

impl From<XsdDouble> for f64 {
	fn from(value: XsdDouble) -> Self {
		value.0
	}
}

impl AsRef<f64> for XsdDouble {
	fn as_ref(&self) -> &f64 {
		&self.0
	}
}

impl Borrow<f64> for XsdDouble {
	fn borrow(&self) -> &f64 {
		&self.0
	}
}

impl Deref for XsdDouble {
	type Target = f64;

	fn deref(&self) -> &f64 {
		&self.0
	}
}

impl DerefMut for XsdDouble {
	fn deref_mut(&mut self) -> &mut f64 {
		&mut self.0
	}
}

impl Add for XsdDouble {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

impl Sub for XsdDouble {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self(self.0 - rhs.0)
	}
}

impl Mul for XsdDouble {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self(self.0 * rhs.0)
	}
}

impl Div for XsdDouble {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self(self.0 / rhs.0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// <https://www.w3.org/TR/xmlschema11-2/#double>: "NaN ≠ NaN", and "NaN is
	// incomparable with any value in the value space including itself".
	#[test]
	fn nan_is_not_equal_to_itself() {
		assert_ne!(XsdDouble::NAN, XsdDouble::NAN);
	}

	#[test]
	fn nan_is_incomparable() {
		assert_eq!(XsdDouble::NAN.partial_cmp(&XsdDouble::NAN), None);
		assert_eq!(XsdDouble::NAN.partial_cmp(&XsdDouble::new(0.0)), None);
	}

	// <https://www.w3.org/TR/xmlschema11-2/#double>: "0 = −0 (although they are
	// not identical)".
	#[test]
	fn positive_and_negative_zero_are_equal() {
		assert_eq!(XsdDouble::new(0.0), XsdDouble::new(-0.0));
	}

	fn assert_lexical_form_round_trip(input: &str) {
		let value: XsdDouble = input.parse().unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);

		let reparsed = XsdDouble::xsd_from_str(lexical.as_str()).unwrap();
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
