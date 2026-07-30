use static_automata::Validate;
use str_newtype::StrNewType;

use crate::lexical::{Lexical, LexicalFormOf};

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

impl Lexical for Language {
	type Error = InvalidLanguage<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidLanguage(value.to_owned()))
	}
}

impl LexicalFormOf<LanguageBuf> for Language {
	type ValueError = InvalidLanguage<String>;

	fn try_as_value(&self) -> Result<LanguageBuf, Self::ValueError> {
		self.as_str().parse()
	}
}
