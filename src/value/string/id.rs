use crate::ParseXsd;

pub use crate::lexical::{Id, IdBuf, InvalidId};

impl ParseXsd for IdBuf {
	type LexicalForm = crate::lexical::Id;
}
