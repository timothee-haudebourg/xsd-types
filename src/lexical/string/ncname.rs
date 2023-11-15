use crate::lexical::{Lexical, LexicalFormOf};
use crate::InvalidNCName;
pub use crate::{NCName, NCNameBuf};

impl Lexical for NCName {
	type Error = InvalidNCName<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidNCName(value.to_owned()))
	}
}

impl LexicalFormOf<crate::NCNameBuf> for NCName {
	type ValueError = InvalidNCName<String>;

	fn try_as_value(&self) -> Result<crate::NCNameBuf, Self::ValueError> {
		self.as_str().parse()
	}
}
