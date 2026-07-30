use crate::ParseXsd;

/// Language value.
///
/// Value space representation of the `language` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#language>.
pub use crate::lexical::Language;
pub use crate::lexical::{InvalidLanguage, LanguageBuf};

impl ParseXsd for LanguageBuf {
	type LexicalForm = crate::lexical::Language;
}
