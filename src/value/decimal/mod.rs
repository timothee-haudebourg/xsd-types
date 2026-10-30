use std::fmt;
use std::hash::Hash;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::LazyLock;
use std::{borrow::Borrow, collections::HashSet};

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive, Zero};

use crate::{
	XsdDatatype, XsdDecimalDatatype, XsdDouble, XsdFloat, XsdFromStr, XsdIntDatatype,
	XsdLongDatatype, XsdNonNegativeIntegerDatatype, XsdNonPositiveIntegerDatatype,
	XsdShortDatatype, XsdTyped, XsdUnsignedIntDatatype, XsdUnsignedLongDatatype,
	XsdUnsignedShortDatatype, XsdValueDomain, lexical,
};

/// Sign of a decimal or integer value, re-exported from [`num_bigint`].
pub use num_bigint::Sign;

mod integer;

pub use integer::*;

static I64_MIN: LazyLock<BigInt> = LazyLock::new(|| i64::MIN.into());
static I64_MIN_RATIO: LazyLock<BigRational> = LazyLock::new(|| I64_MIN.clone().into());
static I32_MIN: LazyLock<BigInt> = LazyLock::new(|| i32::MIN.into());
static I32_MIN_RATIO: LazyLock<BigRational> = LazyLock::new(|| I32_MIN.clone().into());
static I16_MIN: LazyLock<BigInt> = LazyLock::new(|| i16::MIN.into());
static I16_MIN_RATIO: LazyLock<BigRational> = LazyLock::new(|| I16_MIN.clone().into());
static I8_MIN: LazyLock<BigInt> = LazyLock::new(|| i8::MIN.into());
static I8_MIN_RATIO: LazyLock<BigRational> = LazyLock::new(|| I8_MIN.clone().into());
static U64_MAX: LazyLock<BigInt> = LazyLock::new(|| u64::MAX.into());
static U64_MAX_RATIO: LazyLock<BigRational> = LazyLock::new(|| U64_MAX.clone().into());
static U32_MAX: LazyLock<BigInt> = LazyLock::new(|| u32::MAX.into());
static U32_MAX_RATIO: LazyLock<BigRational> = LazyLock::new(|| U32_MAX.clone().into());
static U16_MAX: LazyLock<BigInt> = LazyLock::new(|| u16::MAX.into());
static U16_MAX_RATIO: LazyLock<BigRational> = LazyLock::new(|| U16_MAX.clone().into());
static U8_MAX: LazyLock<BigInt> = LazyLock::new(|| u8::MAX.into());
static U8_MAX_RATIO: LazyLock<BigRational> = LazyLock::new(|| U8_MAX.clone().into());
static TEN: LazyLock<BigInt> = LazyLock::new(|| 10u32.into());

/// Decimal number.
///
/// This is the value-space counterpart of [`lexical::XsdDecimalStr`]: it stores
/// the decoded number rather than its lexical (string) representation.
/// See: <https://www.w3.org/TR/xmlschema11-2/#decimal>.
///
/// Internally a decimal number is represented as a `BigRational` with a finite
/// decimal representation.
#[derive(Clone)]
pub struct XsdDecimal {
	data: BigRational,
}

impl PartialEq for XsdDecimal {
	fn eq(&self, other: &Self) -> bool {
		self.data.eq(&other.data)
	}
}

impl Eq for XsdDecimal {}

impl PartialOrd for XsdDecimal {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for XsdDecimal {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.data.cmp(&other.data)
	}
}

impl Hash for XsdDecimal {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.data.hash(state)
	}
}

impl fmt::Debug for XsdDecimal {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "XsdDecimal({:?})", self.data)
	}
}

/// Checks that a rational has a finite decimal representation.
///
/// This structure will cache some data to avoid reallocation.
/// This way running the check for multiple rational numbers can be slightly
/// more efficient.
#[derive(Default)]
pub struct DecimalCheck {
	set: HashSet<BigInt>,
}

