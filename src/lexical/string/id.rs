use crate::lexical::{Lexical, LexicalFormOf};
use crate::InvalidId;
pub use crate::{Id, IdBuf};

impl Lexical for Id {
	type Error = InvalidId<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidId(value.to_owned()))
	}
}

impl LexicalFormOf<crate::IdBuf> for Id {
	type ValueError = InvalidId<String>;

	fn try_as_value(&self) -> Result<crate::IdBuf, Self::ValueError> {
		self.as_str().parse()
	}
}
