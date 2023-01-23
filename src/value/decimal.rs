use std::fmt;
use std::hash::Hash;
use std::ops::Deref;
use std::{borrow::Borrow, collections::HashSet};

use lazy_static::lazy_static;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};
use once_cell::unsync::OnceCell;

use crate::{
	lexical, Datatype, DecimalDatatype, Double, Float, IntDatatype, IntegerDatatype, LongDatatype,
	NonNegativeIntegerDatatype, NonPositiveIntegerDatatype, ShortDatatype, UnsignedIntDatatype,
	UnsignedLongDatatype, UnsignedShortDatatype, XsdDatatype,
};

mod integer;

pub use integer::*;

lazy_static! {
	static ref I64_MIN: BigInt = i64::MIN.into();
	static ref I64_MIN_RATIO: BigRational = I64_MIN.clone().into();
	static ref I32_MIN: BigInt = i32::MIN.into();
	static ref I32_MIN_RATIO: BigRational = I32_MIN.clone().into();
	static ref I16_MIN: BigInt = i16::MIN.into();
	static ref I16_MIN_RATIO: BigRational = I16_MIN.clone().into();
	static ref I8_MIN: BigInt = i8::MIN.into();
	static ref I8_MIN_RATIO: BigRational = I8_MIN.clone().into();
	static ref U64_MAX: BigInt = u64::MAX.into();
	static ref U64_MAX_RATIO: BigRational = U64_MAX.clone().into();
	static ref U32_MAX: BigInt = u32::MAX.into();
	static ref U32_MAX_RATIO: BigRational = U32_MAX.clone().into();
	static ref U16_MAX: BigInt = u16::MAX.into();
	static ref U16_MAX_RATIO: BigRational = U16_MAX.clone().into();
	static ref U8_MAX: BigInt = u8::MAX.into();
	static ref U8_MAX_RATIO: BigRational = U8_MAX.clone().into();
	static ref TEN: BigInt = 10u32.into();
}

/// Decimal number.
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
		self.data.partial_cmp(&other.data)
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

	#[inline(always)]
	pub fn zero() -> Self {
		Self {
			data: BigRational::zero(),
			lexical: OnceCell::new(),
		}
	}

	#[inline(always)]
	pub fn is_zero(&self) -> bool {
		self.data.is_zero()
	}

	#[inline(always)]
	pub fn is_positive(&self) -> bool {
		self.data.is_positive()
	}

	#[inline(always)]
	pub fn is_negative(&self) -> bool {
		self.data.is_negative()
	}

	pub fn decimal_type(&self) -> Option<DecimalDatatype> {
		if self.data.is_integer() {
			if self.data >= BigRational::zero() {
				if self.data > BigRational::zero() {
					if self.data <= *U8_MAX_RATIO {
						Some(UnsignedShortDatatype::UnsignedByte.into())
					} else if self.data <= *U16_MAX_RATIO {
						Some(UnsignedIntDatatype::UnsignedShort(None).into())
					} else if self.data <= *U32_MAX_RATIO {
						Some(UnsignedLongDatatype::UnsignedInt(None).into())
					} else if self.data <= *U64_MAX_RATIO {
						Some(NonNegativeIntegerDatatype::UnsignedLong(None).into())
					} else {
						Some(NonNegativeIntegerDatatype::PositiveInteger.into())
					}
				} else {
					Some(UnsignedShortDatatype::UnsignedByte.into())
				}
			} else if self.data >= *I8_MIN_RATIO {
				Some(ShortDatatype::Byte.into())
			} else if self.data >= *I16_MIN_RATIO {
				Some(IntDatatype::Short(None).into())
			} else if self.data >= *I32_MIN_RATIO {
				Some(LongDatatype::Int(None).into())
			} else if self.data >= *I64_MIN_RATIO {
				Some(IntegerDatatype::Long(None).into())
			} else {
				Some(NonPositiveIntegerDatatype::NegativeInteger.into())
			}
		} else {
			None
		}
	}

	#[inline(always)]
	pub fn lexical_representation(&self) -> &lexical::DecimalBuf {
		self.lexical
			.get_or_init(|| decimal_lexical_representation(&self.data).unwrap())
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
		let numer: BigInt = value.integer_part().as_str().parse().unwrap();
		let data = match value.fractional_part() {
			Some(fract) => {
				let f = BigRational::new(1.into(), fract.as_str().len().into());
				let fract: BigRational = fract.as_str().parse().unwrap();
				BigRational::from(numer) + fract * f
			}
			None => numer.into(),
		};

		Self {
			data,
			lexical: value.into(),
		}
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

impl XsdDatatype for Decimal {
	#[inline(always)]
	fn type_(&self) -> Datatype {
		self.decimal_type().into()
	}
}

#[derive(Debug, thiserror::Error)]
pub enum NonDecimalFloat {
	#[error("float is NaN")]
	Nan,

	#[error("float is positive infinity")]
	PositiveInfinity,

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
