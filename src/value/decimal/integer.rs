use std::borrow::Borrow;
use std::ops::Deref;

use num_bigint::BigInt;
use num_traits::Zero;

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
pub struct Integer(BigInt);

impl Integer {
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

impl From<lexical::NonPositiveIntegerBuf> for Integer {
	#[inline(always)]
	fn from(value: lexical::NonPositiveIntegerBuf) -> Self {
		value.as_integer().into()
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

impl Deref for Integer {
	type Target = BigInt;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

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
