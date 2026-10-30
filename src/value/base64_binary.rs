use std::{
	borrow::Borrow,
	fmt,
	ops::{Deref, DerefMut},
	str::FromStr,
};

use crate::{XsdDatatype, XsdFromStr, XsdTyped, XsdValueDomain, lexical};

const CHARS: [char; 64] = [
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
	'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
	'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
	'5', '6', '7', '8', '9', '+', '/',
];

const PADDING: char = '=';

/// Error raised when decoding an invalid base64-encoded byte string.
#[derive(Debug, thiserror::Error)]
#[error("invalid base64")]
pub struct InvalidXsdBase64;

/// Decoded `base64Binary` value, as an owned byte buffer.
///
/// This is the value-space representation of the XSD 1.1 `base64Binary`
/// datatype. See: <https://www.w3.org/TR/xmlschema11-2/#base64Binary>.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XsdBase64BinaryBuf(Vec<u8>);

impl XsdBase64BinaryBuf {
	/// Creates a new, empty `base64Binary` value.
	pub fn new() -> Self {
		Self::default()
	}

	/// Creates a `base64Binary` value from already-decoded raw bytes.
	pub fn from_bytes(bytes: Vec<u8>) -> Self {
		Self(bytes)
	}

	/// Decodes a base64-encoded byte string into its raw bytes.
	pub fn decode(input: impl AsRef<[u8]>) -> Result<Self, InvalidXsdBase64> {
		let input = input.as_ref();
		let mut bytes = Vec::with_capacity(input.len() * 3 / 4);
		let mut buffer = 0u16;
		let mut buffer_len = 0u16;
		let mut padding = false;

		for &c in input {
			if c == 0x20 {
				continue;
			}

			if padding {
				if c != b'=' {
					return Err(InvalidXsdBase64);
				}
			} else if c == b'=' {
				padding = true
			} else {
				buffer_len += 6;
				buffer = buffer << 6 | decode_char(c)? as u16;

				while buffer_len >= 8 {
					buffer_len -= 8;
					let b = buffer >> buffer_len;
					bytes.push(b as u8)
				}
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

	/// Borrows `self` as a [`XsdBase64Binary`] slice.
	pub fn as_base64_binary(&self) -> &XsdBase64Binary {
		XsdBase64Binary::new(&self.0)
	}

	/// Mutably borrows `self` as a [`XsdBase64Binary`] slice.
	pub fn as_base64_binary_mut(&mut self) -> &mut XsdBase64Binary {
		XsdBase64Binary::new_mut(&mut self.0)
	}
}

fn decode_char(c: u8) -> Result<u8, InvalidXsdBase64> {
	match c {
		b'A'..=b'Z' => Ok(c - b'A'),
		b'a'..=b'z' => Ok(c - b'a' + 26),
		b'0'..=b'9' => Ok(c - b'0' + 52),
		b'+' => Ok(62),
		b'/' => Ok(63),
		_ => Err(InvalidXsdBase64),
	}
}

impl fmt::Display for XsdBase64BinaryBuf {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_base64_binary().fmt(f)
	}
}

impl From<Vec<u8>> for XsdBase64BinaryBuf {
	fn from(value: Vec<u8>) -> Self {
		XsdBase64BinaryBuf::from_bytes(value)
	}
}

impl FromStr for XsdBase64BinaryBuf {
	type Err = lexical::InvalidXsdBase64BinaryStr<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::XsdBase64BinaryStr::new(s)
			.map_err(|_| lexical::InvalidXsdBase64BinaryStr(s.to_owned()))?;
		Ok(l.into())
	}
}

impl AsRef<[u8]> for XsdBase64BinaryBuf {
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl AsRef<XsdBase64Binary> for XsdBase64BinaryBuf {
	fn as_ref(&self) -> &XsdBase64Binary {
		self.as_base64_binary()
	}
}

impl Borrow<XsdBase64Binary> for XsdBase64BinaryBuf {
	fn borrow(&self) -> &XsdBase64Binary {
		self.as_base64_binary()
	}
}

impl Deref for XsdBase64BinaryBuf {
	type Target = XsdBase64Binary;

	fn deref(&self) -> &Self::Target {
		self.as_base64_binary()
	}
}

impl DerefMut for XsdBase64BinaryBuf {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_base64_binary_mut()
	}
}

impl XsdTyped for XsdBase64BinaryBuf {
	fn datatype(&self) -> XsdDatatype {
		XsdDatatype::Base64Binary
	}
}

impl XsdValueDomain for XsdBase64BinaryBuf {
	type LexicalForm = lexical::XsdBase64BinaryStr;

	fn to_lexical_form(&self) -> lexical::XsdBase64BinaryString {
		// `Display` emits the canonical lexical form by contract (padded
		// base64 without whitespace), so it is always valid and this cannot
		// panic.
		lexical::XsdBase64BinaryString::new(self.to_string())
			.expect("Display emits a valid base64Binary lexical form")
	}
}

impl<'a> XsdFromStr<'a> for XsdBase64BinaryBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdBase64BinaryStr, Self>(lexical_value)
	}
}

