use iref::{InvalidUri, Uri, UriBuf};

use super::{Lexical, LexicalFormOf};

impl Lexical for Uri {
	type Error = InvalidUri<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Uri::new(value).map_err(|_| InvalidUri(value.to_owned()))
	}
}

impl LexicalFormOf<UriBuf> for Uri {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<UriBuf, Self::ValueError> {
		Ok(self.to_owned())
	}
}
