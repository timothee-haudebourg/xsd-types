use crate::ParseXsd;

pub use crate::lexical::{InvalidName, Name, NameBuf};

impl ParseXsd for NameBuf {
	type LexicalForm = crate::lexical::Name;
}
