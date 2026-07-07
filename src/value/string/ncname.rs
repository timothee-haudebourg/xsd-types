use crate::ParseXsd;

pub use crate::lexical::{InvalidNCName, NCName, NCNameBuf};

impl ParseXsd for NCNameBuf {
	type LexicalForm = crate::lexical::NCName;
}
