use crate::lexical::XsdLexicalForm;
use crate::{InvalidXsdToken, XsdError, XsdToken};

impl XsdLexicalForm for XsdToken {
	type Value = crate::XsdTokenBuf;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdToken {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdToken::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdToken(value)))
	}
}

impl<'a> TryFrom<&'a XsdToken> for crate::XsdTokenBuf {
	type Error = InvalidXsdToken<String>;

	fn try_from(value: &'a XsdToken) -> Result<crate::XsdTokenBuf, Self::Error> {
		value.as_str().parse()
	}
}
