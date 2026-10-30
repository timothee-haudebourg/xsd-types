use std::{
	borrow::Borrow,
	fmt,
	ops::{Deref, DerefMut},
	str::FromStr,
};

use crate::{Datatype, XsdFromStr, XsdTyped, XsdValueDomain, lexical};

const CHARS: [char; 16] = [
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

/// Error raised when decoding an invalid hexadecimal-encoded byte string.
#[derive(Debug, thiserror::Error)]
#[error("invalid hexadecimal")]
pub struct InvalidHex;

/// Decoded `hexBinary` value, as an owned byte buffer.
///
/// This is the value-space representation of the XSD 1.1 `hexBinary`
/// datatype. See: <https://www.w3.org/TR/xmlschema11-2/#hexBinary>.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XsdHexBinaryBuf(Vec<u8>);

impl XsdHexBinaryBuf {
	/// Creates a new, empty `hexBinary` value.
	pub fn new() -> Self {
		Self::default()
	}

	/// Creates a `hexBinary` value from already-decoded raw bytes.
	pub fn from_bytes(bytes: Vec<u8>) -> Self {
		Self(bytes)
	}

	/// Decodes a hexadecimal-encoded byte string into its raw bytes.
	pub fn decode(input: impl AsRef<[u8]>) -> Result<Self, InvalidHex> {
		let input = input.as_ref();
		let mut bytes = Vec::with_capacity(input.len() / 2);

		let mut iter = input.iter();
		while let Some(&a) = iter.next() {
			let a = decode_char(a)?;

			match iter.next() {
				Some(&b) => {
					let b = decode_char(b)?;
					bytes.push(a << 4 | b)
				}
				None => return Err(InvalidHex),
			}
		}

		Ok(Self(bytes))
	}

	/// Consumes the buffer, returning the raw decoded bytes.
	pub fn into_bytes(self) -> Vec<u8> {
		self.0
	}

	/// Returns the raw decoded bytes as a slice.
	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}

	/// Borrows `self` as a [`XsdHexBinary`] slice.
	pub fn as_hex_binary(&self) -> &XsdHexBinary {
		XsdHexBinary::new(&self.0)
	}

	/// Mutably borrows `self` as a [`XsdHexBinary`] slice.
	pub fn as_hex_binary_mut(&mut self) -> &mut XsdHexBinary {
		XsdHexBinary::new_mut(&mut self.0)
	}
}

fn decode_char(c: u8) -> Result<u8, InvalidHex> {
	match c {
		b'0'..=b'9' => Ok(c - b'0'),
		b'A'..=b'F' => Ok(0xa + (c - b'A')),
		b'a'..=b'f' => Ok(0xa + (c - b'a')),
		_ => Err(InvalidHex),
	}
}

impl fmt::Display for XsdHexBinaryBuf {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_hex_binary().fmt(f)
	}
}

impl From<Vec<u8>> for XsdHexBinaryBuf {
	fn from(value: Vec<u8>) -> Self {
		XsdHexBinaryBuf::from_bytes(value)
	}
}

impl FromStr for XsdHexBinaryBuf {
	type Err = lexical::InvalidXsdHexBinaryStr<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::XsdHexBinaryStr::new(s)
			.map_err(|_| lexical::InvalidXsdHexBinaryStr(s.to_owned()))?;
		Ok(l.into())
	}
}

impl AsRef<[u8]> for XsdHexBinaryBuf {
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl AsRef<XsdHexBinary> for XsdHexBinaryBuf {
	fn as_ref(&self) -> &XsdHexBinary {
		self.as_hex_binary()
	}
}

impl Borrow<XsdHexBinary> for XsdHexBinaryBuf {
	fn borrow(&self) -> &XsdHexBinary {
		self.as_hex_binary()
	}
}

impl Deref for XsdHexBinaryBuf {
	type Target = XsdHexBinary;

	fn deref(&self) -> &Self::Target {
		self.as_hex_binary()
	}
}

impl DerefMut for XsdHexBinaryBuf {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_hex_binary_mut()
	}
}

impl XsdTyped for XsdHexBinaryBuf {
	fn datatype(&self) -> Datatype {
		Datatype::HexBinary
	}
}

impl XsdValueDomain for XsdHexBinaryBuf {
	type LexicalForm = lexical::XsdHexBinaryStr;

	fn to_lexical_form(&self) -> lexical::XsdHexBinaryString {
		// `Display` emits the canonical lexical form by contract (upper case
		// hexadecimal digits), so it is always valid and this cannot panic.
		lexical::XsdHexBinaryString::new(self.to_string())
			.expect("Display emits a valid hexBinary lexical form")
	}
}

