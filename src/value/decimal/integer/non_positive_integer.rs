use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Div, Mul, Sub},
	str::FromStr,
};

use num_bigint::{BigInt, TryFromBigIntError};
use num_traits::{Signed, Zero};

use crate::{
	impl_integer_arithmetic, lexical, Datatype, Integer, NonPositiveIntegerDatatype, XsdFromStr,
	XsdTyped, XsdValue,
};

use super::Sign;

/// Non positive integer number.
///
/// This is the value-space counterpart of [`lexical::NonPositiveInteger`].
/// Like its lexical form, `nonPositiveInteger` has no numbered grammar
/// production of its own in the XSD 1.1 Datatypes specification: it is
/// defined by restricting [`Integer`]'s value space with a `maxInclusive`
/// of 0.
/// See: <https://www.w3.org/TR/xmlschema11-2/#nonPositiveInteger>.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NonPositiveInteger(BigInt);

impl NonPositiveInteger {
	/// Create a new non positive integer from a `BigInt`.
	///
	/// # Safety
	///
	/// The input number must be non positive.
	pub unsafe fn new_unchecked(n: BigInt) -> Self {
		Self(n)
	}

	/// Creates a non positive integer from its unsigned big endian bytes
	/// representation.
	pub fn from_bytes_be(bytes: &[u8]) -> Self {
		Self(BigInt::from_bytes_be(Sign::Minus, bytes))
	}

	/// Creates a non positive integer from its unsigned little endian bytes
	/// representation.
	pub fn from_bytes_le(bytes: &[u8]) -> Self {
		Self(BigInt::from_bytes_le(Sign::Minus, bytes))
	}

	/// Creates a non positive integer from its signed big endian bytes
	/// representation.
	///
	/// # Safety
	///
	/// The represented number must be non positive.
	pub unsafe fn from_signed_bytes_be_unchecked(bytes: &[u8]) -> Self {
		Self(BigInt::from_signed_bytes_be(bytes))
	}

	/// Creates a non positive integer from its signed little endian bytes
	/// representation.
	///
	/// # Safety
	///
	/// The represented number must be non positive.
	pub unsafe fn from_signed_bytes_le_unchecked(bytes: &[u8]) -> Self {
		Self(BigInt::from_signed_bytes_le(bytes))
	}

	/// Creates a non positive integer from its signed big endian bytes
	/// representation, failing if the represented value is positive.
	pub fn from_signed_bytes_be(bytes: &[u8]) -> Result<Self, IntegerIsPositive> {
		Integer::from_signed_bytes_be(bytes).try_into()
	}

	/// Creates a non positive integer from its signed little endian bytes
	/// representation, failing if the represented value is positive.
	pub fn from_signed_bytes_le(bytes: &[u8]) -> Result<Self, IntegerIsPositive> {
		Integer::from_signed_bytes_le(bytes).try_into()
	}

	/// Converts this value into its underlying `BigInt`.
	#[inline(always)]
	pub fn into_big_int(self) -> BigInt {
		self.0
	}

	/// Returns the non positive integer `0`.
	#[inline(always)]
	pub fn zero() -> Self {
		Self(BigInt::zero())
	}

	/// Returns `true` if this value is zero.
	#[inline(always)]
	pub fn is_zero(&self) -> bool {
		self.0.is_zero()
	}

