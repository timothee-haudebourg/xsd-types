use static_automata::Validate;
use str_newtype::StrNewType;

use crate::{lexical::XsdLexicalForm, XsdError};

/// Language.
///
/// See: <https://www.w3.org/TR/xmlschema11-2/#language>.
///
/// ```abnf
/// language = 1*8ALPHA *("-" 1*8(ALPHA / DIGIT))
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::Language)]
#[newtype(owned(LanguageBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct Language(str);

impl XsdLexicalForm for Language {
	type Value = LanguageBuf;
}

impl<'a> crate::XsdFromStr<'a> for &'a Language {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		Language::new(value).map_err(|_| XsdError::lexical_form(InvalidLanguage(value)))
	}
}

impl<'a> TryFrom<&'a Language> for LanguageBuf {
	type Error = InvalidLanguage<String>;

	fn try_from(value: &'a Language) -> Result<LanguageBuf, Self::Error> {
		value.as_str().parse()
	}
}
