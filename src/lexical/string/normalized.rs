use crate::lexical::{Lexical, LexicalFormOf};
use crate::InvalidNormalizedStr;
pub use crate::{NormalizedStr, NormalizedString};

impl Lexical for NormalizedStr {
	type Error = InvalidNormalizedStr<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidNormalizedStr(value.to_owned()))
	}
}

impl LexicalFormOf<crate::NormalizedString> for NormalizedStr {
	type ValueError = InvalidNormalizedStr<String>;

	fn try_as_value(&self) -> Result<crate::NormalizedString, Self::ValueError> {
		self.as_str().parse()
	}
}
