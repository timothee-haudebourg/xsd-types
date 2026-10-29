use crate::lexical::XsdLexicalForm;
use crate::{InvalidNormalizedStr, XsdError};
pub use crate::{NormalizedStr, NormalizedString};

impl XsdLexicalForm for NormalizedStr {
	type Value = crate::NormalizedString;
}

impl<'a> crate::XsdFromStr<'a> for &'a NormalizedStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		NormalizedStr::new(value).map_err(|_| XsdError::lexical_form(InvalidNormalizedStr(value)))
	}
}

impl<'a> TryFrom<&'a NormalizedStr> for crate::NormalizedString {
	type Error = InvalidNormalizedStr<String>;

	fn try_from(value: &'a NormalizedStr) -> Result<crate::NormalizedString, Self::Error> {
		value.as_str().parse()
	}
}
