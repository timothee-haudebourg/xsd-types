use static_automata::Validate;
use str_newtype::StrNewType;

use crate::XsdError;

use super::XsdLexicalForm;

/// Qualified Name.
///
/// ```abnf
/// QName = PrefixedName / UnprefixedName
///
/// PrefixedName = Prefix ":" LocalPart
///
/// UnprefixedName = LocalPart
///
/// Prefix = NCName
///
/// LocalPart = NCName
///
/// NCName = NCNameStartChar *NCNameChar
///
/// NCNameStartChar = ALPHA / "_" / %xC0-D6 / %xD8-F6 / %xF8-2FF / %x370-37D / %x37F-1FFF / %x200C-200D / %x2070-218F / %x2C00-2FEF / %x3001-D7FF / %xF900-FDCF / %xFDF0-FFFD / %x10000-EFFFF
///
/// NCNameChar = NCNameStartChar / "-" / "." / DIGIT / %xB7 / %x0300-036F / %x203F-2040
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::QName)]
#[newtype(owned(QNameBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct QName(str);

impl XsdLexicalForm for QName {
	type Value = QNameBuf;
}

impl<'a> crate::XsdFromStr<'a> for &'a QName {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		QName::new(value).map_err(|_| XsdError::lexical_form(InvalidQName(value)))
	}
}

impl<'a> From<&'a QName> for QNameBuf {
	fn from(value: &'a QName) -> QNameBuf {
		value.to_owned()
	}
}
