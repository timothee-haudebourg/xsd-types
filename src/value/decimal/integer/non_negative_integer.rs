use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Div, Mul, Sub},
	str::FromStr,
};

use num_bigint::{BigInt, TryFromBigIntError};
use num_traits::{Signed, Zero};

use crate::{
	impl_integer_arithmetic, lexical,
	value::decimal::{U16_MAX, U32_MAX, U64_MAX, U8_MAX},
	Datatype, NonNegativeIntegerDatatype, UnsignedIntDatatype, UnsignedLongDatatype,
	UnsignedShortDatatype, XsdFromStr, XsdInteger, XsdTyped, XsdValue,
};

use super::Sign;

/// Non negative integer number.
///
/// This is the value-space counterpart of [`lexical::XsdNonNegativeIntegerStr`].
/// Like its lexical form, `nonNegativeInteger` has no numbered grammar
/// production of its own in the XSD 1.1 Datatypes specification: it is
/// defined by restricting [`XsdInteger`]'s value space with a `minInclusive`
/// of 0.
/// See: <https://www.w3.org/TR/xmlschema11-2/#nonNegativeInteger>.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct XsdNonNegativeInteger(BigInt);

impl XsdNonNegativeInteger {
	/// Create a new non negative integer from a `BigInt`.
	///
	/// # Safety
	///
	/// The input number must be non negative.
	pub unsafe fn new_unchecked(n: BigInt) -> Self {
		Self(n)
	}

	/// Creates a non negative integer from its unsigned big endian bytes
	/// representation.
	pub fn from_bytes_be(bytes: &[u8]) -> Self {
		Self(BigInt::from_bytes_be(Sign::Plus, bytes))
	}

	/// Creates a non negative integer from its unsigned little endian bytes
	/// representation.
	pub fn from_bytes_le(bytes: &[u8]) -> Self {
		Self(BigInt::from_bytes_le(Sign::Plus, bytes))
	}

	/// Creates a non negative integer from its signed big endian bytes
	/// representation.
	///
	/// # Safety
	///
	/// The represented number must be non negative.
	pub unsafe fn from_signed_bytes_be_unchecked(bytes: &[u8]) -> Self {
		Self(BigInt::from_signed_bytes_be(bytes))
	}

	/// Creates a non negative integer from its signed little endian bytes
	/// representation.
	///
	/// # Safety
	///
	/// The represented number must be non negative.
	pub unsafe fn from_signed_bytes_le_unchecked(bytes: &[u8]) -> Self {
		Self(BigInt::from_signed_bytes_le(bytes))
	}

	/// Creates a non negative integer from its signed big endian bytes
	/// representation, failing if the represented value is negative.
	pub fn from_signed_bytes_be(bytes: &[u8]) -> Result<Self, IntegerIsNegative> {
		XsdInteger::from_signed_bytes_be(bytes).try_into()
	}

	/// Creates a non negative integer from its signed little endian bytes
	/// representation, failing if the represented value is negative.
	pub fn from_signed_bytes_le(bytes: &[u8]) -> Result<Self, IntegerIsNegative> {
		XsdInteger::from_signed_bytes_le(bytes).try_into()
	}

	/// Converts this value into its underlying `BigInt`.
	#[inline(always)]
	pub fn into_big_int(self) -> BigInt {
		self.0
	}

	/// Returns the non negative integer `0`.
	#[inline(always)]
	pub fn zero() -> Self {
		Self(BigInt::zero())
	}

	/// Returns `true` if this value is zero.
	#[inline(always)]
	pub fn is_zero(&self) -> bool {
		self.0.is_zero()
	}

	/// Returns the most specific XSD non-negative-integer-derived datatype
	/// that can represent this value (e.g. `unsignedByte`, `unsignedInt`,
	/// `positiveInteger`, ...).
	pub fn non_negative_integer_type(&self) -> NonNegativeIntegerDatatype {
		if self.0 <= *U8_MAX {
			UnsignedShortDatatype::UnsignedByte.into()
		} else if self.0 <= *U16_MAX {
			UnsignedShortDatatype::UnsignedShort.into()
		} else if self.0 <= *U32_MAX {
			UnsignedIntDatatype::UnsignedInt.into()
		} else if self.0 <= *U64_MAX {
			UnsignedLongDatatype::UnsignedLong.into()
		} else {
			NonNegativeIntegerDatatype::PositiveInteger
		}
	}

