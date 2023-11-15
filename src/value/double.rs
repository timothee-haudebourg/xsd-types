use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Deref, DerefMut, Div, Mul, Sub},
	str::FromStr,
};

use ordered_float::OrderedFloat;

use crate::{
	lexical::{self, LexicalFormOf},
	Datatype, ParseRdf, XsdValue,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Double(OrderedFloat<f64>);

impl Double {
	pub const NEG_INFINITY: Self = Self(OrderedFloat(f64::NEG_INFINITY));
	pub const INFINITY: Self = Self(OrderedFloat(f64::INFINITY));
	pub const MIN: Self = Self(OrderedFloat(f64::MIN));
	pub const MAX: Self = Self(OrderedFloat(f64::MAX));
	pub const NAN: Self = Self(OrderedFloat(f64::NAN));

	#[inline(always)]
	pub fn new(f: f64) -> Self {
		Self(OrderedFloat(f))
	}

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

	/// Converts this value into a `f64`.
	#[inline(always)]
	pub const fn into_f64(self) -> f64 {
		self.0 .0
	}
}

// <https://www.w3.org/TR/xmlschema11-2/#f-doubleLexmap>
const XSD_CANONICAL_DOUBLE: pretty_dtoa::FmtFloatConfig = pretty_dtoa::FmtFloatConfig::default()
	.force_e_notation()
	.capitalize_e(true);

impl fmt::Display for Double {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		pretty_dtoa::dtoa(self.0 .0, XSD_CANONICAL_DOUBLE).fmt(f)
	}
}

impl XsdValue for Double {
	fn datatype(&self) -> Datatype {
		Datatype::Double
	}
}

impl ParseRdf for Double {
	type LexicalForm = lexical::Double;
}

impl LexicalFormOf<Double> for lexical::Double {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<Double, Self::ValueError> {
		Ok(self.value())
	}
}

impl<'a> From<&'a lexical::Double> for Double {
	fn from(value: &'a lexical::Double) -> Self {
		Self::new(value.into())
	}
}

impl From<lexical::DoubleBuf> for Double {
	fn from(value: lexical::DoubleBuf) -> Self {
		Self::new(value.into())
	}
}

impl FromStr for Double {
	type Err = lexical::InvalidDouble;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::Double::new(s)?;
		Ok(l.into())
	}
}

impl From<f32> for Double {
	fn from(value: f32) -> Self {
		Self(OrderedFloat(value as f64))
	}
}

impl From<f64> for Double {
	fn from(value: f64) -> Self {
		Self(OrderedFloat(value))
	}
}

impl From<Double> for f64 {
	fn from(value: Double) -> Self {
		value.0 .0
	}
}

impl AsRef<f64> for Double {
	fn as_ref(&self) -> &f64 {
		&self.0
	}
}

impl Borrow<f64> for Double {
	fn borrow(&self) -> &f64 {
		&self.0
	}
}

impl Deref for Double {
	type Target = f64;

	fn deref(&self) -> &f64 {
		&self.0
	}
}

impl DerefMut for Double {
	fn deref_mut(&mut self) -> &mut f64 {
		&mut self.0
	}
}

impl Add for Double {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(OrderedFloat(*self.0 + *rhs.0))
	}
}

impl Sub for Double {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self(OrderedFloat(*self.0 - *rhs.0))
	}
}

impl Mul for Double {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self(OrderedFloat(*self.0 * *rhs.0))
	}
}

impl Div for Double {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self(OrderedFloat(*self.0 / *rhs.0))
	}
}