impl DecimalCheck {
	/// Returns `true` if the given rational number has a finite decimal
	/// representation.
	///
	/// Reuses internal scratch state across calls, so repeated calls on the
	/// same `DecimalCheck` are slightly cheaper than calling the free
	/// function [`is_decimal`] repeatedly.
	pub fn is_decimal(&mut self, r: &BigRational) -> bool {
		self.set.clear();

		let mut rem = if *r < BigRational::zero() {
			-r.numer()
		} else {
			r.numer().clone()
		};

		rem %= r.denom();
		while !rem.is_zero() && !self.set.contains(&rem) {
			self.set.insert(rem.clone());
			rem = (rem * TEN.clone()) % r.denom();
		}

		rem.is_zero()
	}
}

/// Checks that the given rational has a finite decimal representation.
#[inline(always)]
pub fn is_decimal(r: &BigRational) -> bool {
	let mut c = DecimalCheck::default();
	c.is_decimal(r)
}

/// Returns the decimal lexical representation of the given rational number, if
/// any.
pub fn decimal_lexical_form(r: &BigRational) -> Option<lexical::XsdDecimalString> {
	use std::fmt::Write;

	let mut fraction = String::new();
	let mut map = std::collections::HashMap::new();

	let mut rem = if r.is_negative() {
		-r.numer()
	} else {
		r.numer().clone()
	};

	rem %= r.denom();
	while !rem.is_zero() && !map.contains_key(&rem) {
		map.insert(rem.clone(), fraction.len());
		rem *= TEN.clone();
		fraction.push_str(&(rem.clone() / r.denom()).to_string());
		rem %= r.denom();
	}

	let mut output = if r.is_negative() {
		"-".to_owned()
	} else {
		String::new()
	};

	output.push_str(&(r.numer().abs() / r.denom()).to_string());

	if rem.is_zero() {
		if !fraction.is_empty() {
			write!(output, ".{}", &fraction).unwrap();
		}

		Some(unsafe { lexical::XsdDecimalString::new_unchecked(output) })
	} else {
		None
	}
}

impl XsdDecimal {
	/// Creates a new decimal number from a rational number.
	///
	/// # Safety
	///
	/// The input rational number must have a finite decimal representation.
	pub unsafe fn new_unchecked(r: BigRational) -> Self {
		Self { data: r }
	}

	/// Returns a reference to the underlying `BigRational` value.
	#[inline(always)]
	pub fn as_big_rational(&self) -> &BigRational {
		&self.data
	}

	/// Converts this decimal number into its underlying `BigRational` value.
	#[inline(always)]
	pub fn into_big_rational(self) -> BigRational {
		self.data
	}

	/// Returns the decimal number `0`.
	#[inline(always)]
	pub fn zero() -> Self {
		Self {
			data: BigRational::zero(),
		}
	}

	/// Returns `true` if this value is zero.
	#[inline(always)]
	pub fn is_zero(&self) -> bool {
		self.data.is_zero()
	}

	/// Returns `true` if this value is strictly positive.
	#[inline(always)]
	pub fn is_positive(&self) -> bool {
		self.data.is_positive()
	}

	/// Returns `true` if this value is strictly negative.
	#[inline(always)]
	pub fn is_negative(&self) -> bool {
		self.data.is_negative()
	}

	/// Returns this value as an [`XsdInteger`] reference, if it has no
	/// fractional part.
	pub fn as_integer(&self) -> Option<&XsdInteger> {
		if self.data.is_integer() {
			Some(XsdInteger::from_bigint_ref(self.data.numer()))
		} else {
			None
		}
	}

	/// Converts this value into an [`XsdInteger`], if it has no fractional part.
	pub fn into_integer(self) -> Option<XsdInteger> {
		if self.data.is_integer() {
			Some(XsdInteger::from(self.data.numer().clone())) // TODO avoid cloning.
		} else {
			None
		}
	}

