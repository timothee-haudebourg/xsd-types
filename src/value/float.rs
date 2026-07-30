use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Div, Mul, Sub},
	str::FromStr,
};

use crate::{
	lexical::{self, Lexical, LexicalFormOf},
	Datatype, ParseXsd, XsdValue,
};

/// XSD `float`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Float(pub f32);

impl Float {
	pub const NEG_INFINITY: Self = Self(f32::NEG_INFINITY);
	pub const INFINITY: Self = Self(f32::INFINITY);
	pub const MIN: Self = Self(f32::MIN);
	pub const MAX: Self = Self(f32::MAX);
	pub const NAN: Self = Self(f32::NAN);

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
const XSD_CANONICAL_FLOAT: pretty_dtoa::FmtFloatConfig = pretty_dtoa::FmtFloatConfig::default()
	.force_e_notation()
	.capitalize_e(true);

impl fmt::Display for Float {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		pretty_dtoa::ftoa(self.0, XSD_CANONICAL_FLOAT).fmt(f)
	}
}

impl XsdValue for Float {
	#[inline(always)]
	fn datatype(&self) -> Datatype {
		Datatype::Float
	}
}

impl ParseXsd for Float {
	type LexicalForm = lexical::Float;
}

impl LexicalFormOf<Float> for lexical::Float {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<Float, Self::ValueError> {
		Ok(self.value())
	}
}

impl<'a> From<&'a lexical::Float> for Float {
	fn from(value: &'a lexical::Float) -> Self {
		Self::new(value.into())
	}
}

impl From<lexical::FloatBuf> for Float {
	fn from(value: lexical::FloatBuf) -> Self {
		Self::new(value.into())
	}
}

impl FromStr for Float {
	type Err = lexical::InvalidFloat<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::Float::parse(s)?;
		Ok(l.as_value())
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for Float {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Float {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = Float;

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

impl From<f32> for Float {
	#[inline(always)]
	fn from(value: f32) -> Self {
		Self(value)
	}
}

impl From<Float> for f32 {
	#[inline(always)]
	fn from(value: Float) -> Self {
		value.0
	}
}

impl From<Float> for f64 {
	#[inline(always)]
	fn from(value: Float) -> Self {
		value.0 as f64
	}
}

impl AsRef<f32> for Float {
	#[inline(always)]
	fn as_ref(&self) -> &f32 {
		&self.0
	}
}

impl Borrow<f32> for Float {
	#[inline(always)]
	fn borrow(&self) -> &f32 {
		&self.0
	}
}

impl Add for Float {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

impl Sub for Float {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		Self(self.0 - rhs.0)
	}
}

impl Mul for Float {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self::Output {
		Self(self.0 * rhs.0)
	}
}

impl Div for Float {
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: Self) -> Self::Output {
		Self(self.0 / rhs.0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// <https://www.w3.org/TR/xmlschema11-2/#float>: "NaN ≠ NaN", and "NaN is
	// incomparable with any value in the value space including itself".
	#[test]
	fn nan_is_not_equal_to_itself() {
		assert_ne!(Float::NAN, Float::NAN);
	}

	#[test]
	fn nan_is_incomparable() {
		assert_eq!(Float::NAN.partial_cmp(&Float::NAN), None);
		assert_eq!(Float::NAN.partial_cmp(&Float::new(0.0)), None);
	}

	// <https://www.w3.org/TR/xmlschema11-2/#float>: "0 = −0 (although they are
	// not identical)".
	#[test]
	fn positive_and_negative_zero_are_equal() {
		assert_eq!(Float::new(0.0), Float::new(-0.0));
	}
}
