use crate::ParseXsd;

/// NCName value.
///
/// Value space representation of the `NCName` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#NCName>.
pub use crate::lexical::NCName;
pub use crate::lexical::{InvalidNCName, NCNameBuf};

impl ParseXsd for NCNameBuf {
	type LexicalForm = crate::lexical::NCName;
}
