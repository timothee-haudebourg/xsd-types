use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Div, Mul, Sub},
	str::FromStr,
};

use num_bigint::{BigInt, TryFromBigIntError};
use num_traits::{Signed, Zero};

use crate::{
	lexical, Datatype, IntDatatype, IntegerDatatype, LongDatatype, NonNegativeIntegerDatatype,
	NonPositiveIntegerDatatype, ShortDatatype, UnsignedIntDatatype, UnsignedLongDatatype,
	UnsignedShortDatatype, XsdDatatype,
};

use super::{I16_MIN, I32_MIN, I64_MIN, I8_MIN, U16_MAX, U32_MAX, U64_MAX, U8_MAX};

mod non_negative_integer;
mod non_positive_integer;

pub use non_negative_integer::*;
pub use non_positive_integer::*;

/// Integer number.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct Integer(BigInt);

impl Integer {
	/// Converts a `BigInt` reference into an `Integer` reference.
	#[inline(always)]
	pub fn from_bigint_ref(n: &BigInt) -> &Self {
		unsafe {
			// This is safe because `Integer` is a transparent wrapper around
			// `BigInt`.
			std::mem::transmute(n)
		}
	}

	#[inline(always)]
	pub fn zero() -> Self {
		Self(BigInt::zero())
	}

	#[inline(always)]
	pub fn is_zero(&self) -> bool {
		self.0.is_zero()
	}

	#[inline(always)]
	pub fn is_positive(&self) -> bool {
		self.0.is_positive()
	}

	#[inline(always)]
	pub fn is_negative(&self) -> bool {
		self.0.is_negative()
	}

	pub fn integer_type(&self) -> Option<IntegerDatatype> {
		if self.0 >= BigInt::zero() {
			if self.0 > BigInt::zero() {
				if self.0 <= *U8_MAX {
					Some(UnsignedShortDatatype::UnsignedByte.into())
				} else if self.0 <= *U16_MAX {
					Some(UnsignedIntDatatype::UnsignedShort(None).into())
				} else if self.0 <= *U32_MAX {
					Some(UnsignedLongDatatype::UnsignedInt(None).into())
				} else if self.0 <= *U64_MAX {
					Some(NonNegativeIntegerDatatype::UnsignedLong(None).into())
				} else {
					Some(NonNegativeIntegerDatatype::PositiveInteger.into())
				}
			} else {
				Some(UnsignedShortDatatype::UnsignedByte.into())
			}
		} else if self.0 >= *I8_MIN {
			Some(ShortDatatype::Byte.into())
		} else if self.0 >= *I16_MIN {
			Some(IntDatatype::Short(None).into())
		} else if self.0 >= *I32_MIN {
			Some(LongDatatype::Int(None).into())
		} else if self.0 >= *I64_MIN {
			Some(IntegerDatatype::Long(None))
		} else {
			Some(NonPositiveIntegerDatatype::NegativeInteger.into())
		}
	}

	/// Returns a lexical representation of this integer.
	#[inline(always)]
	pub fn lexical_representation(&self) -> lexical::IntegerBuf {
		unsafe {
			// This is safe because the `Display::fmt` method matches the
			// XSD lexical representation.
			lexical::IntegerBuf::new_unchecked(format!("{}", self))
		}
	}
}

impl XsdDatatype for Integer {
	#[inline(always)]
	fn type_(&self) -> Datatype {
		self.integer_type().into()
	}
}

impl From<BigInt> for Integer {
	#[inline(always)]
	fn from(value: BigInt) -> Self {
		Self(value)
	}
}

impl From<Integer> for BigInt {
	#[inline(always)]
	fn from(value: Integer) -> Self {
		value.0
	}
}

impl<'a> From<&'a lexical::Integer> for Integer {
	#[inline(always)]
	fn from(value: &'a lexical::Integer) -> Self {
		Self(value.as_str().parse().unwrap())
	}
}

impl From<lexical::IntegerBuf> for Integer {
	#[inline(always)]
	fn from(value: lexical::IntegerBuf) -> Self {
		value.as_integer().into()
	}
}

impl FromStr for Integer {
	type Err = lexical::InvalidInteger;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::Integer::new(s)?;
		Ok(l.into())
	}
}

impl From<lexical::NonPositiveIntegerBuf> for Integer {
	#[inline(always)]
	fn from(value: lexical::NonPositiveIntegerBuf) -> Self {
		value.as_integer().into()
	}
}

