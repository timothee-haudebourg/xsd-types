use crate::lexical::{Lexical, LexicalFormOf};
use crate::InvalidToken;
pub use crate::{Token, TokenBuf};

impl Lexical for Token {
	type Error = InvalidToken<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidToken(value.to_owned()))
	}
}

impl LexicalFormOf<crate::TokenBuf> for Token {
	type ValueError = InvalidToken<String>;

	fn try_as_value(&self) -> Result<crate::TokenBuf, Self::ValueError> {
		self.as_str().parse()
	}
}