	/// Returns the most specific XSD decimal-derived datatype that can
	/// represent this value (e.g. `int`, `nonNegativeInteger`, or `decimal`
	/// itself).
	pub fn decimal_type(&self) -> XsdDecimalDatatype {
		if self.data.is_integer() {
			if self.data >= BigRational::zero() {
				if self.data > BigRational::zero() {
					if self.data <= *U8_MAX_RATIO {
						XsdUnsignedShortDatatype::UnsignedByte.into()
					} else if self.data <= *U16_MAX_RATIO {
						XsdUnsignedShortDatatype::UnsignedShort.into()
					} else if self.data <= *U32_MAX_RATIO {
						XsdUnsignedIntDatatype::UnsignedInt.into()
					} else if self.data <= *U64_MAX_RATIO {
						XsdUnsignedLongDatatype::UnsignedLong.into()
					} else {
						XsdNonNegativeIntegerDatatype::PositiveInteger.into()
					}
				} else {
					XsdUnsignedShortDatatype::UnsignedByte.into()
				}
			} else if self.data >= *I8_MIN_RATIO {
				XsdShortDatatype::Byte.into()
			} else if self.data >= *I16_MIN_RATIO {
				XsdShortDatatype::Short.into()
			} else if self.data >= *I32_MIN_RATIO {
				XsdIntDatatype::Int.into()
			} else if self.data >= *I64_MIN_RATIO {
				XsdLongDatatype::Long.into()
			} else {
				XsdNonPositiveIntegerDatatype::NegativeInteger.into()
			}
		} else {
			XsdDecimalDatatype::Decimal
		}
	}

	/// Converts this value to an `f64`, or returns `None` if it cannot be
	/// represented.
	pub fn as_f64(&self) -> Option<f64> {
		self.data.to_f64()
	}

	/// Converts this value to an `f32`, or returns `None` if it cannot be
	/// represented.
	pub fn as_f32(&self) -> Option<f32> {
		self.data.to_f32()
	}

	/// Converts this value into a [`XsdFloat`] (XSD `float`), if it can be
	/// represented as an `f32`.
	pub fn as_float(&self) -> Option<XsdFloat> {
		self.as_f32().map(XsdFloat::from)
	}

	/// Converts this value into a [`XsdDouble`] (XSD `double`), if it can be
	/// represented as an `f64`.
	pub fn as_double(&self) -> Option<XsdDouble> {
		self.as_f64().map(XsdDouble::from)
	}
}

impl fmt::Display for XsdDecimal {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self.to_lexical_form(), f)
	}
}

impl<'a> From<&'a lexical::XsdDecimalStr> for XsdDecimal {
	#[inline(always)]
	fn from(value: &'a lexical::XsdDecimalStr) -> Self {
		value.to_owned().into()
	}
}

impl From<lexical::XsdDecimalString> for XsdDecimal {
	#[inline(always)]
	fn from(value: lexical::XsdDecimalString) -> Self {
		// The sign belongs to the whole number, not just the integer part:
		// work on the magnitude and negate at the end (`-0.5` has an integer
		// part of `-0`, which is zero).
		let integer_part: BigInt = value
			.integer_part()
			.as_str()
			.parse::<BigInt>()
			.unwrap()
			.abs();
		let magnitude = match value.fractional_part() {
			Some(fract) => {
				let numer = fract.as_str().parse().unwrap();
				let mut denom = BigInt::new(Sign::Plus, vec![1u32]);
				for _ in 0..fract.as_str().len() {
					denom *= 10
				}

				BigRational::from(integer_part) + BigRational::new(numer, denom)
			}
			None => integer_part.into(),
		};

		let data = if value.is_negative() {
			-magnitude
		} else {
			magnitude
		};

		Self { data }
	}
}

impl FromStr for XsdDecimal {
	type Err = lexical::InvalidXsdDecimalStr<String>;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::XsdDecimalStr::new(s)
			.map_err(|_| lexical::InvalidXsdDecimalStr(s.to_owned()))?;
		Ok(l.value())
	}
}

macro_rules! from_int {
	($($ty:ident),*) => {
		$(
			impl From<$ty> for XsdDecimal {
				fn from(value: $ty) -> Self {
					Self {
						data: BigInt::from(value).into(),
					}
				}
			}
		)*
	};
}

from_int!(u8, u16, u32, u64, i8, i16, i32, i64, usize, isize);