impl<'a> XsdFromStr<'a> for XsdHexBinaryBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdHexBinaryStr, Self>(lexical_value)
	}
}

impl<'a> From<&'a lexical::XsdHexBinaryStr> for XsdHexBinaryBuf {
	fn from(value: &'a lexical::XsdHexBinaryStr) -> XsdHexBinaryBuf {
		value.value()
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdHexBinaryBuf {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdHexBinaryBuf {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdHexBinaryBuf;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#hexBinary")
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

/// Decoded `hexBinary` value, as a borrowed byte slice.
///
/// This is the borrowed counterpart of [`XsdHexBinaryBuf`], analogous to how
/// `str` relates to `String`.
/// See: <https://www.w3.org/TR/xmlschema11-2/#hexBinary>.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct XsdHexBinary([u8]);

impl XsdHexBinary {
	/// Wraps a byte slice as a [`XsdHexBinary`] value, without copying it.
	pub fn new(bytes: &[u8]) -> &Self {
		unsafe { std::mem::transmute(bytes) }
	}

	/// Wraps a mutable byte slice as a [`XsdHexBinary`] value, without copying
	/// it.
	pub fn new_mut(bytes: &mut [u8]) -> &mut Self {
		unsafe { std::mem::transmute(bytes) }
	}

	/// Returns an iterator over the hexadecimal-encoded characters
	/// representing this value.
	pub fn chars(&self) -> Chars<'_> {
		Chars {
			pending: None,
			bytes: self.0.iter(),
		}
	}

	/// Returns the raw decoded bytes as a slice.
	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

impl<'a> From<&'a [u8]> for &'a XsdHexBinary {
	fn from(value: &'a [u8]) -> Self {
		XsdHexBinary::new(value)
	}
}

impl<'a> From<&'a mut [u8]> for &'a XsdHexBinary {
	fn from(value: &'a mut [u8]) -> Self {
		XsdHexBinary::new(value)
	}
}

impl<'a> From<&'a mut [u8]> for &'a mut XsdHexBinary {
	fn from(value: &'a mut [u8]) -> Self {
		XsdHexBinary::new_mut(value)
	}
}

impl fmt::Display for XsdHexBinary {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for c in self.chars() {
			c.fmt(f)?
		}

		Ok(())
	}
}

impl ToOwned for XsdHexBinary {
	type Owned = XsdHexBinaryBuf;

	fn to_owned(&self) -> Self::Owned {
		XsdHexBinaryBuf::from_bytes(self.as_bytes().to_vec())
	}
}

impl XsdTyped for XsdHexBinary {
	fn datatype(&self) -> Datatype {
		Datatype::HexBinary
	}
}

/// Iterator over the hexadecimal-encoded characters representing a
/// [`XsdHexBinary`] value.
pub struct Chars<'a> {
	pending: Option<char>,
	bytes: std::slice::Iter<'a, u8>,
}

impl<'a> Iterator for Chars<'a> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		self.pending.take().or_else(|| {
			self.bytes.next().map(|v| {
				let a = v >> 4;
				let b = v & 0x0f;

				self.pending = Some(CHARS[b as usize]);
				CHARS[a as usize]
			})
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const TESTS: [(&[u8], &str); 9] = [
		(b"M", "4D"),
		(b"Ma", "4D61"),
		(b"Man", "4D616E"),
		(b"light w", "6C696768742077"),
		(b"light wo", "6C6967687420776F"),
		(b"light wor", "6C6967687420776F72"),
		(b"light work", "6C6967687420776F726B"),
		(b"light work.", "6C6967687420776F726B2E"),
		(
			b"Many hands make light work.",
			"4D616E792068616E6473206D616B65206C6967687420776F726B2E",
		),
	];

	#[test]
	fn encoding() {
		for (input, expected) in TESTS {
			let output = XsdHexBinary::new(input).to_string();
			assert_eq!(output, expected)
		}
	}

	#[test]
	fn decoding() {
		for (expected, input) in TESTS {
			let output = XsdHexBinaryBuf::decode(input).unwrap();
			assert_eq!(output.as_bytes(), expected)
		}
	}

	#[test]
	fn to_lexical_form_round_trips() {
		for input in ["", "00", "FF00", "4D616E", "DEADBEEF"] {
			let value = <XsdHexBinaryBuf as XsdFromStr>::xsd_from_str(input).unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), input);
			assert_eq!(XsdHexBinaryBuf::from_str(lexical.as_str()).unwrap(), value);
		}
	}

	#[test]
	fn to_lexical_form_is_upper_case() {
		let value = XsdHexBinaryBuf::from_str("deadbeef").unwrap();
		assert_eq!(value.to_lexical_form().as_str(), "DEADBEEF");
	}
}
