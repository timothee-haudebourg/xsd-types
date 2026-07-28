use crate::lexical::lexical_form;
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use str_newtype::StrNewType;

use crate::lexical::Lexical;

/// Base 64 string.
///
/// This is the `Base64Binary` production of the XSD 1.1 Datatypes
/// specification:
/// <https://www.w3.org/TR/xmlschema11-2/#nt-Base64Binary>.
///
/// ```abnf
/// Base64Binary = [ *B64quad B64final ]
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
#[newtype(owned(Base64BinaryBuf, derive(PartialEq, Eq)))]
pub struct Base64Binary(str);

impl Lexical for Base64Binary {
	type Error = InvalidBase64Binary<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidBase64Binary(value.to_owned()))
	}
}

lexical_form! {
	ty: Base64Binary,
	buffer: Base64BinaryBuf,
	value: crate::Base64BinaryBuf,
	error: InvalidBase64Binary,
	parent_forms: {}
}

impl Base64Binary {
	#[inline(always)]
	pub fn value(&self) -> crate::Base64BinaryBuf {
		crate::Base64BinaryBuf::decode(self.as_bytes()).unwrap()
	}
}

impl PartialEq for Base64Binary {
	fn eq(&self, other: &Self) -> bool {
		self.as_str() == other.as_str()
	}
}

impl Eq for Base64Binary {}

impl Hash for Base64Binary {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_str().hash(h)
	}
}

impl Ord for Base64Binary {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_str().cmp(other.as_str())
	}
}

impl PartialOrd for Base64Binary {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Default for Base64BinaryBuf {
	fn default() -> Self {
		unsafe { Self::new_unchecked(String::new()) }
	}
}

impl PartialOrd for Base64BinaryBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Base64BinaryBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_base64_binary().cmp(other.as_base64_binary())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_empty() {
		assert!(Base64Binary::new("").is_ok());
	}

	#[test]
	fn parse_full_quad() {
		// "Man" encoded as base64, with no padding required.
		assert!(Base64Binary::new("TWFu").is_ok());
	}

	/// A `Padded16` group (two octets) ends with a `B16char`, whose
	/// 6-bit value must end in `00`. See:
	/// <https://www.w3.org/TR/xmlschema11-2/#nt-Padded16>.
	#[test]
	fn parse_padded16() {
		// "Ma" encoded as base64.
		assert!(Base64Binary::new("TWE=").is_ok());
	}

	/// A `Padded8` group (one octet) ends with a `B04char`, whose 6-bit
	/// value must end in `0000`. See:
	/// <https://www.w3.org/TR/xmlschema11-2/#nt-Padded8>.
	#[test]
	fn parse_padded8() {
		// "M" encoded as base64.
		assert!(Base64Binary::new("TQ==").is_ok());
	}

	/// `F` is not a valid `B16char` (its 6-bit value does not end in
	/// `00`), so this is not a well-formed `Padded16` group, even though
	/// it uses only characters from the base64 alphabet.
	#[test]
	fn parse_padded16_invalid_char_rejected() {
		assert!(Base64Binary::new("TWF=").is_err());
	}

	/// `F` is not a valid `B04char` (its 6-bit value does not end in
	/// `0000`), so this is not a well-formed `Padded8` group.
	#[test]
	fn parse_padded8_invalid_char_rejected() {
		assert!(Base64Binary::new("TF==").is_err());
	}

	/// `B16char`/`B04char` are case-sensitive: lowercase `e` is not a
	/// substitute for uppercase `E`.
	#[test]
	fn parse_padded16_wrong_case_rejected() {
		assert!(Base64Binary::new("TWe=").is_err());
	}

	/// A final group of characters that isn't padded and isn't a full
	/// quad is not a valid `B64final`.
	#[test]
	fn parse_incomplete_group_rejected() {
		assert!(Base64Binary::new("TWE").is_err());
	}
}
