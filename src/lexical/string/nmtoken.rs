use crate::lexical::{Lexical, LexicalFormOf};
use crate::InvalidNMToken;
pub use crate::{NMToken, NMTokenBuf};

impl Lexical for NMToken {
	type Error = InvalidNMToken<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidNMToken(value.to_owned()))
	}
}

impl LexicalFormOf<crate::NMTokenBuf> for NMToken {
	type ValueError = InvalidNMToken<String>;

	fn try_as_value(&self) -> Result<crate::NMTokenBuf, Self::ValueError> {
		self.as_str().parse()
	}
}