impl<'a> From<&'a lexical::XsdBase64BinaryStr> for XsdBase64BinaryBuf {
	fn from(value: &'a lexical::XsdBase64BinaryStr) -> XsdBase64BinaryBuf {
		value.value()
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdBase64BinaryBuf {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdBase64BinaryBuf {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdBase64BinaryBuf;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#base64Binary")
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

/// Decoded `base64Binary` value, as a borrowed byte slice.
///
/// This is the borrowed counterpart of [`XsdBase64BinaryBuf`], analogous to how
/// `str` relates to `String`.
/// See: <https://www.w3.org/TR/xmlschema11-2/#base64Binary>.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct XsdBase64Binary([u8]);

impl XsdBase64Binary {
	/// Wraps a byte slice as a [`XsdBase64Binary`] value, without copying it.
	pub fn new(bytes: &[u8]) -> &Self {
		unsafe { std::mem::transmute(bytes) }
	}

	/// Wraps a mutable byte slice as a [`XsdBase64Binary`] value, without
	/// copying it.
	pub fn new_mut(bytes: &mut [u8]) -> &mut Self {
		unsafe { std::mem::transmute(bytes) }
	}

	/// Returns an iterator over the base64-encoded characters representing
	/// this value.
	pub fn chars(&self) -> Chars<'_> {
		Chars {
			offset: 0,
			rest: 0,
			padding: false,
			bytes: self.0.iter(),
		}
	}

	/// Returns the raw decoded bytes as a slice.
	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

impl<'a> From<&'a [u8]> for &'a XsdBase64Binary {
	fn from(value: &'a [u8]) -> Self {
		XsdBase64Binary::new(value)
	}
}

impl<'a> From<&'a mut [u8]> for &'a XsdBase64Binary {
	fn from(value: &'a mut [u8]) -> Self {
		XsdBase64Binary::new(value)
	}
}

impl<'a> From<&'a mut [u8]> for &'a mut XsdBase64Binary {
	fn from(value: &'a mut [u8]) -> Self {
		XsdBase64Binary::new_mut(value)
	}
}

impl fmt::Display for XsdBase64Binary {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for c in self.chars() {
			c.fmt(f)?;
		}

		Ok(())
	}
}

impl ToOwned for XsdBase64Binary {
	type Owned = XsdBase64BinaryBuf;

	fn to_owned(&self) -> Self::Owned {
		XsdBase64BinaryBuf::from_bytes(self.as_bytes().to_vec())
	}
}

/// Iterator over the base64-encoded characters representing a
/// [`XsdBase64Binary`] value.
pub struct Chars<'a> {
	offset: u8,
	rest: u8,
	padding: bool,
	bytes: std::slice::Iter<'a, u8>,
}

impl<'a> Iterator for Chars<'a> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		if self.padding {
			if self.offset <= 6 {
				self.offset += 2;
				Some(PADDING)
			} else {
				None
			}
		} else if self.offset == 6 {
			let sextet = self.rest;
			self.rest = 0;
			self.offset = 0;
			Some(CHARS[sextet as usize])
		} else {
			match self.bytes.next() {
				Some(b) => {
					let sextet = self.rest | (b >> 2 >> self.offset & 0b111111);
					self.offset += 2;
					self.rest = b << (6 - self.offset) & 0b111111;
					Some(CHARS[sextet as usize])
				}
				None => {
					if self.offset > 0 {
						let sextet = self.rest;
						self.offset += 2;
						self.padding = true;
						Some(CHARS[sextet as usize])
					} else {
						None
					}
				}
			}
		}
	}
}

impl XsdTyped for XsdBase64Binary {
	fn datatype(&self) -> XsdDatatype {
		XsdDatatype::Base64Binary
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const TESTS: [(&[u8], &str); 9] = [
		(b"M", "TQ=="),
		(b"Ma", "TWE="),
		(b"Man", "TWFu"),
		(b"light w", "bGlnaHQgdw=="),
		(b"light wo", "bGlnaHQgd28="),
		(b"light wor", "bGlnaHQgd29y"),
		(b"light work", "bGlnaHQgd29yaw=="),
		(b"light work.", "bGlnaHQgd29yay4="),
		(
			b"Many hands make light work.",
			"TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu",
		),
	];

	#[test]
	fn encoding() {
		for (input, expected) in TESTS {
			let output = XsdBase64Binary::new(input).to_string();
			assert_eq!(output, expected)
		}
	}

	#[test]
	fn decoding() {
		for (expected, input) in TESTS {
			let output = XsdBase64BinaryBuf::decode(input).unwrap();
			assert_eq!(output.as_bytes(), expected)
		}
	}

	#[test]
	fn to_lexical_form_round_trips() {
		// Empty, 1 byte (`==` padding), 2 bytes (`=` padding), 3 bytes (no
		// padding), and a longer value.
		for input in [
			"",
			"TQ==",
			"TWE=",
			"TWFu",
			"TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu",
		] {
			let value = <XsdBase64BinaryBuf as XsdFromStr>::xsd_from_str(input).unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), input);
			assert_eq!(
				XsdBase64BinaryBuf::from_str(lexical.as_str()).unwrap(),
				value
			);
		}
	}

	#[test]
	fn to_lexical_form_of_all_byte_values() {
		let value = XsdBase64BinaryBuf::from_bytes((0..=255).collect());
		let lexical = value.to_lexical_form();
		assert_eq!(
			XsdBase64BinaryBuf::from_str(lexical.as_str()).unwrap(),
			value
		);
	}
}
