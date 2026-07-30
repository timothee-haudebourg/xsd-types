use std::cell::OnceCell;
use std::fmt;
use std::hash::Hash;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::LazyLock;
use std::{borrow::Borrow, collections::HashSet};

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive, Zero};

use crate::lexical::{Lexical, LexicalFormOf};
use crate::{
	lexical, Datatype, DecimalDatatype, Double, Float, IntDatatype, LongDatatype,
	NonNegativeIntegerDatatype, NonPositiveIntegerDatatype, ParseXsd, ShortDatatype,
	UnsignedIntDatatype, UnsignedLongDatatype, UnsignedShortDatatype, XsdValue,
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
/// This is the value-space counterpart of [`lexical::Decimal`]: it stores
/// the decoded number rather than its lexical (string) representation.
/// See: <https://www.w3.org/TR/xmlschema11-2/#decimal>.
///
/// Internally a decimal number is represented as a `BigRational` with a finite
/// decimal representation.
#[derive(Clone)]
pub struct Decimal {
	data: BigRational,
	lexical: OnceCell<lexical::DecimalBuf>,
}

impl PartialEq for Decimal {
	fn eq(&self, other: &Self) -> bool {
		self.data.eq(&other.data)
	}
}

impl Eq for Decimal {}

impl PartialOrd for Decimal {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Decimal {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.data.cmp(&other.data)
	}
}

impl Hash for Decimal {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.data.hash(state)
	}
}

