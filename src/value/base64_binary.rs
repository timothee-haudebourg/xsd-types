use std::{
	borrow::Borrow,
	fmt,
	ops::{Deref, DerefMut},
	str::FromStr,
};

use crate::{
	lexical::{self, LexicalFormOf},
	Datatype, ParseRdf, XsdValue,
};

const CHARS: [char; 64] = [
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
	'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
	'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
	'5', '6', '7', '8', '9', '+', '/',
];

const PADDING: char = '=';

#[derive(Debug, thiserror::Error)]
#[error("invalid base64")]
pub struct InvalidBase64;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Base64BinaryBuf(Vec<u8>);

impl Base64BinaryBuf {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn from_bytes(bytes: Vec<u8>) -> Self {
		Self(bytes)
	}

	pub fn decode(input: impl AsRef<[u8]>) -> Result<Self, InvalidBase64> {
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
					return Err(InvalidBase64);
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

	pub fn into_bytes(self) -> Vec<u8> {
		self.0
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}

	pub fn as_base64_binary(&self) -> &Base64Binary {
		Base64Binary::new(&self.0)
	}

	pub fn as_base64_binary_mut(&mut self) -> &mut Base64Binary {
		Base64Binary::new_mut(&mut self.0)
	}
}

fn decode_char(c: u8) -> Result<u8, InvalidBase64> {
	match c {
		b'A'..=b'Z' => Ok(c - b'A'),
		b'a'..=b'z' => Ok(c - b'a' + 26),
		b'0'..=b'9' => Ok(c - b'0' + 52),
		b'+' => Ok(62),
		b'/' => Ok(63),
		_ => Err(InvalidBase64),
	}
}

impl fmt::Display for Base64BinaryBuf {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_base64_binary().fmt(f)
	}
}

impl From<Vec<u8>> for Base64BinaryBuf {
	fn from(value: Vec<u8>) -> Self {
		Base64BinaryBuf::from_bytes(value)
	}
}

impl FromStr for Base64BinaryBuf {
	type Err = InvalidBase64;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::decode(s)
	}
}

impl AsRef<[u8]> for Base64BinaryBuf {
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl AsRef<Base64Binary> for Base64BinaryBuf {
	fn as_ref(&self) -> &Base64Binary {
		self.as_base64_binary()
	}
}

impl Borrow<Base64Binary> for Base64BinaryBuf {
	fn borrow(&self) -> &Base64Binary {
		self.as_base64_binary()
	}
}

impl Deref for Base64BinaryBuf {
	type Target = Base64Binary;

	fn deref(&self) -> &Self::Target {
		self.as_base64_binary()
	}
}

impl DerefMut for Base64BinaryBuf {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_base64_binary_mut()
	}
}

impl XsdValue for Base64BinaryBuf {
	fn datatype(&self) -> Datatype {
		Datatype::Base64Binary
	}
}

impl ParseRdf for Base64BinaryBuf {
	type LexicalForm = lexical::Base64Binary;
}

impl LexicalFormOf<Base64BinaryBuf> for lexical::Base64Binary {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<Base64BinaryBuf, Self::ValueError> {
		Ok(self.value())
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Base64Binary([u8]);

impl Base64Binary {
	pub fn new(bytes: &[u8]) -> &Self {
		unsafe { std::mem::transmute(bytes) }
	}

	pub fn new_mut(bytes: &mut [u8]) -> &mut Self {
		unsafe { std::mem::transmute(bytes) }
	}

	pub fn chars(&self) -> Chars {
		Chars {
			offset: 0,
			rest: 0,
			padding: false,
			bytes: self.0.iter(),
		}
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

impl<'a> From<&'a [u8]> for &'a Base64Binary {
	fn from(value: &'a [u8]) -> Self {
		Base64Binary::new(value)
	}
}

impl<'a> From<&'a mut [u8]> for &'a Base64Binary {
	fn from(value: &'a mut [u8]) -> Self {
		Base64Binary::new(value)
	}
}

impl<'a> From<&'a mut [u8]> for &'a mut Base64Binary {
	fn from(value: &'a mut [u8]) -> Self {
		Base64Binary::new_mut(value)
	}
}

impl fmt::Display for Base64Binary {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for c in self.chars() {
			c.fmt(f)?;
		}

		Ok(())
	}
}

impl ToOwned for Base64Binary {
	type Owned = Base64BinaryBuf;

	fn to_owned(&self) -> Self::Owned {
		Base64BinaryBuf::from_bytes(self.as_bytes().to_vec())
	}
}

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

impl XsdValue for Base64Binary {
	fn datatype(&self) -> Datatype {
		Datatype::Base64Binary
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const TESTS: [(&'static [u8], &'static str); 9] = [
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
			let output = Base64Binary::new(input).to_string();
			assert_eq!(output, expected)
		}
	}

	#[test]
	fn decoding() {
		for (expected, input) in TESTS {
			let output = Base64BinaryBuf::decode(input).unwrap();
			assert_eq!(output.as_bytes(), expected)
		}
	}
}
