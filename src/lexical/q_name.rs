use static_automata::Validate;
use str_newtype::StrNewType;

use crate::XsdError;

use super::XsdLexicalForm;

/// Qualified XsdName.
///
/// ```abnf
/// XsdQName = PrefixedName / UnprefixedName
///
/// PrefixedName = Prefix ":" LocalPart
///
/// UnprefixedName = LocalPart
///
/// Prefix = XsdNCName
///
/// LocalPart = XsdNCName
///
/// XsdNCName = NCNameStartChar *NCNameChar
///
/// NCNameStartChar = ALPHA / "_" / %xC0-D6 / %xD8-F6 / %xF8-2FF / %x370-37D / %x37F-1FFF / %x200C-200D / %x2070-218F / %x2C00-2FEF / %x3001-D7FF / %xF900-FDCF / %xFDF0-FFFD / %x10000-EFFFF
///
/// NCNameChar = NCNameStartChar / "-" / "." / DIGIT / %xB7 / %x0300-036F / %x203F-2040
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::QName)]
#[newtype(owned(XsdQNameBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct XsdQName(str);

impl XsdLexicalForm for XsdQName {
	type Value = XsdQNameBuf;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdQName {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdQName::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdQName(value)))
	}
}

impl<'a> From<&'a XsdQName> for XsdQNameBuf {
	fn from(value: &'a XsdQName) -> XsdQNameBuf {
		value.to_owned()
	}
}
