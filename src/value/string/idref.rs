use crate::ParseXsd;

/// IDREF value.
///
/// Value space representation of the `IDREF` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#IDREF>.
pub use crate::lexical::IdRef;
pub use crate::lexical::{IdRefBuf, InvalidIdRef};

impl ParseXsd for IdRefBuf {
	type LexicalForm = crate::lexical::IdRef;
}
