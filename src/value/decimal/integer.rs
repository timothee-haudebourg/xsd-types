use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Div, Mul, Sub},
	str::FromStr,
};

use num_bigint::{BigInt, TryFromBigIntError};
use num_traits::{Signed, Zero};

use crate::{
	Datatype, IntDatatype, IntegerDatatype, LongDatatype, NonNegativeIntegerDatatype,
	NonPositiveIntegerDatatype, ShortDatatype, UnsignedIntDatatype, UnsignedLongDatatype,
	UnsignedShortDatatype, XsdFromStr, XsdTyped, XsdValue, lexical,
};

use super::{I8_MIN, I16_MIN, I32_MIN, I64_MIN, Sign, U8_MAX, U16_MAX, U32_MAX, U64_MAX};

/// Builds the canonical owned lexical form (`$buf`) of an integer-like value
/// from its `Display` output.
///
/// The integer `Display` output (no `+` sign, no leading zeros, `0` for zero)
/// is always a valid lexical form of the value's datatype: the value is
/// already known to belong to the datatype (e.g. a negative value can never be
/// a `nonNegativeInteger`), so the checked constructor cannot fail.
macro_rules! canonical_integer_lexical_form {
	($buf:ty, $value:expr) => {
		<$buf>::new($value.to_string())
			.expect("integer `Display` output is a valid lexical form of its datatype")
	};
}

mod non_negative_integer;
mod non_positive_integer;

pub use non_negative_integer::*;
pub use non_positive_integer::*;

/// Integer number.
///
/// This is the value-space counterpart of [`lexical::XsdIntegerStr`]. Like its
/// lexical form, `integer` has no numbered grammar production of its own
/// in the XSD 1.1 Datatypes specification: it is defined by restricting
/// [`XsdDecimal`](crate::XsdDecimal)'s value space to numbers with no fractional
/// part.
/// See: <https://www.w3.org/TR/xmlschema11-2/#integer>.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct XsdInteger(BigInt);

impl XsdInteger {
	/// Converts a `BigInt` reference into an `XsdInteger` reference.
	#[inline(always)]
	pub fn from_bigint_ref(n: &BigInt) -> &Self {
		unsafe {
			// This is safe because `XsdInteger` is a transparent wrapper around
			// `BigInt`.
			std::mem::transmute(n)
		}
	}

	/// Converts a `BigInt` into an `XsdInteger`.
	#[inline(always)]
	pub fn from_bigint(n: BigInt) -> Self {
		Self(n)
	}

	/// Creates an integer from its sign and unsigned big-endian bytes
	/// representation.
	pub fn from_bytes_be(sign: Sign, bytes: &[u8]) -> Self {
		Self(BigInt::from_bytes_be(sign, bytes))
	}

	/// Creates an integer from its sign and unsigned little-endian bytes
	/// representation.
	pub fn from_bytes_le(sign: Sign, bytes: &[u8]) -> Self {
		Self(BigInt::from_bytes_le(sign, bytes))
	}

	/// Creates an integer from its two's-complement big-endian bytes
	/// representation.
	pub fn from_signed_bytes_be(bytes: &[u8]) -> Self {
		Self(BigInt::from_signed_bytes_be(bytes))
	}

	/// Creates an integer from its two's-complement little-endian bytes
	/// representation.
	pub fn from_signed_bytes_le(bytes: &[u8]) -> Self {
		Self(BigInt::from_signed_bytes_le(bytes))
	}

	/// Returns the integer `0`.
	#[inline(always)]
	pub fn zero() -> Self {
		Self(BigInt::zero())
	}

	/// Returns `true` if this value is zero.
	#[inline(always)]
	pub fn is_zero(&self) -> bool {
		self.0.is_zero()
	}

	/// Returns `true` if this value is strictly positive.
	#[inline(always)]
	pub fn is_positive(&self) -> bool {
		self.0.is_positive()
	}

	/// Returns `true` if this value is strictly negative.
	#[inline(always)]
	pub fn is_negative(&self) -> bool {
		self.0.is_negative()
	}

