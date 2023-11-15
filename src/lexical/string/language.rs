use crate::lexical::{Lexical, LexicalFormOf};
use crate::InvalidLanguage;
pub use crate::{Language, LanguageBuf};

impl Lexical for Language {
	type Error = InvalidLanguage<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidLanguage(value.to_owned()))
	}
}

impl LexicalFormOf<crate::LanguageBuf> for Language {
	type ValueError = InvalidLanguage<String>;

	fn try_as_value(&self) -> Result<crate::LanguageBuf, Self::ValueError> {
		self.as_str().parse()
	}
}