macro_rules! try_into_int {
	($($ty:ident),*) => {
		$(
			impl TryFrom<XsdDecimal> for $ty {
				type Error = FromXsdDecimalError;

				fn try_from(value: XsdDecimal) -> Result<Self, FromXsdDecimalError> {
					match value.as_integer() {
						Some(i) => {
							i.try_into().map_err(|_| FromXsdDecimalError)
						}
						None => Err(FromXsdDecimalError)
					}
				}
			}

			impl<'a> TryFrom<&'a XsdDecimal> for $ty {
				type Error = FromXsdDecimalError;

				fn try_from(value: &'a XsdDecimal) -> Result<Self, FromXsdDecimalError> {
					match value.as_integer() {
						Some(i) => {
							i.try_into().map_err(|_| FromXsdDecimalError)
						}
						None => Err(FromXsdDecimalError)
					}
				}
			}
		)*
	};
}

/// Error raised when a [`XsdDecimal`] cannot be converted into the target
/// numeric type (e.g. it has a fractional part or is out of range).
#[derive(Debug, thiserror::Error)]
#[error("decimal number conversion failed")]
pub struct FromXsdDecimalError;

try_into_int!(u8, u16, u32, u64, i8, i16, i32, i64, usize, isize);

impl TryFrom<XsdDecimal> for f32 {
	type Error = FromXsdDecimalError;

	fn try_from(value: XsdDecimal) -> Result<Self, Self::Error> {
		value.as_f32().ok_or(FromXsdDecimalError)
	}
}

impl TryFrom<XsdDecimal> for XsdFloat {
	type Error = FromXsdDecimalError;

	fn try_from(value: XsdDecimal) -> Result<Self, Self::Error> {
		value.as_float().ok_or(FromXsdDecimalError)
	}
}

impl TryFrom<XsdDecimal> for f64 {
	type Error = FromXsdDecimalError;

	fn try_from(value: XsdDecimal) -> Result<Self, Self::Error> {
		value.as_f64().ok_or(FromXsdDecimalError)
	}
}

impl TryFrom<XsdDecimal> for XsdDouble {
	type Error = FromXsdDecimalError;

	fn try_from(value: XsdDecimal) -> Result<Self, Self::Error> {
		value.as_double().ok_or(FromXsdDecimalError)
	}
}

impl From<BigInt> for XsdDecimal {
	#[inline(always)]
	fn from(value: BigInt) -> Self {
		Self { data: value.into() }
	}
}

impl From<XsdInteger> for XsdDecimal {
	#[inline(always)]
	fn from(value: XsdInteger) -> Self {
		let n: BigInt = value.into();
		n.into()
	}
}

impl AsRef<BigRational> for XsdDecimal {
	#[inline(always)]
	fn as_ref(&self) -> &BigRational {
		&self.data
	}
}

impl Borrow<BigRational> for XsdDecimal {
	#[inline(always)]
	fn borrow(&self) -> &BigRational {
		&self.data
	}
}

impl Deref for XsdDecimal {
	type Target = BigRational;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

/// Error raised when trying to create a decimal value from a rational without
/// finite decimal representation.
#[derive(Debug, thiserror::Error)]
#[error("no decimal representation for rational number {0}")]
pub struct NoXsdDecimalRepresentation(pub BigRational);

impl TryFrom<BigRational> for XsdDecimal {
	type Error = NoXsdDecimalRepresentation;

	#[inline(always)]
	fn try_from(value: BigRational) -> Result<Self, Self::Error> {
		if is_decimal(&value) {
			Ok(unsafe { Self::new_unchecked(value) })
		} else {
			Err(NoXsdDecimalRepresentation(value))
		}
	}
}

impl From<XsdDecimal> for BigRational {
	#[inline(always)]
	fn from(value: XsdDecimal) -> Self {
		value.data
	}
}

impl XsdTyped for XsdDecimal {
	#[inline(always)]
	fn datatype(&self) -> XsdDatatype {
		self.decimal_type().into()
	}
}

impl XsdValueDomain for XsdDecimal {
	type LexicalForm = lexical::XsdDecimalStr;

