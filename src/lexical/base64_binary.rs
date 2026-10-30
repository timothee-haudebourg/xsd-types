use crate::lexical::lexical_form;
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use str_newtype::StrNewType;

use crate::lexical::XsdLexicalForm;
use crate::XsdError;

/// Base 64 string.
///
/// This is the `XsdBase64BinaryStr` production of the XSD 1.1 Datatypes
/// specification:
/// <https://www.w3.org/TR/xmlschema11-2/#nt-XsdBase64BinaryStr>.
///
/// ```abnf
/// XsdBase64BinaryStr = [ *B64quad B64final ]
///
/// B64quad = B64 B64 B64 B64
///
/// B64final = B64finalquad / Padded16 / Padded8
///
/// B64finalquad = B64 B64 B64 B64char
///
/// Padded16 = B64 B64 B16 "="
///
/// Padded8 = B64 B04 "=" [ %x20 ] "="
///
/// B64 = B64char [ %x20 ]
///
/// B64char = ALPHA / DIGIT / "+" / "/"
///
/// B16 = B16char [ %x20 ]
///
/// B16char = %s"A" / %s"E" / %s"I" / %s"M" / %s"Q" / %s"U" / %s"Y"
///         / %s"c" / %s"g" / %s"k" / %s"o" / %s"s" / %s"w"
///         / "0" / "4" / "8"
///
/// B04 = B04char [ %x20 ]
///
/// B04char = %s"A" / %s"Q" / %s"g" / %s"w"
/// ```
#[derive(Validate, StrNewType)]
#[automaton(super::grammar::Base64Binary)]
#[newtype(owned(XsdBase64BinaryString, derive(PartialEq, Eq)))]
pub struct XsdBase64BinaryStr(str);

impl XsdLexicalForm for XsdBase64BinaryStr {
	type Value = crate::XsdBase64BinaryBuf;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdBase64BinaryStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdBase64BinaryStr::new(value)
			.map_err(|_| XsdError::lexical_form(InvalidXsdBase64BinaryStr(value)))
	}
}

lexical_form! {
	ty: XsdBase64BinaryStr,
	buffer: XsdBase64BinaryString,
	value: crate::XsdBase64BinaryBuf,
	error: InvalidXsdBase64BinaryStr,
	parent_forms: {}
}

impl XsdBase64BinaryStr {
	#[inline(always)]
	pub fn value(&self) -> crate::XsdBase64BinaryBuf {
		crate::XsdBase64BinaryBuf::decode(self.as_bytes()).unwrap()
	}
}

impl PartialEq for XsdBase64BinaryStr {
	fn eq(&self, other: &Self) -> bool {
		self.as_str() == other.as_str()
	}
}

impl Eq for XsdBase64BinaryStr {}

impl Hash for XsdBase64BinaryStr {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_str().hash(h)
	}
}

impl Ord for XsdBase64BinaryStr {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_str().cmp(other.as_str())
	}
}

impl PartialOrd for XsdBase64BinaryStr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Default for XsdBase64BinaryString {
	fn default() -> Self {
		unsafe { Self::new_unchecked(String::new()) }
	}
}

impl PartialOrd for XsdBase64BinaryString {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for XsdBase64BinaryString {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_xsd_base64_binary_str()
			.cmp(other.as_xsd_base64_binary_str())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_empty() {
		assert!(XsdBase64BinaryStr::new("").is_ok());
	}

	#[test]
	fn parse_full_quad() {
		// "Man" encoded as base64, with no padding required.
		assert!(XsdBase64BinaryStr::new("TWFu").is_ok());
	}

	/// A `Padded16` group (two octets) ends with a `B16char`, whose
	/// 6-bit value must end in `00`. See:
	/// <https://www.w3.org/TR/xmlschema11-2/#nt-Padded16>.
	#[test]
	fn parse_padded16() {
		// "Ma" encoded as base64.
		assert!(XsdBase64BinaryStr::new("TWE=").is_ok());
	}

	/// A `Padded8` group (one octet) ends with a `B04char`, whose 6-bit
	/// value must end in `0000`. See:
	/// <https://www.w3.org/TR/xmlschema11-2/#nt-Padded8>.
	#[test]
	fn parse_padded8() {
		// "M" encoded as base64.
		assert!(XsdBase64BinaryStr::new("TQ==").is_ok());
	}

	/// `F` is not a valid `B16char` (its 6-bit value does not end in
	/// `00`), so this is not a well-formed `Padded16` group, even though
	/// it uses only characters from the base64 alphabet.
	#[test]
	fn parse_padded16_invalid_char_rejected() {
		assert!(XsdBase64BinaryStr::new("TWF=").is_err());
	}

	/// `F` is not a valid `B04char` (its 6-bit value does not end in
	/// `0000`), so this is not a well-formed `Padded8` group.
	#[test]
	fn parse_padded8_invalid_char_rejected() {
		assert!(XsdBase64BinaryStr::new("TF==").is_err());
	}

	/// `B16char`/`B04char` are case-sensitive: lowercase `e` is not a
	/// substitute for uppercase `E`.
	#[test]
	fn parse_padded16_wrong_case_rejected() {
		assert!(XsdBase64BinaryStr::new("TWe=").is_err());
	}

	/// A final group of characters that isn't padded and isn't a full
	/// quad is not a valid `B64final`.
	#[test]
	fn parse_incomplete_group_rejected() {
		assert!(XsdBase64BinaryStr::new("TWE").is_err());
	}
}
