use crate::ParseXsd;

pub use crate::lexical::{InvalidNMToken, NMToken, NMTokenBuf};

impl ParseXsd for NMTokenBuf {
	type LexicalForm = crate::lexical::NMToken;
}
