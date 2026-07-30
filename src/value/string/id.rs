use crate::ParseXsd;

/// ID value.
///
/// Value space representation of the `ID` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#ID>.
pub use crate::lexical::Id;
pub use crate::lexical::{IdBuf, InvalidId};

impl ParseXsd for IdBuf {
	type LexicalForm = crate::lexical::Id;
}