	#[inline(always)]
	fn non_positive_integer_type(&self) -> NonPositiveIntegerDatatype {
		if self.0 < BigInt::zero() {
			NonPositiveIntegerDatatype::NegativeInteger
		} else {
			NonPositiveIntegerDatatype::NonPositiveInteger
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

impl XsdTyped for NonPositiveInteger {
	#[inline(always)]
	fn datatype(&self) -> Datatype {
		self.non_positive_integer_type().into()
	}
}

impl XsdValue for NonPositiveInteger {
	type LexicalForm = lexical::NonPositiveInteger;

	fn to_lexical_form(&self) -> lexical::NonPositiveIntegerBuf {
		canonical_integer_lexical_form!(lexical::NonPositiveIntegerBuf, self)
	}
}

impl<'a> XsdFromStr<'a> for NonPositiveInteger {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::NonPositiveInteger, Self>(lexical_value)
	}
}

impl From<NonPositiveInteger> for BigInt {
	fn from(value: NonPositiveInteger) -> Self {
		value.0
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

impl FromStr for NonPositiveInteger {
	type Err = lexical::InvalidNonPositiveInteger<String>;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::NonPositiveInteger::new(s)
			.map_err(|_| lexical::InvalidNonPositiveInteger(s.to_owned()))?;
		Ok(l.into())
	}
}

impl fmt::Display for NonPositiveInteger {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for NonPositiveInteger {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for NonPositiveInteger {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = NonPositiveInteger;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#nonPositiveInteger")
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

impl_integer_arithmetic!(
	for NonPositiveInteger where r ( !r.is_positive() ) {
		Integer [.0],
		NonPositiveInteger [.0],
		NegativeInteger [.0],
		super::NonNegativeInteger [.into_big_int()],
		super::PositiveInteger [.into_big_int()],
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

/// Error raised when converting a [`NonPositiveInteger`] into a smaller
/// target integer type that cannot represent its value.
#[derive(Debug, thiserror::Error)]
#[error("integer out of supported bounds: {0}")]
pub struct NonPositiveIntegerOutOfTargetBounds(pub NonPositiveInteger);

macro_rules! try_into {
	{ $( $ty:ty ),* } => {
		$(
			impl TryFrom<NonPositiveInteger> for $ty {
				type Error = NonPositiveIntegerOutOfTargetBounds;

				fn try_from(value: NonPositiveInteger) -> Result<Self, Self::Error> {
					value.0.try_into().map_err(|e: TryFromBigIntError<BigInt>| NonPositiveIntegerOutOfTargetBounds(NonPositiveInteger(e.into_original())))
				}
			}
		)*
	};
}

try_into!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

/// Error raised when trying to convert a positive [`Integer`] into a
/// [`NonPositiveInteger`].
#[derive(Debug, thiserror::Error)]
#[error("integer {0} is positive")]
pub struct IntegerIsPositive(Integer);

impl TryFrom<Integer> for NonPositiveInteger {
	type Error = IntegerIsPositive;

	fn try_from(value: Integer) -> Result<Self, Self::Error> {
		if value.is_positive() {
			Err(IntegerIsPositive(value))
		} else {
			Ok(Self(value.into()))
		}
	}
}

/// Negative integer number.
///
/// This is the value-space counterpart of [`lexical::NegativeInteger`].
/// Like its lexical form, `negativeInteger` has no numbered grammar
/// production of its own in the XSD 1.1 Datatypes specification: it is
/// defined by restricting [`NonPositiveInteger`]'s value space with a
/// `maxInclusive` of -1.
/// See: <https://www.w3.org/TR/xmlschema11-2/#negativeInteger>.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NegativeInteger(BigInt);

impl NegativeInteger {
	/// Creates a new negative integer from the given `BigInt`.
	///
	/// # Safety
	///
	/// The input value *must* but a negative integer.
	pub unsafe fn new_unchecked(n: BigInt) -> Self {
		Self(n)
	}

	/// Creates a negative integer from its unsigned big endian bytes
	/// representation.
	///
	/// # Safety
	///
	/// The represented number must be negative.
	pub unsafe fn from_bytes_be_unchecked(bytes: &[u8]) -> Self {
		Self(BigInt::from_bytes_be(Sign::Minus, bytes))
	}

	/// Creates a negative integer from its unsigned little endian bytes
	/// representation.
	///
	/// # Safety
	///
	/// The represented number must be negative.
	pub unsafe fn from_bytes_le_unchecked(bytes: &[u8]) -> Self {
		Self(BigInt::from_bytes_le(Sign::Minus, bytes))
	}

	/// Creates a negative integer from its signed big endian bytes
	/// representation.
	///
	/// # Safety
	///
	/// The represented number must be negative.
	pub unsafe fn from_signed_bytes_be_unchecked(bytes: &[u8]) -> Self {
		Self(BigInt::from_signed_bytes_be(bytes))
	}

	/// Creates a negative integer from its signed little endian bytes
	/// representation.
	///
	/// # Safety
	///
	/// The represented number must be negative.
	pub unsafe fn from_signed_bytes_le_unchecked(bytes: &[u8]) -> Self {
		Self(BigInt::from_signed_bytes_le(bytes))
	}

	/// Converts this value into its underlying `BigInt`.
	pub fn into_big_int(self) -> BigInt {
		self.0
	}

	/// Returns `true` if this value is `-1`.
	pub fn is_minus_one(&self) -> bool {
		matches!(i8::try_from(&self.0), Ok(-1))
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

impl XsdTyped for NegativeInteger {
	fn datatype(&self) -> Datatype {
		NonPositiveIntegerDatatype::NegativeInteger.into()
	}
}

impl XsdValue for NegativeInteger {
	type LexicalForm = lexical::NegativeInteger;

	fn to_lexical_form(&self) -> lexical::NegativeIntegerBuf {
		canonical_integer_lexical_form!(lexical::NegativeIntegerBuf, self)
	}
}

impl<'a> XsdFromStr<'a> for NegativeInteger {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::NegativeInteger, Self>(lexical_value)
	}
}

impl<'a> From<&'a lexical::NegativeInteger> for NegativeInteger {
	fn from(value: &'a lexical::NegativeInteger) -> Self {
		value.value()
	}
}

impl FromStr for NegativeInteger {
	type Err = lexical::InvalidNegativeInteger<String>;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::NegativeInteger::new(s)
			.map_err(|_| lexical::InvalidNegativeInteger(s.to_owned()))?;
		Ok(l.into())
	}
}

impl fmt::Display for NegativeInteger {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for NegativeInteger {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for NegativeInteger {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = NegativeInteger;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#negativeInteger")
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

impl From<NegativeInteger> for BigInt {
	fn from(value: NegativeInteger) -> Self {
		value.into_big_int()
	}
}

impl_integer_arithmetic!(
	for NegativeInteger where r ( r.is_negative() ) {
		Integer [.0],
		NonPositiveInteger [.0],
		NegativeInteger [.0],
		super::NonNegativeInteger [.into_big_int()],
		super::PositiveInteger [.into_big_int()],
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
	fn non_positive_integer_to_lexical_form() {
		for s in ["0", "-1", "-123456789012345678901234567890123456789"] {
			check!(NonPositiveInteger, s);
		}

		let value: NonPositiveInteger = "-0".parse().unwrap();
		assert_eq!(value.to_lexical_form().as_str(), "0");
	}

	#[test]
	fn negative_integer_to_lexical_form() {
		for s in ["-1", "-42", "-123456789012345678901234567890123456789"] {
			check!(NegativeInteger, s);
		}
	}
}