impl fmt::Debug for Decimal {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Decimal({:?})", self.data)
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
pub fn decimal_lexical_representation(r: &BigRational) -> Option<lexical::DecimalBuf> {
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

	output.push_str(&(r.numer() / r.denom()).to_string());

	if rem.is_zero() {
		if !fraction.is_empty() {
			write!(output, ".{}", &fraction).unwrap();
		}

		Some(unsafe { lexical::DecimalBuf::new_unchecked(output) })
	} else {
		None
	}
}

impl Decimal {
	/// Creates a new decimal number from a rational number.
	///
	/// # Safety
	///
	/// The input rational number must have a finite decimal representation.
	pub unsafe fn new_unchecked(r: BigRational) -> Self {
		Self {
			data: r,
			lexical: OnceCell::new(),
		}
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
			lexical: OnceCell::new(),
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

	/// Returns this value as an [`Integer`] reference, if it has no
	/// fractional part.
	pub fn as_integer(&self) -> Option<&Integer> {
		if self.data.is_integer() {
			Some(Integer::from_bigint_ref(self.data.numer()))
		} else {
			None
		}
	}

	/// Converts this value into an [`Integer`], if it has no fractional part.
	pub fn into_integer(self) -> Option<Integer> {
		if self.data.is_integer() {
			Some(Integer::from(self.data.numer().clone())) // TODO avoid cloning.
		} else {
			None
		}
	}

	/// Returns the most specific XSD decimal-derived datatype that can
	/// represent this value (e.g. `int`, `nonNegativeInteger`, or `decimal`
	/// itself).
	pub fn decimal_type(&self) -> DecimalDatatype {
		if self.data.is_integer() {
			if self.data >= BigRational::zero() {
				if self.data > BigRational::zero() {
					if self.data <= *U8_MAX_RATIO {
						UnsignedShortDatatype::UnsignedByte.into()
					} else if self.data <= *U16_MAX_RATIO {
						UnsignedShortDatatype::UnsignedShort.into()
					} else if self.data <= *U32_MAX_RATIO {
						UnsignedIntDatatype::UnsignedInt.into()
					} else if self.data <= *U64_MAX_RATIO {
						UnsignedLongDatatype::UnsignedLong.into()
					} else {
						NonNegativeIntegerDatatype::PositiveInteger.into()
					}
				} else {
					UnsignedShortDatatype::UnsignedByte.into()
				}
			} else if self.data >= *I8_MIN_RATIO {
				ShortDatatype::Byte.into()
			} else if self.data >= *I16_MIN_RATIO {
				ShortDatatype::Short.into()
			} else if self.data >= *I32_MIN_RATIO {
				IntDatatype::Int.into()
			} else if self.data >= *I64_MIN_RATIO {
				LongDatatype::Long.into()
			} else {
				NonPositiveIntegerDatatype::NegativeInteger.into()
			}
		} else {
			DecimalDatatype::Decimal
		}
	}

	/// Returns the canonical lexical representation of this decimal number,
	/// computing and caching it on first access.
	#[inline(always)]
	pub fn lexical_representation(&self) -> &lexical::DecimalBuf {
		self.lexical
			.get_or_init(|| decimal_lexical_representation(&self.data).unwrap())
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

	/// Converts this value into a [`Float`] (XSD `float`), if it can be
	/// represented as an `f32`.
	pub fn as_float(&self) -> Option<Float> {
		self.as_f32().map(Float::from)
	}

	/// Converts this value into a [`Double`] (XSD `double`), if it can be
	/// represented as an `f64`.
	pub fn as_double(&self) -> Option<Double> {
		self.as_f64().map(Double::from)
	}
}

impl fmt::Display for Decimal {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.lexical_representation().fmt(f)
	}
}

impl<'a> From<&'a lexical::Decimal> for Decimal {
	#[inline(always)]
	fn from(value: &'a lexical::Decimal) -> Self {
		value.to_owned().into()
	}
}

impl From<lexical::DecimalBuf> for Decimal {
	#[inline(always)]
	fn from(value: lexical::DecimalBuf) -> Self {
		let integer_part: BigInt = value.integer_part().as_str().parse().unwrap();
		let data = match value.fractional_part() {
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

		Self {
			data,
			lexical: value.into(),
		}
	}
}

impl FromStr for Decimal {
	type Err = lexical::InvalidDecimal<String>;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::Decimal::parse(s)?;
		Ok(l.value())
	}
}

macro_rules! from_int {
	($($ty:ident),*) => {
		$(
			impl From<$ty> for Decimal {
				fn from(value: $ty) -> Self {
					Self {
						data: BigInt::from(value).into(),
						lexical: OnceCell::new()
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
			impl TryFrom<Decimal> for $ty {
				type Error = FromDecimalError;

				fn try_from(value: Decimal) -> Result<Self, FromDecimalError> {
					match value.as_integer() {
						Some(i) => {
							i.try_into().map_err(|_| FromDecimalError)
						}
						None => Err(FromDecimalError)
					}
				}
			}

			impl<'a> TryFrom<&'a Decimal> for $ty {
				type Error = FromDecimalError;

				fn try_from(value: &'a Decimal) -> Result<Self, FromDecimalError> {
					match value.as_integer() {
						Some(i) => {
							i.try_into().map_err(|_| FromDecimalError)
						}
						None => Err(FromDecimalError)
					}
				}
			}
		)*
	};
}

/// Error raised when a [`Decimal`] cannot be converted into the target
/// numeric type (e.g. it has a fractional part or is out of range).
#[derive(Debug, thiserror::Error)]
#[error("decimal number conversion failed")]
pub struct FromDecimalError;

try_into_int!(u8, u16, u32, u64, i8, i16, i32, i64, usize, isize);

impl TryFrom<Decimal> for f32 {
	type Error = FromDecimalError;

	fn try_from(value: Decimal) -> Result<Self, Self::Error> {
		value.as_f32().ok_or(FromDecimalError)
	}
}

impl TryFrom<Decimal> for Float {
	type Error = FromDecimalError;

	fn try_from(value: Decimal) -> Result<Self, Self::Error> {
		value.as_float().ok_or(FromDecimalError)
	}
}

impl TryFrom<Decimal> for f64 {
	type Error = FromDecimalError;

	fn try_from(value: Decimal) -> Result<Self, Self::Error> {
		value.as_f64().ok_or(FromDecimalError)
	}
}

impl TryFrom<Decimal> for Double {
	type Error = FromDecimalError;

	fn try_from(value: Decimal) -> Result<Self, Self::Error> {
		value.as_double().ok_or(FromDecimalError)
	}
}

impl From<BigInt> for Decimal {
	#[inline(always)]
	fn from(value: BigInt) -> Self {
		Self {
			data: value.into(),
			lexical: OnceCell::new(),
		}
	}
}

impl From<Integer> for Decimal {
	#[inline(always)]
	fn from(value: Integer) -> Self {
		let n: BigInt = value.into();
		n.into()
	}
}

impl AsRef<BigRational> for Decimal {
	#[inline(always)]
	fn as_ref(&self) -> &BigRational {
		&self.data
	}
}

impl Borrow<BigRational> for Decimal {
	#[inline(always)]
	fn borrow(&self) -> &BigRational {
		&self.data
	}
}

impl Deref for Decimal {
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
pub struct NoDecimalRepresentation(pub BigRational);

impl TryFrom<BigRational> for Decimal {
	type Error = NoDecimalRepresentation;

	#[inline(always)]
	fn try_from(value: BigRational) -> Result<Self, Self::Error> {
		if is_decimal(&value) {
			Ok(unsafe { Self::new_unchecked(value) })
		} else {
			Err(NoDecimalRepresentation(value))
		}
	}
}

impl From<Decimal> for BigRational {
	#[inline(always)]
	fn from(value: Decimal) -> Self {
		value.data
	}
}

impl XsdValue for Decimal {
	#[inline(always)]
	fn datatype(&self) -> Datatype {
		self.decimal_type().into()
	}
}

impl ParseXsd for Decimal {
	type LexicalForm = lexical::Decimal;
}

impl LexicalFormOf<Decimal> for lexical::Decimal {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<Decimal, Self::ValueError> {
		Ok(self.value())
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for Decimal {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Decimal {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = Decimal;

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

/// Error raised when trying to convert a non-finite [`Float`] or [`Double`]
/// (NaN or infinite) into a [`Decimal`], which has no representation for
/// such values.
#[derive(Debug, thiserror::Error)]
pub enum NonDecimalFloat {
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

impl TryFrom<Float> for Decimal {
	type Error = NonDecimalFloat;

	fn try_from(value: Float) -> Result<Self, Self::Error> {
		if value.is_nan() {
			Err(NonDecimalFloat::Nan)
		} else if value.is_infinite() {
			if value.is_positive() {
				Err(NonDecimalFloat::PositiveInfinity)
			} else {
				Err(NonDecimalFloat::NegativeInfinity)
			}
		} else {
			Ok(BigRational::from_float(value.into_f32())
				.unwrap()
				.try_into()
				.unwrap())
		}
	}
}

impl TryFrom<Double> for Decimal {
	type Error = NonDecimalFloat;

	fn try_from(value: Double) -> Result<Self, Self::Error> {
		if value.is_nan() {
			Err(NonDecimalFloat::Nan)
		} else if value.is_infinite() {
			if value.is_sign_positive() {
				Err(NonDecimalFloat::PositiveInfinity)
			} else {
				Err(NonDecimalFloat::NegativeInfinity)
			}
		} else {
			Ok(BigRational::from_float(*value).unwrap().try_into().unwrap())
		}
	}
}
