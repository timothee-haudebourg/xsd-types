use crate::ParseXsd;

/// Name value.
///
/// Value space representation of the `Name` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#Name>.
pub use crate::lexical::Name;
pub use crate::lexical::{InvalidName, NameBuf};

impl ParseXsd for NameBuf {
	type LexicalForm = crate::lexical::Name;
}
