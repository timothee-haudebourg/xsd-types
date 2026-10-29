use crate::lexical::XsdLexicalForm;
use crate::{InvalidToken, XsdError};
pub use crate::{Token, TokenBuf};

impl XsdLexicalForm for Token {
	type Value = crate::TokenBuf;
}

impl<'a> crate::XsdFromStr<'a> for &'a Token {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		Token::new(value).map_err(|_| XsdError::lexical_form(InvalidToken(value)))
	}
}

impl<'a> TryFrom<&'a Token> for crate::TokenBuf {
	type Error = InvalidToken<String>;

	fn try_from(value: &'a Token) -> Result<crate::TokenBuf, Self::Error> {
		value.as_str().parse()
	}
}
