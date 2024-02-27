use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Div, Mul, Sub},
	str::FromStr,
};

use num_bigint::{BigInt, TryFromBigIntError};
use num_traits::{Signed, Zero};

use crate::{
	lexical::{self, LexicalFormOf},
	Datatype, IntDatatype, IntegerDatatype, LongDatatype, NonNegativeIntegerDatatype,
	NonPositiveIntegerDatatype, ParseXsd, ShortDatatype, UnsignedIntDatatype, UnsignedLongDatatype,
	UnsignedShortDatatype, XsdValue,
};

use super::{Sign, I16_MIN, I32_MIN, I64_MIN, I8_MIN, U16_MAX, U32_MAX, U64_MAX, U8_MAX};

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

	pub fn from_bytes_be(sign: Sign, bytes: &[u8]) -> Self {
		Self(BigInt::from_bytes_be(sign, bytes))
	}

	pub fn from_bytes_le(sign: Sign, bytes: &[u8]) -> Self {
		Self(BigInt::from_bytes_le(sign, bytes))
	}

	pub fn from_signed_bytes_be(bytes: &[u8]) -> Self {
		Self(BigInt::from_signed_bytes_be(bytes))
	}

	pub fn from_signed_bytes_le(bytes: &[u8]) -> Self {
		Self(BigInt::from_signed_bytes_le(bytes))
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

	pub fn integer_type(&self) -> IntegerDatatype {
		if self.0 >= BigInt::zero() {
			if self.0 > BigInt::zero() {
				if self.0 <= *U8_MAX {
					UnsignedShortDatatype::UnsignedByte.into()
				} else if self.0 <= *U16_MAX {
					UnsignedShortDatatype::UnsignedShort.into()
				} else if self.0 <= *U32_MAX {
					UnsignedIntDatatype::UnsignedInt.into()
				} else if self.0 <= *U64_MAX {
					UnsignedLongDatatype::UnsignedLong.into()
				} else {
					NonNegativeIntegerDatatype::PositiveInteger.into()
				}
			} else {
				UnsignedShortDatatype::UnsignedByte.into()
			}
		} else if self.0 >= *I8_MIN {
			ShortDatatype::Byte.into()
		} else if self.0 >= *I16_MIN {
			ShortDatatype::Short.into()
		} else if self.0 >= *I32_MIN {
			IntDatatype::Int.into()
		} else if self.0 >= *I64_MIN {
			LongDatatype::Long.into()
		} else {
			NonPositiveIntegerDatatype::NegativeInteger.into()
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

	pub fn to_bytes_be(&self) -> (Sign, Vec<u8>) {
		self.0.to_bytes_be()
	}

	pub fn to_bytes_le(&self) -> (Sign, Vec<u8>) {
		self.0.to_bytes_le()
	}

	pub fn to_signed_bytes_be(&self) -> Vec<u8> {
		self.0.to_signed_bytes_be()
	}

	pub fn to_signed_bytes_le(&self) -> Vec<u8> {
		self.0.to_signed_bytes_le()
	}
}

impl XsdValue for Integer {
	#[inline(always)]
	fn datatype(&self) -> Datatype {
		self.integer_type().into()
	}
}

impl ParseXsd for Integer {
	type LexicalForm = lexical::Integer;
}

impl LexicalFormOf<Integer> for lexical::Integer {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<Integer, Self::ValueError> {
		Ok(self.value())
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
	fn long_type(&self) -> LongDatatype;
}

impl XsdLong for Long {
	fn long_type(&self) -> LongDatatype {
		if (i8::MIN as i64..=i8::MAX as i64).contains(self) {
			ShortDatatype::Byte.into()
		} else if (i16::MIN as i64..=i16::MAX as i64).contains(self) {
			ShortDatatype::Short.into()
		} else if (i32::MIN as i64..=i32::MAX as i64).contains(self) {
			IntDatatype::Int.into()
		} else {
			LongDatatype::Long
		}
	}
}

impl XsdValue for Long {
	fn datatype(&self) -> Datatype {
		self.long_type().into()
	}
}

impl ParseXsd for Long {
	type LexicalForm = lexical::Integer;
}

impl LexicalFormOf<Long> for lexical::Integer {
	type ValueError = IntegerOutOfTargetBounds;

	fn try_as_value(&self) -> Result<Long, Self::ValueError> {
		self.value().try_into()
	}
}

pub type Int = i32;

pub trait XsdInt {
	fn int_type(&self) -> IntDatatype;
}

impl XsdInt for Int {
	fn int_type(&self) -> IntDatatype {
		if (i8::MIN as i32..=i8::MAX as i32).contains(self) {
			ShortDatatype::Byte.into()
		} else if (i16::MIN as i32..=i16::MAX as i32).contains(self) {
			ShortDatatype::Short.into()
		} else {
			IntDatatype::Int
		}
	}
}

impl XsdValue for Int {
	fn datatype(&self) -> Datatype {
		self.int_type().into()
	}
}

impl ParseXsd for Int {
	type LexicalForm = lexical::Integer;
}

impl LexicalFormOf<Int> for lexical::Integer {
	type ValueError = IntegerOutOfTargetBounds;

	fn try_as_value(&self) -> Result<Int, Self::ValueError> {
		self.value().try_into()
	}
}

pub type Short = i16;

pub trait XsdShort {
	fn short_type(&self) -> ShortDatatype;
}

impl XsdShort for Short {
	fn short_type(&self) -> ShortDatatype {
		if (i8::MIN as i16..=i8::MAX as i16).contains(self) {
			ShortDatatype::Byte
		} else {
			ShortDatatype::Short
		}
	}
}

impl XsdValue for Short {
	fn datatype(&self) -> Datatype {
		self.short_type().into()
	}
}

impl ParseXsd for Short {
	type LexicalForm = lexical::Integer;
}

impl LexicalFormOf<Short> for lexical::Integer {
	type ValueError = IntegerOutOfTargetBounds;

	fn try_as_value(&self) -> Result<Short, Self::ValueError> {
		self.value().try_into()
	}
}

pub type Byte = i8;

impl XsdValue for Byte {
	fn datatype(&self) -> Datatype {
		ShortDatatype::Byte.into()
	}
}

impl ParseXsd for Byte {
	type LexicalForm = lexical::Integer;
}

impl LexicalFormOf<Byte> for lexical::Integer {
	type ValueError = IntegerOutOfTargetBounds;

	fn try_as_value(&self) -> Result<Byte, Self::ValueError> {
		self.value().try_into()
	}
}

macro_rules! impl_integer_arithmetic {
	{
		for $target:ty where $id:ident ( $test:expr ) {
			$( $ty:ty $([$($accessor:tt)*])? ),*
		}
	} => {
		$(
			impl Add<$ty> for $target {
				type Output = Self;

				fn add(self, rhs: $ty) -> Self::Output {
					let $id = self.0 + rhs $($($accessor)*)?;

					if !($test) {
						panic!("attempt to add with overflow")
					}

					Self($id)
				}
			}

			impl Sub<$ty> for $target {
				type Output = Self;

				fn sub(self, rhs: $ty) -> Self::Output {
					let $id = self.0 - rhs $($($accessor)*)?;

					if !($test) {
						panic!("attempt to subtract with overflow")
					}

					Self($id)
				}
			}

			impl Mul<$ty> for $target {
				type Output = Self;

				fn mul(self, rhs: $ty) -> Self::Output {
					let $id = self.0 * rhs $($($accessor)*)?;

					if !($test) {
						panic!("attempt to multiply with overflow")
					}

					Self($id)
				}
			}

			impl Div<$ty> for $target {
				type Output = Self;

				fn div(self, rhs: $ty) -> Self::Output {
					let $id = self.0 / rhs $($($accessor)*)?;

					if !($test) {
						panic!("attempt to divide with overflow")
					}

					Self($id)
				}
			}
		)*
	};
}

pub(crate) use impl_integer_arithmetic;

impl_integer_arithmetic! {
	for Integer where r (true) {
		Integer [.0], i8, i16, i32, i64, isize, u8, u16, u32, u64, usize
	}
}