	/// Returns the sign and unsigned big-endian bytes representation of this
	/// value.
	pub fn to_bytes_be(&self) -> (Sign, Vec<u8>) {
		self.0.to_bytes_be()
	}

	/// Returns the sign and unsigned little-endian bytes representation of
	/// this value.
	pub fn to_bytes_le(&self) -> (Sign, Vec<u8>) {
		self.0.to_bytes_le()
	}

	/// Returns the two's-complement big-endian bytes representation of this
	/// value.
	pub fn to_signed_bytes_be(&self) -> Vec<u8> {
		self.0.to_signed_bytes_be()
	}

	/// Returns the two's-complement little-endian bytes representation of
	/// this value.
	pub fn to_signed_bytes_le(&self) -> Vec<u8> {
		self.0.to_signed_bytes_le()
	}
}

impl XsdTyped for XsdNonNegativeInteger {
	fn datatype(&self) -> Datatype {
		self.non_negative_integer_type().into()
	}
}

impl XsdValue for XsdNonNegativeInteger {
	type LexicalForm = lexical::XsdNonNegativeIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdNonNegativeIntegerString {
		canonical_integer_lexical_form!(lexical::XsdNonNegativeIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdNonNegativeInteger {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdNonNegativeIntegerStr, Self>(lexical_value)
	}
}

impl From<XsdNonNegativeInteger> for BigInt {
	fn from(value: XsdNonNegativeInteger) -> Self {
		value.0
	}
}

impl<'a> From<&'a lexical::XsdNonNegativeIntegerStr> for XsdNonNegativeInteger {
	#[inline(always)]
	fn from(value: &'a lexical::XsdNonNegativeIntegerStr) -> Self {
		Self(value.as_str().parse().unwrap())
	}
}

impl From<lexical::XsdNonNegativeIntegerString> for XsdNonNegativeInteger {
	#[inline(always)]
	fn from(value: lexical::XsdNonNegativeIntegerString) -> Self {
		value.as_xsd_non_negative_integer_str().into()
	}
}

impl FromStr for XsdNonNegativeInteger {
	type Err = lexical::InvalidXsdNonNegativeIntegerStr<String>;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::XsdNonNegativeIntegerStr::new(s)
			.map_err(|_| lexical::InvalidXsdNonNegativeIntegerStr(s.to_owned()))?;
		Ok(l.into())
	}
}

impl fmt::Display for XsdNonNegativeInteger {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdNonNegativeInteger {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdNonNegativeInteger {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdNonNegativeInteger;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#nonNegativeInteger")
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

impl AsRef<BigInt> for XsdNonNegativeInteger {
	#[inline(always)]
	fn as_ref(&self) -> &BigInt {
		&self.0
	}
}

impl Borrow<BigInt> for XsdNonNegativeInteger {
	#[inline(always)]
	fn borrow(&self) -> &BigInt {
		&self.0
	}
}

/// Error raised when trying to convert a negative [`XsdInteger`] into a
/// [`XsdNonNegativeInteger`].
#[derive(Debug, thiserror::Error)]
#[error("integer {0} is negative")]
pub struct IntegerIsNegative(XsdInteger);

impl TryFrom<XsdInteger> for XsdNonNegativeInteger {
	type Error = IntegerIsNegative;

