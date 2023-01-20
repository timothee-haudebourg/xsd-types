use std::borrow::Borrow;
use std::ops::Deref;

use num_bigint::BigInt;
use num_traits::Zero;

use crate::{lexical, Datatype, NonPositiveIntegerDatatype, XsdDatatype};

pub struct NonPositiveInteger(BigInt);

impl NonPositiveInteger {
	#[inline(always)]
	fn non_positive_integer_type(&self) -> Option<NonPositiveIntegerDatatype> {
		if self.0 > BigInt::zero() {
			Some(NonPositiveIntegerDatatype::NegativeInteger)
		} else {
			None
		}
	}
}

impl XsdDatatype for NonPositiveInteger {
	#[inline(always)]
	fn type_(&self) -> Datatype {
		self.non_positive_integer_type().into()
	}
}

impl<'a> From<&'a lexical::NonPositiveInteger> for NonPositiveInteger {
	#[inline(always)]
	fn from(value: &'a lexical::NonPositiveInteger) -> Self {
		Self(value.as_str().parse().unwrap())
	}
}

impl From<lexical::NonPositiveIntegerBuf> for NonPositiveInteger {
	#[inline(always)]
	fn from(value: lexical::NonPositiveIntegerBuf) -> Self {
		value.as_non_positive_integer().into()
	}
}

impl AsRef<BigInt> for NonPositiveInteger {
	#[inline(always)]
	fn as_ref(&self) -> &BigInt {
		&self.0
	}
}

impl Borrow<BigInt> for NonPositiveInteger {
	#[inline(always)]
	fn borrow(&self) -> &BigInt {
		&self.0
	}
}

impl Deref for NonPositiveInteger {
	type Target = BigInt;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub struct NegativeInteger(BigInt);

impl XsdDatatype for NegativeInteger {
	fn type_(&self) -> Datatype {
		NonPositiveIntegerDatatype::NegativeInteger.into()
	}
}
