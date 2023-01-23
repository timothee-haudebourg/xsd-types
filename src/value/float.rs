use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Div, Mul, Sub},
};

use ordered_float::OrderedFloat;

use crate::{Datatype, XsdDatatype};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Float(OrderedFloat<f32>);

impl Float {
	/// Returns `true` if this value is NaN.
	#[inline(always)]
	pub fn is_nan(&self) -> bool {
		self.0 .0.is_nan()
	}

	/// Returns `true` if this number is neither infinite nor NaN.
	#[inline(always)]
	pub fn is_finite(&self) -> bool {
		self.0 .0.is_finite()
	}

	/// Returns `true` if this value is positive infinity or negative infinity, and `false` otherwise.
	#[inline(always)]
	pub fn is_infinite(&self) -> bool {
		self.0 .0.is_infinite()
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
		self.0 .0.is_sign_positive()
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
		self.0 .0.is_sign_negative()
	}

	/// Converts this value into a `f32`.
	#[inline(always)]
	pub const fn into_f32(self) -> f32 {
		self.0 .0
	}
}

// <https://www.w3.org/TR/xmlschema11-2/#f-doubleLexmap>
const XSD_CANONICAL_FLOAT: pretty_dtoa::FmtFloatConfig = pretty_dtoa::FmtFloatConfig::default()
	.force_e_notation()
	.capitalize_e(true);

impl fmt::Display for Float {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		pretty_dtoa::ftoa(self.0 .0, XSD_CANONICAL_FLOAT).fmt(f)
	}
}

impl XsdDatatype for Float {
	#[inline(always)]
	fn type_(&self) -> Datatype {
		Datatype::Float
	}
}

impl From<f32> for Float {
	#[inline(always)]
	fn from(value: f32) -> Self {
		Self(OrderedFloat(value))
	}
}

impl From<Float> for f32 {
	#[inline(always)]
	fn from(value: Float) -> Self {
		value.0 .0
	}
}

impl From<Float> for f64 {
	#[inline(always)]
	fn from(value: Float) -> Self {
		value.0 .0 as f64
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
		Self(OrderedFloat(*self.0 + *rhs.0))
	}
}

impl Sub for Float {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		Self(OrderedFloat(*self.0 - *rhs.0))
	}
}

impl Mul for Float {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self::Output {
		Self(OrderedFloat(*self.0 * *rhs.0))
	}
}

impl Div for Float {
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: Self) -> Self::Output {
		Self(OrderedFloat(*self.0 / *rhs.0))
	}
}
