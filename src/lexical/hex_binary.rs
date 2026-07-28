use crate::lexical::lexical_form;
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use str_newtype::StrNewType;

use crate::lexical::Lexical;

/// Hexadecimal string.
///
/// This is the `hexBinary` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-hexBinary>.
///
/// ```abnf
/// hexBinary = *hexOctet
///
/// hexOctet = hexDigit hexDigit
///
/// hexDigit = DIGIT / %x41-46 / %x61-66 ; 0-9, A-F, a-f
/// ```
#[derive(Validate, StrNewType)]
#[automaton(super::grammar::HexBinary)]
#[newtype(owned(HexBinaryBuf, derive(PartialEq, Eq)))]
pub struct HexBinary(str);

impl Lexical for HexBinary {
	type Error = InvalidHexBinary<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidHexBinary(value.to_owned()))
	}
}

lexical_form! {
	ty: HexBinary,
	buffer: HexBinaryBuf,
	value: crate::HexBinaryBuf,
	error: InvalidHexBinary,
	parent_forms: {}
}

impl HexBinary {
	#[inline(always)]
	pub fn value(&self) -> crate::HexBinaryBuf {
		crate::HexBinaryBuf::decode(self.as_bytes()).unwrap()
	}
}

impl PartialEq for HexBinary {
	fn eq(&self, other: &Self) -> bool {
		self.as_str() == other.as_str()
	}
}

impl Eq for HexBinary {}

impl Hash for HexBinary {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_str().hash(h)
	}
}

impl Ord for HexBinary {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_str().cmp(other.as_str())
	}
}

impl PartialOrd for HexBinary {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Default for HexBinaryBuf {
	fn default() -> Self {
		unsafe { Self::new_unchecked(String::new()) }
	}
}

impl PartialOrd for HexBinaryBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for HexBinaryBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_hex_binary().cmp(other.as_hex_binary())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_hex_digits() {
		HexBinary::new("1FaB").unwrap();
	}

	#[test]
	fn parse_empty() {
		HexBinary::new("").unwrap();
	}

	#[test]
	fn parse_odd_length() {
		// An odd number of hex digits is not a valid `hexBinary`.
		HexBinary::new("1").unwrap_err();
	}

	#[test]
	fn parse_non_hex_letter() {
		// `g` is not a hex digit.
		HexBinary::new("g0").unwrap_err();
	}

	#[test]
	fn parse_base64_alphabet_rejected() {
		// This is a valid base64 string, but not a valid hex string (`+` and
		// `/` are not hex digits, and `z` is out of the hex digit range).
		HexBinary::new("+/z=").unwrap_err();
	}

	#[test]
	fn value_roundtrip() {
		let hex = HexBinary::new("4D616E").unwrap();
		assert_eq!(hex.value().as_bytes(), b"Man");
	}
}