	/// Returns the canonical lexical form of this decimal number.
	///
	/// This is recomputed on every call. Wrap the value in an
	/// [`XsdLiteral`](crate::XsdLiteral) to cache it.
	fn to_lexical_form(&self) -> lexical::XsdDecimalString {
		// A `XsdDecimal` always has a finite decimal representation (see
		// `XsdDecimal::new_unchecked`), so this cannot panic.
		decimal_lexical_form(&self.data)
			.expect("a `XsdDecimal` has a finite decimal representation")
	}
}

impl<'a> XsdFromStr<'a> for XsdDecimal {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdDecimalStr, Self>(lexical_value)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdDecimal {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdDecimal {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdDecimal;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#decimal")
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

/// Error raised when trying to convert a non-finite [`XsdFloat`] or [`XsdDouble`]
/// (NaN or infinite) into a [`XsdDecimal`], which has no representation for
/// such values.
#[derive(Debug, thiserror::Error)]
pub enum NonXsdDecimalFloat {
	/// The float is NaN.
	#[error("float is NaN")]
	Nan,

	/// The float is positive infinity.
	#[error("float is positive infinity")]
	PositiveInfinity,

	/// The float is negative infinity.
	#[error("float is negative infinity")]
	NegativeInfinity,
}

impl TryFrom<XsdFloat> for XsdDecimal {
	type Error = NonXsdDecimalFloat;

	fn try_from(value: XsdFloat) -> Result<Self, Self::Error> {
		if value.is_nan() {
			Err(NonXsdDecimalFloat::Nan)
		} else if value.is_infinite() {
			if value.is_positive() {
				Err(NonXsdDecimalFloat::PositiveInfinity)
			} else {
				Err(NonXsdDecimalFloat::NegativeInfinity)
			}
		} else {
			Ok(BigRational::from_float(value.into_f32())
				.unwrap()
				.try_into()
				.unwrap())
		}
	}
}

impl TryFrom<XsdDouble> for XsdDecimal {
	type Error = NonXsdDecimalFloat;

	fn try_from(value: XsdDouble) -> Result<Self, Self::Error> {
		if value.is_nan() {
			Err(NonXsdDecimalFloat::Nan)
		} else if value.is_infinite() {
			if value.is_sign_positive() {
				Err(NonXsdDecimalFloat::PositiveInfinity)
			} else {
				Err(NonXsdDecimalFloat::NegativeInfinity)
			}
		} else {
			Ok(BigRational::from_float(*value).unwrap().try_into().unwrap())
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn to_lexical_form_round_trip() {
		for canonical in [
			"0",
			"1",
			"3.14",
			"-0.5",
			"-42",
			"-3.25",
			"123456789012345678901234567890.000001",
		] {
			let value: XsdDecimal = canonical.parse().unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), canonical);
			assert_eq!(lexical.as_str().parse::<XsdDecimal>().unwrap(), value);
		}
	}

	#[test]
	fn to_lexical_form_is_canonical() {
		// Non-canonical inputs are normalized.
		for (input, canonical) in [("0.0", "0"), ("+1.50", "1.5"), ("007", "7"), ("-0", "0")] {
			let value: XsdDecimal = input.parse().unwrap();
			assert_eq!(value.to_lexical_form().as_str(), canonical);
		}
	}

	#[test]
	fn parse_negative_decimal() {
		for (input, numer, denom) in [
			("-0.5", -1, 2),
			("-3.25", -13, 4),
			("-42", -42, 1),
			("+3.25", 13, 4),
			("-0.0", 0, 1),
		] {
			let value: XsdDecimal = input.parse().unwrap();
			assert_eq!(
				BigRational::from(value),
				BigRational::new(numer.into(), denom.into()),
				"{input}"
			);
		}
	}

	#[test]
	fn to_lexical_form_from_xsd_parse() {
		let value = <XsdDecimal as XsdFromStr>::xsd_from_str("3.14").unwrap();
		assert_eq!(value.to_lexical_form().as_str(), "3.14");
	}
}
