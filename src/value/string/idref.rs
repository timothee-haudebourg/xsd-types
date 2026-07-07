use crate::ParseXsd;

pub use crate::lexical::{IdRef, IdRefBuf, InvalidIdRef};

impl ParseXsd for IdRefBuf {
	type LexicalForm = crate::lexical::IdRef;
}
