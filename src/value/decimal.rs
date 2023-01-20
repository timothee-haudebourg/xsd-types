use std::ops::Deref;
use std::{borrow::Borrow, collections::HashSet};

use lazy_static::lazy_static;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

use crate::{
	lexical, Datatype, DecimalDatatype, IntDatatype, IntegerDatatype, LongDatatype,
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
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Decimal(BigRational);

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

impl Decimal {
	pub fn decimal_type(&self) -> Option<DecimalDatatype> {
		if self.0.is_integer() {
			if self.0 >= BigRational::zero() {
				if self.0 > BigRational::zero() {
					if self.0 <= *U8_MAX_RATIO {
						Some(UnsignedShortDatatype::UnsignedByte.into())
					} else if self.0 <= *U16_MAX_RATIO {
						Some(UnsignedIntDatatype::UnsignedShort(None).into())
					} else if self.0 <= *U32_MAX_RATIO {
						Some(UnsignedLongDatatype::UnsignedInt(None).into())
					} else if self.0 <= *U64_MAX_RATIO {
						Some(NonNegativeIntegerDatatype::UnsignedLong(None).into())
					} else {
						Some(NonNegativeIntegerDatatype::PositiveInteger.into())
					}
				} else {
					Some(UnsignedShortDatatype::UnsignedByte.into())
				}
			} else if self.0 >= *I8_MIN_RATIO {
				Some(ShortDatatype::Byte.into())
			} else if self.0 >= *I16_MIN_RATIO {
				Some(IntDatatype::Short(None).into())
			} else if self.0 >= *I32_MIN_RATIO {
				Some(LongDatatype::Int(None).into())
			} else if self.0 >= *I64_MIN_RATIO {
				Some(IntegerDatatype::Long(None).into())
			} else {
				Some(NonPositiveIntegerDatatype::NegativeInteger.into())
			}
		} else {
			None
		}
	}
}

impl<'a> From<&'a lexical::Decimal> for Decimal {
	fn from(value: &'a lexical::Decimal) -> Self {
		let numer: BigInt = value.integer_part().as_str().parse().unwrap();
		match value.fractional_part() {
			Some(fract) => {
				let f = BigRational::new(1.into(), fract.as_str().len().into());
				let fract: BigRational = fract.as_str().parse().unwrap();
				Self(BigRational::from(numer) + fract * f)
			}
			None => Self(numer.into()),
		}
	}
}

impl From<lexical::DecimalBuf> for Decimal {
	#[inline(always)]
	fn from(value: lexical::DecimalBuf) -> Self {
		value.as_decimal().into()
	}
}

impl From<Integer> for Decimal {
	fn from(value: Integer) -> Self {
		let n: BigInt = value.into();
		Self(n.into())
	}
}

impl AsRef<BigRational> for Decimal {
	#[inline(always)]
	fn as_ref(&self) -> &BigRational {
		&self.0
	}
}

impl Borrow<BigRational> for Decimal {
	#[inline(always)]
	fn borrow(&self) -> &BigRational {
		&self.0
	}
}

impl Deref for Decimal {
	type Target = BigRational;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
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
			Ok(Self(value))
		} else {
			Err(NoDecimalRepresentation(value))
		}
	}
}

impl From<Decimal> for BigRational {
	#[inline(always)]
	fn from(value: Decimal) -> Self {
		value.0
	}
}

impl XsdDatatype for Decimal {
	#[inline(always)]
	fn type_(&self) -> Datatype {
		self.decimal_type().into()
	}
}
