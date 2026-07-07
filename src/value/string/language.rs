use crate::ParseXsd;

pub use crate::lexical::{InvalidLanguage, Language, LanguageBuf};

impl ParseXsd for LanguageBuf {
	type LexicalForm = crate::lexical::Language;
}
