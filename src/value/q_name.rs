/// Qualified name value.
///
/// `QName` has no distinct value-space encoding, so the lexical
/// [`QName`](crate::lexical::QName)/[`QNameBuf`](crate::lexical::QNameBuf)
/// representation is reused directly as the value type.
/// See: <https://www.w3.org/TR/xmlschema11-2/#QName>.
pub use crate::lexical::{QName, QNameBuf};
use crate::ParseXsd;

impl ParseXsd for QNameBuf {
	type LexicalForm = QName;
}
