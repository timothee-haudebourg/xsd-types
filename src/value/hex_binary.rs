use std::{
	fmt,
	ops::{Deref, DerefMut},
	str::FromStr,
};

use crate::{
	lexical::{self, LexicalFormOf},
	Datatype, ParseRdf, XsdDatatype,
};

const CHARS: [char; 16] = [
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

#[derive(Debug, thiserror::Error)]
#[error("invalid hexadecimal")]
pub struct InvalidHex;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexBinaryBuf(Vec<u8>);

impl HexBinaryBuf {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn from_bytes(bytes: Vec<u8>) -> Self {
		Self(bytes)
	}

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

	pub fn into_bytes(self) -> Vec<u8> {
		self.0
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}

	pub fn as_hex_binary(&self) -> &HexBinary {
		HexBinary::new(&self.0)
	}

	pub fn as_hex_binary_mut(&mut self) -> &mut HexBinary {
		HexBinary::new_mut(&mut self.0)
	}
}

pub fn decode_char(c: u8) -> Result<u8, InvalidHex> {
	match c {
		b'0'..=b'9' => Ok(c - b'0'),
		b'A'..=b'F' => Ok(0xa + (c - b'A')),
		b'a'..=b'f' => Ok(0xa + (c - b'a')),
		_ => Err(InvalidHex),
	}
}

impl From<Vec<u8>> for HexBinaryBuf {
	fn from(value: Vec<u8>) -> Self {
		HexBinaryBuf::from_bytes(value)
	}
}

impl FromStr for HexBinaryBuf {
	type Err = InvalidHex;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::decode(s)
	}
}

impl AsRef<[u8]> for HexBinaryBuf {
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl AsRef<HexBinary> for HexBinaryBuf {
	fn as_ref(&self) -> &HexBinary {
		self.as_hex_binary()
	}
}

impl Deref for HexBinaryBuf {
	type Target = HexBinary;

	fn deref(&self) -> &Self::Target {
		self.as_hex_binary()
	}
}

impl DerefMut for HexBinaryBuf {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_hex_binary_mut()
	}
}

impl XsdDatatype for HexBinaryBuf {
	fn type_(&self) -> Datatype {
		Datatype::HexBinary
	}
}

impl ParseRdf for HexBinaryBuf {
	type LexicalForm = lexical::HexBinary;
}

impl LexicalFormOf<HexBinaryBuf> for lexical::HexBinary {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<HexBinaryBuf, Self::ValueError> {
		Ok(self.value())
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexBinary([u8]);

impl HexBinary {
	pub fn new(bytes: &[u8]) -> &Self {
		unsafe { std::mem::transmute(bytes) }
	}

	pub fn new_mut(bytes: &mut [u8]) -> &mut Self {
		unsafe { std::mem::transmute(bytes) }
	}
}

impl<'a> From<&'a [u8]> for &'a HexBinary {
	fn from(value: &'a [u8]) -> Self {
		HexBinary::new(value)
	}
}

impl<'a> From<&'a mut [u8]> for &'a HexBinary {
	fn from(value: &'a mut [u8]) -> Self {
		HexBinary::new(value)
	}
}

impl<'a> From<&'a mut [u8]> for &'a mut HexBinary {
	fn from(value: &'a mut [u8]) -> Self {
		HexBinary::new_mut(value)
	}
}

impl fmt::Display for HexBinary {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for c in &self.0 {
			let a = c >> 4;
			let b = c & 0x0f;

			CHARS[a as usize].fmt(f)?;
			CHARS[b as usize].fmt(f)?
		}

		Ok(())
	}
}

impl XsdDatatype for HexBinary {
	fn type_(&self) -> Datatype {
		Datatype::HexBinary
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const TESTS: [(&'static [u8], &'static str); 9] = [
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
			let output = HexBinary::new(input).to_string();
			assert_eq!(output, expected)
		}
	}

	#[test]
	fn decoding() {
		for (expected, input) in TESTS {
			let output = HexBinaryBuf::decode(input).unwrap();
			assert_eq!(output.as_bytes(), expected)
		}
	}
}
