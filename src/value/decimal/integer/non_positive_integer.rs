use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Div, Mul, Sub},
	str::FromStr,
};

use num_bigint::{BigInt, TryFromBigIntError};
use num_traits::{Signed, Zero};

use crate::{
	XsdDatatype, XsdFromStr, XsdInteger, XsdNonPositiveIntegerDatatype, XsdTyped, XsdValueDomain,
	impl_integer_arithmetic, lexical,
};

use super::Sign;

/// Non positive integer number.
///
/// This is the value-space counterpart of [`lexical::XsdNonPositiveIntegerStr`].
/// Like its lexical form, `nonPositiveInteger` has no numbered grammar
/// production of its own in the XSD 1.1 Datatypes specification: it is
/// defined by restricting [`XsdInteger`]'s value space with a `maxInclusive`
/// of 0.
/// See: <https://www.w3.org/TR/xmlschema11-2/#nonPositiveInteger>.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct XsdNonPositiveInteger(BigInt);

impl XsdNonPositiveInteger {
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
		XsdInteger::from_signed_bytes_be(bytes).try_into()
	}

	/// Creates a non positive integer from its signed little endian bytes
	/// representation, failing if the represented value is positive.
	pub fn from_signed_bytes_le(bytes: &[u8]) -> Result<Self, IntegerIsPositive> {
		XsdInteger::from_signed_bytes_le(bytes).try_into()
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
	fn non_positive_integer_type(&self) -> XsdNonPositiveIntegerDatatype {
		if self.0 < BigInt::zero() {
			XsdNonPositiveIntegerDatatype::NegativeInteger
		} else {
			XsdNonPositiveIntegerDatatype::NonPositiveInteger
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

impl XsdTyped for XsdNonPositiveInteger {
	#[inline(always)]
	fn datatype(&self) -> XsdDatatype {
		self.non_positive_integer_type().into()
	}
}

impl XsdValueDomain for XsdNonPositiveInteger {
	type LexicalForm = lexical::XsdNonPositiveIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdNonPositiveIntegerString {
		canonical_integer_lexical_form!(lexical::XsdNonPositiveIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdNonPositiveInteger {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdNonPositiveIntegerStr, Self>(lexical_value)
	}
}

impl From<XsdNonPositiveInteger> for BigInt {
	fn from(value: XsdNonPositiveInteger) -> Self {
		value.0
	}
}

impl<'a> From<&'a lexical::XsdNonPositiveIntegerStr> for XsdNonPositiveInteger {
	#[inline(always)]
	fn from(value: &'a lexical::XsdNonPositiveIntegerStr) -> Self {
		Self(value.as_str().parse().unwrap())
	}
}

impl From<lexical::XsdNonPositiveIntegerString> for XsdNonPositiveInteger {
	#[inline(always)]
	fn from(value: lexical::XsdNonPositiveIntegerString) -> Self {
		value.as_xsd_non_positive_integer_str().into()
	}
}

impl FromStr for XsdNonPositiveInteger {
	type Err = lexical::InvalidXsdNonPositiveIntegerStr<String>;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::XsdNonPositiveIntegerStr::new(s)
			.map_err(|_| lexical::InvalidXsdNonPositiveIntegerStr(s.to_owned()))?;
		Ok(l.into())
	}
}

impl fmt::Display for XsdNonPositiveInteger {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdNonPositiveInteger {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdNonPositiveInteger {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdNonPositiveInteger;

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

impl AsRef<BigInt> for XsdNonPositiveInteger {
	#[inline(always)]
	fn as_ref(&self) -> &BigInt {
		&self.0
	}
}

impl Borrow<BigInt> for XsdNonPositiveInteger {
	#[inline(always)]
	fn borrow(&self) -> &BigInt {
		&self.0
	}
}

impl_integer_arithmetic!(
	for XsdNonPositiveInteger where r ( !r.is_positive() ) {
		XsdInteger [.0],
		XsdNonPositiveInteger [.0],
		XsdNegativeInteger [.0],
		super::XsdNonNegativeInteger [.into_big_int()],
		super::XsdPositiveInteger [.into_big_int()],
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

/// Error raised when converting a [`XsdNonPositiveInteger`] into a smaller
/// target integer type that cannot represent its value.
#[derive(Debug, thiserror::Error)]
#[error("integer out of supported bounds: {0}")]
pub struct NonPositiveIntegerOutOfTargetBounds(pub XsdNonPositiveInteger);

macro_rules! try_into {
	{ $( $ty:ty ),* } => {
		$(
			impl TryFrom<XsdNonPositiveInteger> for $ty {
				type Error = NonPositiveIntegerOutOfTargetBounds;

				fn try_from(value: XsdNonPositiveInteger) -> Result<Self, Self::Error> {
					value.0.try_into().map_err(|e: TryFromBigIntError<BigInt>| NonPositiveIntegerOutOfTargetBounds(XsdNonPositiveInteger(e.into_original())))
				}
			}
		)*
	};
}

try_into!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

/// Error raised when trying to convert a positive [`XsdInteger`] into a
/// [`XsdNonPositiveInteger`].
#[derive(Debug, thiserror::Error)]
#[error("integer {0} is positive")]
pub struct IntegerIsPositive(XsdInteger);

impl TryFrom<XsdInteger> for XsdNonPositiveInteger {
	type Error = IntegerIsPositive;

	fn try_from(value: XsdInteger) -> Result<Self, Self::Error> {
		if value.is_positive() {
			Err(IntegerIsPositive(value))
		} else {
			Ok(Self(value.into()))
		}
	}
}

/// Negative integer number.
///
/// This is the value-space counterpart of [`lexical::XsdNegativeIntegerStr`].
/// Like its lexical form, `negativeInteger` has no numbered grammar
/// production of its own in the XSD 1.1 Datatypes specification: it is
/// defined by restricting [`XsdNonPositiveInteger`]'s value space with a
/// `maxInclusive` of -1.
/// See: <https://www.w3.org/TR/xmlschema11-2/#negativeInteger>.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct XsdNegativeInteger(BigInt);

impl XsdNegativeInteger {
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

impl XsdTyped for XsdNegativeInteger {
	fn datatype(&self) -> XsdDatatype {
		XsdNonPositiveIntegerDatatype::NegativeInteger.into()
	}
}

impl XsdValueDomain for XsdNegativeInteger {
	type LexicalForm = lexical::XsdNegativeIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdNegativeIntegerString {
		canonical_integer_lexical_form!(lexical::XsdNegativeIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdNegativeInteger {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdNegativeIntegerStr, Self>(lexical_value)
	}
}

impl<'a> From<&'a lexical::XsdNegativeIntegerStr> for XsdNegativeInteger {
	fn from(value: &'a lexical::XsdNegativeIntegerStr) -> Self {
		value.value()
	}
}

impl FromStr for XsdNegativeInteger {
	type Err = lexical::InvalidXsdNegativeIntegerStr<String>;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::XsdNegativeIntegerStr::new(s)
			.map_err(|_| lexical::InvalidXsdNegativeIntegerStr(s.to_owned()))?;
		Ok(l.into())
	}
}

impl fmt::Display for XsdNegativeInteger {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdNegativeInteger {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdNegativeInteger {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdNegativeInteger;

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

impl From<XsdNegativeInteger> for BigInt {
	fn from(value: XsdNegativeInteger) -> Self {
		value.into_big_int()
	}
}

impl_integer_arithmetic!(
	for XsdNegativeInteger where r ( r.is_negative() ) {
		XsdInteger [.0],
		XsdNonPositiveInteger [.0],
		XsdNegativeInteger [.0],
		super::XsdNonNegativeInteger [.into_big_int()],
		super::XsdPositiveInteger [.into_big_int()],
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
			check!(XsdNonPositiveInteger, s);
		}

		let value: XsdNonPositiveInteger = "-0".parse().unwrap();
		assert_eq!(value.to_lexical_form().as_str(), "0");
	}

	#[test]
	fn negative_integer_to_lexical_form() {
		for s in ["-1", "-42", "-123456789012345678901234567890123456789"] {
			check!(XsdNegativeInteger, s);
		}
	}
}
