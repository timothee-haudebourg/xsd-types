use crate::lexical::{Lexical, LexicalFormOf};
use crate::InvalidName;
pub use crate::{Name, NameBuf};

impl Lexical for Name {
	type Error = InvalidName<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidName(value.to_owned()))
	}
}

impl LexicalFormOf<crate::NameBuf> for Name {
	type ValueError = InvalidName<String>;

	fn try_as_value(&self) -> Result<crate::NameBuf, Self::ValueError> {
		self.as_str().parse()
	}
}
