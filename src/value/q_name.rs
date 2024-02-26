pub use crate::lexical::{QName, QNameBuf};
use crate::ParseRdf;

impl ParseRdf for QNameBuf {
	type LexicalForm = QName;
}
