pub use crate::lexical::{QName, QNameBuf};
use crate::ParseXsd;

impl ParseXsd for QNameBuf {
	type LexicalForm = QName;
}
