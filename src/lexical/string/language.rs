use static_automata::Validate;
use str_newtype::StrNewType;

use crate::{XsdError, lexical::XsdLexicalForm};

/// XsdLanguage.
///
/// See: <https://www.w3.org/TR/xmlschema11-2/#language>.
///
/// ```abnf
/// language = 1*8ALPHA *("-" 1*8(ALPHA / DIGIT))
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::Language)]
#[newtype(owned(XsdLanguageBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct XsdLanguage(str);

impl XsdLexicalForm for XsdLanguage {
	type Value = XsdLanguageBuf;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdLanguage {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdLanguage::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdLanguage(value)))
	}
}

impl<'a> TryFrom<&'a XsdLanguage> for XsdLanguageBuf {
	type Error = InvalidXsdLanguage<String>;

	fn try_from(value: &'a XsdLanguage) -> Result<XsdLanguageBuf, Self::Error> {
		value.as_str().parse()
	}
}