impl From<NonNegativeInteger> for Integer {
	fn from(value: NonNegativeInteger) -> Self {
		let n: BigInt = value.into();
		Self(n)
	}
}

impl fmt::Display for Integer {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl AsRef<BigInt> for Integer {
	#[inline(always)]
	fn as_ref(&self) -> &BigInt {
		&self.0
	}
}

impl Borrow<BigInt> for Integer {
	#[inline(always)]
	fn borrow(&self) -> &BigInt {
		&self.0
	}
}

#[derive(Debug, thiserror::Error)]
#[error("integer out of supported bounds: {0}")]
pub struct IntegerOutOfTargetBounds(pub Integer);

macro_rules! from {
	{ $( $ty:ty ),* } => {
		$(
			impl From<$ty> for Integer {
				fn from(value: $ty) -> Self {
					Self(value.into())
				}
			}
		)*
	};
}

from!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

macro_rules! try_into {
	{ $( $ty:ty ),* } => {
		$(
			impl TryFrom<Integer> for $ty {
				type Error = IntegerOutOfTargetBounds;

				fn try_from(value: Integer) -> Result<Self, Self::Error> {
					value.0.try_into().map_err(|e: TryFromBigIntError<BigInt>| IntegerOutOfTargetBounds(Integer(e.into_original())))
				}
			}
		)*
	};
}

try_into!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

pub type Long = i64;

pub trait XsdLong {
	fn long_type(&self) -> Option<LongDatatype>;
}

impl XsdLong for Long {
	fn long_type(&self) -> Option<LongDatatype> {
		if (i8::MIN as i64..=i8::MAX as i64).contains(self) {
			Some(ShortDatatype::Byte.into())
		} else if (i16::MIN as i64..=i16::MAX as i64).contains(self) {
			Some(IntDatatype::Short(None).into())
		} else if (i32::MIN as i64..=i32::MAX as i64).contains(self) {
			Some(LongDatatype::Int(None))
		} else {
			None
		}
	}
}

impl XsdDatatype for Long {
	fn type_(&self) -> Datatype {
		self.long_type().into()
	}
}

pub type Int = i32;

pub trait XsdInt {
	fn int_type(&self) -> Option<IntDatatype>;
}

impl XsdInt for Int {
	fn int_type(&self) -> Option<IntDatatype> {
		if (i8::MIN as i32..=i8::MAX as i32).contains(self) {
			Some(ShortDatatype::Byte.into())
		} else if (i16::MIN as i32..=i16::MAX as i32).contains(self) {
			Some(IntDatatype::Short(None))
		} else {
			None
		}
	}
}

impl XsdDatatype for Int {
	fn type_(&self) -> Datatype {
		self.int_type().into()
	}
}

pub type Short = i16;

pub trait XsdShort {
	fn short_type(&self) -> Option<ShortDatatype>;
}

impl XsdShort for Short {
	fn short_type(&self) -> Option<ShortDatatype> {
		if (i8::MIN as i16..=i8::MAX as i16).contains(self) {
			Some(ShortDatatype::Byte)
		} else {
			None
		}
	}
}

impl XsdDatatype for Short {
	fn type_(&self) -> Datatype {
		self.short_type().into()
	}
}

pub type Byte = i8;

impl XsdDatatype for Byte {
	fn type_(&self) -> Datatype {
		ShortDatatype::Byte.into()
	}
}

impl Add for Integer {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

impl Sub for Integer {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self(self.0 - rhs.0)
	}
}

impl Mul for Integer {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self(self.0 * rhs.0)
	}
}

impl Div for Integer {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self(self.0 / rhs.0)
	}
}

macro_rules! impl_arithmetic {
	{
		$( $ty:ty ),*
	} => {
		$(
			impl Add<$ty> for Integer {
				type Output = Self;

				fn add(self, rhs: $ty) -> Self::Output {
					Self(self.0 + rhs)
				}
			}

			impl Sub<$ty> for Integer {
				type Output = Self;

				fn sub(self, rhs: $ty) -> Self::Output {
					Self(self.0 - rhs)
				}
			}

			impl Mul<$ty> for Integer {
				type Output = Self;

				fn mul(self, rhs: $ty) -> Self::Output {
					Self(self.0 * rhs)
				}
			}

			impl Div<$ty> for Integer {
				type Output = Self;

				fn div(self, rhs: $ty) -> Self::Output {
					Self(self.0 / rhs)
				}
			}
		)*
	};
}

impl_arithmetic!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
