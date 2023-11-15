use crate::lexical::{Lexical, LexicalFormOf};
use crate::InvalidIdRef;
pub use crate::{IdRef, IdRefBuf};

impl Lexical for IdRef {
	type Error = InvalidIdRef<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidIdRef(value.to_owned()))
	}
}

impl LexicalFormOf<crate::IdRefBuf> for IdRef {
	type ValueError = InvalidIdRef<String>;

	fn try_as_value(&self) -> Result<crate::IdRefBuf, Self::ValueError> {
		self.as_str().parse()
	}
}
