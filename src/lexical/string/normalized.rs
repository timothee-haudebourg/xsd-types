use crate::lexical::XsdLexicalForm;
use crate::{InvalidNormalizedStr, XsdError};
pub use crate::{XsdNormalizedStr, XsdNormalizedString};

impl XsdLexicalForm for XsdNormalizedStr {
	type Value = crate::XsdNormalizedString;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdNormalizedStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdNormalizedStr::new(value)
			.map_err(|_| XsdError::lexical_form(InvalidNormalizedStr(value)))
	}
}

impl<'a> TryFrom<&'a XsdNormalizedStr> for crate::XsdNormalizedString {
	type Error = InvalidNormalizedStr<String>;

	fn try_from(value: &'a XsdNormalizedStr) -> Result<crate::XsdNormalizedString, Self::Error> {
		value.as_str().parse()
	}
}
