use crate::lexical::lexical_form;
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use str_newtype::StrNewType;

use crate::lexical::XsdLexicalForm;
use crate::XsdError;

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
#[newtype(owned(XsdHexBinaryString, derive(PartialEq, Eq)))]
pub struct XsdHexBinaryStr(str);

impl XsdLexicalForm for XsdHexBinaryStr {
	type Value = crate::XsdHexBinaryBuf;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdHexBinaryStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdHexBinaryStr::new(value)
			.map_err(|_| XsdError::lexical_form(InvalidXsdHexBinaryStr(value)))
	}
}

lexical_form! {
	ty: XsdHexBinaryStr,
	buffer: XsdHexBinaryString,
	value: crate::XsdHexBinaryBuf,
	error: InvalidXsdHexBinaryStr,
	parent_forms: {}
}

impl XsdHexBinaryStr {
	#[inline(always)]
	pub fn value(&self) -> crate::XsdHexBinaryBuf {
		crate::XsdHexBinaryBuf::decode(self.as_bytes()).unwrap()
	}
}

impl PartialEq for XsdHexBinaryStr {
	fn eq(&self, other: &Self) -> bool {
		self.as_str() == other.as_str()
	}
}

impl Eq for XsdHexBinaryStr {}

impl Hash for XsdHexBinaryStr {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_str().hash(h)
	}
}

impl Ord for XsdHexBinaryStr {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_str().cmp(other.as_str())
	}
}

impl PartialOrd for XsdHexBinaryStr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Default for XsdHexBinaryString {
	fn default() -> Self {
		unsafe { Self::new_unchecked(String::new()) }
	}
}

impl PartialOrd for XsdHexBinaryString {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for XsdHexBinaryString {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_xsd_hex_binary_str()
			.cmp(other.as_xsd_hex_binary_str())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_hex_digits() {
		XsdHexBinaryStr::new("1FaB").unwrap();
	}

	#[test]
	fn parse_empty() {
		XsdHexBinaryStr::new("").unwrap();
	}

	#[test]
	fn parse_odd_length() {
		// An odd number of hex digits is not a valid `hexBinary`.
		XsdHexBinaryStr::new("1").unwrap_err();
	}

	#[test]
	fn parse_non_hex_letter() {
		// `g` is not a hex digit.
		XsdHexBinaryStr::new("g0").unwrap_err();
	}

	#[test]
	fn parse_base64_alphabet_rejected() {
		// This is a valid base64 string, but not a valid hex string (`+` and
		// `/` are not hex digits, and `z` is out of the hex digit range).
		XsdHexBinaryStr::new("+/z=").unwrap_err();
	}

	#[test]
	fn value_roundtrip() {
		let hex = XsdHexBinaryStr::new("4D616E").unwrap();
		assert_eq!(hex.value().as_bytes(), b"Man");
	}
}
