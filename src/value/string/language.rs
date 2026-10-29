/// Language value.
///
/// Value space representation of the `language` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#language>.
pub use crate::lexical::Language;
pub use crate::lexical::{InvalidLanguage, LanguageBuf};

impl<'a> crate::XsdFromStr<'a> for LanguageBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<Language, Self>(lexical_value)
	}
}

impl crate::XsdTyped for LanguageBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::TokenDatatype::Language.into()
	}
}

impl crate::XsdValue for LanguageBuf {
	type LexicalForm = Language;

	fn to_lexical_form(&self) -> LanguageBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	use super::LanguageBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "en-US";
		let value = LanguageBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert!(LanguageBuf::xsd_from_str(lexical.as_str()).unwrap() == value);
	}
}
