use std::{borrow::Borrow, ops::Deref};

use num_bigint::BigInt;
use num_traits::Zero;

use crate::{
	lexical,
	value::decimal::{U16_MAX, U32_MAX, U64_MAX, U8_MAX},
	Datatype, NonNegativeIntegerDatatype, UnsignedIntDatatype, UnsignedLongDatatype,
	UnsignedShortDatatype, XsdDatatype,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NonNegativeInteger(BigInt);

impl NonNegativeInteger {
	pub fn non_negative_integer_type(&self) -> Option<NonNegativeIntegerDatatype> {
		if self.0 > BigInt::zero() {
			if self.0 <= *U8_MAX {
				Some(UnsignedShortDatatype::UnsignedByte.into())
			} else if self.0 <= *U16_MAX {
				Some(UnsignedIntDatatype::UnsignedShort(None).into())
			} else if self.0 <= *U32_MAX {
				Some(UnsignedLongDatatype::UnsignedInt(None).into())
			} else if self.0 <= *U64_MAX {
				Some(NonNegativeIntegerDatatype::UnsignedLong(None))
			} else {
				Some(NonNegativeIntegerDatatype::PositiveInteger)
			}
		} else {
			Some(UnsignedShortDatatype::UnsignedByte.into())
		}
	}
}

impl XsdDatatype for NonNegativeInteger {
	fn type_(&self) -> Datatype {
		self.non_negative_integer_type().into()
	}
}

impl<'a> From<&'a lexical::NonNegativeInteger> for NonNegativeInteger {
	#[inline(always)]
	fn from(value: &'a lexical::NonNegativeInteger) -> Self {
		Self(value.as_str().parse().unwrap())
	}
}

impl From<lexical::NonNegativeIntegerBuf> for NonNegativeInteger {
	#[inline(always)]
	fn from(value: lexical::NonNegativeIntegerBuf) -> Self {
		value.as_non_negative_integer().into()
	}
}

impl AsRef<BigInt> for NonNegativeInteger {
	#[inline(always)]
	fn as_ref(&self) -> &BigInt {
		&self.0
	}
}

impl Borrow<BigInt> for NonNegativeInteger {
	#[inline(always)]
	fn borrow(&self) -> &BigInt {
		&self.0
	}
}

impl Deref for NonNegativeInteger {
	type Target = BigInt;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub type UnsignedLong = u64;

pub trait XsdUnsignedLong {
	fn unsigned_long_type(&self) -> Option<UnsignedLongDatatype>;
}

impl XsdUnsignedLong for UnsignedLong {
	fn unsigned_long_type(&self) -> Option<UnsignedLongDatatype> {
		if *self <= u8::MAX as u64 {
			Some(UnsignedShortDatatype::UnsignedByte.into())
		} else if *self <= u16::MAX as u64 {
			Some(UnsignedIntDatatype::UnsignedShort(None).into())
		} else if *self <= u32::MAX as u64 {
			Some(UnsignedLongDatatype::UnsignedInt(None))
		} else {
			None
		}
	}
}

impl XsdDatatype for UnsignedLong {
	fn type_(&self) -> Datatype {
		self.unsigned_long_type().into()
	}
}

pub type UnsignedInt = u32;

pub trait XsdUnsignedInt {
	fn unsigned_int_type(&self) -> Option<UnsignedIntDatatype>;
}

impl XsdUnsignedInt for UnsignedInt {
	fn unsigned_int_type(&self) -> Option<UnsignedIntDatatype> {
		if *self <= u8::MAX as u32 {
			Some(UnsignedShortDatatype::UnsignedByte.into())
		} else if *self <= u16::MAX as u32 {
			Some(UnsignedIntDatatype::UnsignedShort(None))
		} else {
			None
		}
	}
}

impl XsdDatatype for UnsignedInt {
	fn type_(&self) -> Datatype {
		self.unsigned_int_type().into()
	}
}

pub type UnsignedShort = u16;

pub trait XsdUnsignedShort {
	fn unsigned_short_type(&self) -> Option<UnsignedShortDatatype>;
}

impl XsdUnsignedShort for UnsignedShort {
	fn unsigned_short_type(&self) -> Option<UnsignedShortDatatype> {
		if *self <= u8::MAX as u16 {
			Some(UnsignedShortDatatype::UnsignedByte)
		} else {
			None
		}
	}
}

impl XsdDatatype for UnsignedShort {
	fn type_(&self) -> Datatype {
		self.unsigned_short_type().into()
	}
}

pub type UnsignedByte = u8;

impl XsdDatatype for UnsignedByte {
	fn type_(&self) -> Datatype {
		UnsignedShortDatatype::UnsignedByte.into()
	}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PositiveInteger(BigInt);

impl XsdDatatype for PositiveInteger {
	fn type_(&self) -> Datatype {
		NonNegativeIntegerDatatype::PositiveInteger.into()
	}
}