	fn try_from(value: XsdInteger) -> Result<Self, Self::Error> {
		if value.is_negative() {
			Err(IntegerIsNegative(value))
		} else {
			Ok(Self(value.into()))
		}
	}
}

macro_rules! from {
	{ $( $ty:ty ),* } => {
		$(
			impl From<$ty> for XsdNonNegativeInteger {
				fn from(value: $ty) -> Self {
					Self(value.into())
				}
			}
		)*
	};
}

from!(u8, u16, u32, u64, usize);

/// Error raised when converting a [`XsdNonNegativeInteger`] into a smaller
/// target integer type that cannot represent its value.
#[derive(Debug, thiserror::Error)]
#[error("integer out of supported bounds: {0}")]
pub struct NonNegativeIntegerOutOfTargetBounds(pub XsdNonNegativeInteger);

macro_rules! try_into {
	{ $( $ty:ty ),* } => {
		$(
			impl TryFrom<XsdNonNegativeInteger> for $ty {
				type Error = NonNegativeIntegerOutOfTargetBounds;

				fn try_from(value: XsdNonNegativeInteger) -> Result<Self, Self::Error> {
					value.0.try_into().map_err(|e: TryFromBigIntError<BigInt>| NonNegativeIntegerOutOfTargetBounds(XsdNonNegativeInteger(e.into_original())))
				}
			}
		)*
	};
}

try_into!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl_integer_arithmetic!(
	for XsdNonNegativeInteger where r ( !r.is_negative() ) {
		XsdInteger [.0],
		XsdNonNegativeInteger [.0],
		XsdPositiveInteger [.0],
		super::XsdNonPositiveInteger [.into_big_int()],
		super::XsdNegativeInteger [.into_big_int()],
		i8,
		i16,
		i32,
		i64,
		isize,
		u8,
		u16,
		u32,
		u64,
		usize
	}
);

pub type XsdUnsignedLong = u64;

pub trait XsdUnsignedLongSubtype {
	fn unsigned_long_type(&self) -> UnsignedLongDatatype;
}

impl XsdUnsignedLongSubtype for XsdUnsignedLong {
	fn unsigned_long_type(&self) -> UnsignedLongDatatype {
		if *self <= u8::MAX as u64 {
			UnsignedShortDatatype::UnsignedByte.into()
		} else if *self <= u16::MAX as u64 {
			UnsignedShortDatatype::UnsignedShort.into()
		} else if *self <= u32::MAX as u64 {
			UnsignedIntDatatype::UnsignedInt.into()
		} else {
			UnsignedLongDatatype::UnsignedLong
		}
	}
}

impl XsdTyped for XsdUnsignedLong {
	fn datatype(&self) -> Datatype {
		self.unsigned_long_type().into()
	}
}

impl XsdValue for XsdUnsignedLong {
	type LexicalForm = lexical::XsdNonNegativeIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdNonNegativeIntegerString {
		canonical_integer_lexical_form!(lexical::XsdNonNegativeIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdUnsignedLong {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdNonNegativeIntegerStr, Self>(lexical_value)
	}
}

pub type XsdUnsignedInt = u32;

pub trait XsdUnsignedIntSubtype {
	fn unsigned_int_type(&self) -> UnsignedIntDatatype;
}

impl XsdUnsignedIntSubtype for XsdUnsignedInt {
	fn unsigned_int_type(&self) -> UnsignedIntDatatype {
		if *self <= u8::MAX as u32 {
			UnsignedShortDatatype::UnsignedByte.into()
		} else if *self <= u16::MAX as u32 {
			UnsignedShortDatatype::UnsignedShort.into()
		} else {
			UnsignedIntDatatype::UnsignedInt
		}
	}
}

impl XsdTyped for XsdUnsignedInt {
	fn datatype(&self) -> Datatype {
		self.unsigned_int_type().into()
	}
}

impl XsdValue for XsdUnsignedInt {
	type LexicalForm = lexical::XsdNonNegativeIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdNonNegativeIntegerString {
		canonical_integer_lexical_form!(lexical::XsdNonNegativeIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdUnsignedInt {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdNonNegativeIntegerStr, Self>(lexical_value)
	}
}

pub type XsdUnsignedShort = u16;

pub trait XsdUnsignedShortSubtype {
	fn unsigned_short_type(&self) -> UnsignedShortDatatype;
}

impl XsdUnsignedShortSubtype for XsdUnsignedShort {
	fn unsigned_short_type(&self) -> UnsignedShortDatatype {
		if *self <= u8::MAX as u16 {
			UnsignedShortDatatype::UnsignedByte
		} else {
			UnsignedShortDatatype::UnsignedShort
		}
	}
}

impl XsdTyped for XsdUnsignedShort {
	fn datatype(&self) -> Datatype {
		self.unsigned_short_type().into()
	}
}

impl XsdValue for XsdUnsignedShort {
	type LexicalForm = lexical::XsdNonNegativeIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdNonNegativeIntegerString {
		canonical_integer_lexical_form!(lexical::XsdNonNegativeIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdUnsignedShort {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdNonNegativeIntegerStr, Self>(lexical_value)
	}
}

pub type XsdUnsignedByte = u8;

impl XsdTyped for XsdUnsignedByte {
	fn datatype(&self) -> Datatype {
		UnsignedShortDatatype::UnsignedByte.into()
	}
}

impl XsdValue for XsdUnsignedByte {
	type LexicalForm = lexical::XsdNonNegativeIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdNonNegativeIntegerString {
		canonical_integer_lexical_form!(lexical::XsdNonNegativeIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdUnsignedByte {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdNonNegativeIntegerStr, Self>(lexical_value)
	}
}

/// Positive integer number.
///
/// This is the value-space counterpart of [`lexical::XsdPositiveIntegerStr`].
/// Like its lexical form, `positiveInteger` has no numbered grammar
/// production of its own in the XSD 1.1 Datatypes specification: it is
/// defined by restricting [`XsdNonNegativeInteger`]'s value space with a
/// `minInclusive` of 1.
/// See: <https://www.w3.org/TR/xmlschema11-2/#positiveInteger>.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct XsdPositiveInteger(BigInt);

impl XsdPositiveInteger {
	/// Creates a new positive integer from the given `BigInt`.
	///
	/// # Safety
	///
	/// The input value *must* but a positive integer.
	pub unsafe fn new_unchecked(n: BigInt) -> Self {
		Self(n)
	}

	/// Creates a positive integer from its unsigned big endian bytes
	/// representation.
	pub fn from_bytes_be(bytes: &[u8]) -> Self {
		Self(BigInt::from_bytes_be(Sign::Plus, bytes))
	}

	/// Creates a positive integer from its unsigned little endian bytes
	/// representation.
	pub fn from_bytes_le(bytes: &[u8]) -> Self {
		Self(BigInt::from_bytes_le(Sign::Plus, bytes))
	}

	/// Creates a positive integer from its unsigned big endian bytes
	/// representation.
	///
	/// # Safety
	///
	/// The represented number must be positive.
	pub unsafe fn from_signed_bytes_be_unchecked(bytes: &[u8]) -> Self {
		Self(BigInt::from_signed_bytes_be(bytes))
	}

	/// Creates a positive integer from its unsigned little endian bytes
	/// representation.
	///
	/// # Safety
	///
	/// The represented number must be positive.
	pub unsafe fn from_signed_bytes_le_unchecked(bytes: &[u8]) -> Self {
		Self(BigInt::from_signed_bytes_le(bytes))
	}

	/// Converts this value into its underlying `BigInt`.
	pub fn into_big_int(self) -> BigInt {
		self.0
	}

	/// Returns `true` if this value is `1`.
	pub fn is_one(&self) -> bool {
		matches!(u8::try_from(&self.0), Ok(1))
	}

	/// Returns the sign and unsigned big-endian bytes representation of this
	/// value.
	pub fn to_bytes_be(&self) -> (Sign, Vec<u8>) {
		self.0.to_bytes_be()
	}

	/// Returns the sign and unsigned little-endian bytes representation of
	/// this value.
	pub fn to_bytes_le(&self) -> (Sign, Vec<u8>) {
		self.0.to_bytes_le()
	}

	/// Returns the two's-complement big-endian bytes representation of this
	/// value.
	pub fn to_signed_bytes_be(&self) -> Vec<u8> {
		self.0.to_signed_bytes_be()
	}

	/// Returns the two's-complement little-endian bytes representation of
	/// this value.
	pub fn to_signed_bytes_le(&self) -> Vec<u8> {
		self.0.to_signed_bytes_le()
	}
}

impl XsdTyped for XsdPositiveInteger {
	fn datatype(&self) -> Datatype {
		NonNegativeIntegerDatatype::PositiveInteger.into()
	}
}

impl XsdValue for XsdPositiveInteger {
	type LexicalForm = lexical::XsdPositiveIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdPositiveIntegerString {
		canonical_integer_lexical_form!(lexical::XsdPositiveIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdPositiveInteger {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdPositiveIntegerStr, Self>(lexical_value)
	}
}

impl<'a> From<&'a lexical::XsdPositiveIntegerStr> for XsdPositiveInteger {
	fn from(value: &'a lexical::XsdPositiveIntegerStr) -> Self {
		value.value()
	}
}

impl FromStr for XsdPositiveInteger {
	type Err = lexical::InvalidXsdPositiveIntegerStr<String>;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::XsdPositiveIntegerStr::new(s)
			.map_err(|_| lexical::InvalidXsdPositiveIntegerStr(s.to_owned()))?;
		Ok(l.into())
	}
}

impl fmt::Display for XsdPositiveInteger {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdPositiveInteger {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdPositiveInteger {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdPositiveInteger;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#positiveInteger")
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

impl From<XsdPositiveInteger> for BigInt {
	fn from(value: XsdPositiveInteger) -> Self {
		value.into_big_int()
	}
}

impl_integer_arithmetic!(
	for XsdPositiveInteger where r ( r.is_positive() ) {
		XsdInteger [.0],
		XsdNonNegativeInteger [.0],
		XsdPositiveInteger [.0],
		super::XsdNonPositiveInteger [.into_big_int()],
		super::XsdNegativeInteger [.into_big_int()],
		i8,
		i16,
		i32,
		i64,
		isize,
		u8,
		u16,
		u32,
		u64,
		usize
	}
);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn zero_is_unsigned_byte() {
		assert_eq!(
			XsdNonNegativeInteger::zero().non_negative_integer_type(),
			UnsignedShortDatatype::UnsignedByte.into()
		);

		assert_eq!(
			XsdInteger::zero().integer_type(),
			UnsignedShortDatatype::UnsignedByte.into()
		);
	}

	/// Parses `canonical`, checks that `to_lexical_form` gives it back and that
	/// the result parses to an equal value.
	macro_rules! check {
		($ty:ty, $canonical:expr) => {{
			let canonical: &str = $canonical;
			let value = <$ty as crate::XsdFromStr>::xsd_from_str(canonical).unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), canonical);
			assert_eq!(
				<$ty as crate::XsdFromStr>::xsd_from_str(lexical.as_str()).unwrap(),
				value
			);
		}};
	}

	#[test]
	fn non_negative_integer_to_lexical_form() {
		for s in ["0", "7", "123456789012345678901234567890123456789"] {
			check!(XsdNonNegativeInteger, s);
		}

		let value: XsdNonNegativeInteger = "+007".parse().unwrap();
		assert_eq!(value.to_lexical_form().as_str(), "7");
	}

	#[test]
	fn unsigned_long_to_lexical_form() {
		for s in ["0", "1", "18446744073709551615"] {
			check!(XsdUnsignedLong, s);
		}
	}

	#[test]
	fn unsigned_int_to_lexical_form() {
		for s in ["0", "1", "4294967295"] {
			check!(XsdUnsignedInt, s);
		}
	}

	#[test]
	fn unsigned_short_to_lexical_form() {
		for s in ["0", "1", "65535"] {
			check!(XsdUnsignedShort, s);
		}
	}

	#[test]
	fn unsigned_byte_to_lexical_form() {
		for s in ["0", "1", "255"] {
			check!(XsdUnsignedByte, s);
		}
	}

	#[test]
	fn positive_integer_to_lexical_form() {
		for s in ["1", "42", "123456789012345678901234567890123456789"] {
			check!(XsdPositiveInteger, s);
		}

		let value: XsdPositiveInteger = "+007".parse().unwrap();
		assert_eq!(value.to_lexical_form().as_str(), "7");
	}
}