	/// Returns the most specific XSD integer-derived datatype that can
	/// represent this value (e.g. `byte`, `int`, `nonNegativeInteger`, ...).
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

impl XsdTyped for XsdInteger {
	#[inline(always)]
	fn datatype(&self) -> Datatype {
		self.integer_type().into()
	}
}

impl XsdValue for XsdInteger {
	type LexicalForm = lexical::XsdIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdIntegerString {
		canonical_integer_lexical_form!(lexical::XsdIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdInteger {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdIntegerStr, Self>(lexical_value)
	}
}

impl From<BigInt> for XsdInteger {
	#[inline(always)]
	fn from(value: BigInt) -> Self {
		Self(value)
	}
}

impl From<XsdInteger> for BigInt {
	#[inline(always)]
	fn from(value: XsdInteger) -> Self {
		value.0
	}
}

impl<'a> From<&'a lexical::XsdIntegerStr> for XsdInteger {
	#[inline(always)]
	fn from(value: &'a lexical::XsdIntegerStr) -> Self {
		Self(value.as_str().parse().unwrap())
	}
}

impl From<lexical::XsdIntegerString> for XsdInteger {
	#[inline(always)]
	fn from(value: lexical::XsdIntegerString) -> Self {
		value.as_xsd_integer_str().into()
	}
}

impl FromStr for XsdInteger {
	type Err = lexical::InvalidXsdIntegerStr<String>;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::XsdIntegerStr::new(s)
			.map_err(|_| lexical::InvalidXsdIntegerStr(s.to_owned()))?;
		Ok(l.into())
	}
}

impl From<lexical::XsdNonPositiveIntegerString> for XsdInteger {
	#[inline(always)]
	fn from(value: lexical::XsdNonPositiveIntegerString) -> Self {
		value.as_integer().into()
	}
}

impl From<XsdNonNegativeInteger> for XsdInteger {
	fn from(value: XsdNonNegativeInteger) -> Self {
		let n: BigInt = value.into();
		Self(n)
	}
}

impl fmt::Display for XsdInteger {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdInteger {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdInteger {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdInteger;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#integer")
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

impl AsRef<BigInt> for XsdInteger {
	#[inline(always)]
	fn as_ref(&self) -> &BigInt {
		&self.0
	}
}

impl Borrow<BigInt> for XsdInteger {
	#[inline(always)]
	fn borrow(&self) -> &BigInt {
		&self.0
	}
}

/// Error raised when converting an [`XsdInteger`] into a smaller target
/// integer type that cannot represent its value.
#[derive(Debug, thiserror::Error)]
#[error("integer out of supported bounds: {0}")]
pub struct IntegerOutOfTargetBounds(pub XsdInteger);

macro_rules! from {
	{ $( $ty:ty ),* } => {
		$(
			impl From<$ty> for XsdInteger {
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
			impl TryFrom<XsdInteger> for $ty {
				type Error = IntegerOutOfTargetBounds;

				fn try_from(value: XsdInteger) -> Result<Self, Self::Error> {
					value.0.try_into().map_err(|e: TryFromBigIntError<BigInt>| IntegerOutOfTargetBounds(XsdInteger(e.into_original())))
				}
			}

			impl<'a> TryFrom<&'a XsdInteger> for $ty {
				type Error = IntegerOutOfTargetBounds;

				fn try_from(value: &'a XsdInteger) -> Result<Self, Self::Error> {
					value.0.clone().try_into().map_err(|e: TryFromBigIntError<BigInt>| IntegerOutOfTargetBounds(XsdInteger(e.into_original()))) // TODO avoid cloning.
				}
			}
		)*
	};
}

try_into!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

pub type XsdLong = i64;

pub trait XsdLongSubtype {
	fn long_type(&self) -> LongDatatype;
}

impl XsdLongSubtype for XsdLong {
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

impl XsdTyped for XsdLong {
	fn datatype(&self) -> Datatype {
		self.long_type().into()
	}
}

impl XsdValue for XsdLong {
	type LexicalForm = lexical::XsdIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdIntegerString {
		canonical_integer_lexical_form!(lexical::XsdIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdLong {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdIntegerStr, Self>(lexical_value)
	}
}

pub type XsdInt = i32;

pub trait XsdIntSubtype {
	fn int_type(&self) -> IntDatatype;
}

impl XsdIntSubtype for XsdInt {
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

impl XsdTyped for XsdInt {
	fn datatype(&self) -> Datatype {
		self.int_type().into()
	}
}

impl XsdValue for XsdInt {
	type LexicalForm = lexical::XsdIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdIntegerString {
		canonical_integer_lexical_form!(lexical::XsdIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdInt {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdIntegerStr, Self>(lexical_value)
	}
}

pub type XsdShort = i16;

pub trait XsdShortSubtype {
	fn short_type(&self) -> ShortDatatype;
}

impl XsdShortSubtype for XsdShort {
	fn short_type(&self) -> ShortDatatype {
		if (i8::MIN as i16..=i8::MAX as i16).contains(self) {
			ShortDatatype::Byte
		} else {
			ShortDatatype::Short
		}
	}
}

impl XsdTyped for XsdShort {
	fn datatype(&self) -> Datatype {
		self.short_type().into()
	}
}

impl XsdValue for XsdShort {
	type LexicalForm = lexical::XsdIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdIntegerString {
		canonical_integer_lexical_form!(lexical::XsdIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdShort {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdIntegerStr, Self>(lexical_value)
	}
}

pub type XsdByte = i8;

impl XsdTyped for XsdByte {
	fn datatype(&self) -> Datatype {
		ShortDatatype::Byte.into()
	}
}

impl XsdValue for XsdByte {
	type LexicalForm = lexical::XsdIntegerStr;

	fn to_lexical_form(&self) -> lexical::XsdIntegerString {
		canonical_integer_lexical_form!(lexical::XsdIntegerString, self)
	}
}

impl<'a> XsdFromStr<'a> for XsdByte {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdIntegerStr, Self>(lexical_value)
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
	for XsdInteger where r (true) {
		XsdInteger [.0], i8, i16, i32, i64, isize, u8, u16, u32, u64, usize
	}
}

#[cfg(test)]
mod tests {
	use super::*;

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
	fn integer_to_lexical_form() {
		for s in [
			"0",
			"-1",
			"42",
			"123456789012345678901234567890123456789",
			"-123456789012345678901234567890123456789",
		] {
			check!(XsdInteger, s);
		}
	}

	#[test]
	fn integer_to_lexical_form_is_canonical() {
		let value: XsdInteger = "+007".parse().unwrap();
		assert_eq!(value.to_lexical_form().as_str(), "7");

		let value: XsdInteger = "-0".parse().unwrap();
		assert_eq!(value.to_lexical_form().as_str(), "0");
	}

	#[test]
	fn long_to_lexical_form() {
		for s in ["0", "-1", "9223372036854775807", "-9223372036854775808"] {
			check!(XsdLong, s);
		}
	}

	#[test]
	fn int_to_lexical_form() {
		for s in ["0", "-1", "2147483647", "-2147483648"] {
			check!(XsdInt, s);
		}
	}

	#[test]
	fn short_to_lexical_form() {
		for s in ["0", "-1", "32767", "-32768"] {
			check!(XsdShort, s);
		}
	}

	#[test]
	fn byte_to_lexical_form() {
		for s in ["0", "-1", "127", "-128"] {
			check!(XsdByte, s);
		}
	}
}
