use crate::ParseXsd;

/// NMTOKEN value.
///
/// Value space representation of the `NMTOKEN` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#NMTOKEN>.
pub use crate::lexical::NMToken;
pub use crate::lexical::{InvalidNMToken, NMTokenBuf};

impl ParseXsd for NMTokenBuf {
	type LexicalForm = crate::lexical::NMToken;
}
