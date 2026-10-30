use crate::lexical::{XsdLanguage, XsdLanguageBuf};

impl<'a> crate::XsdFromStr<'a> for XsdLanguageBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<XsdLanguage, Self>(lexical_value)
	}
}

impl crate::XsdTyped for XsdLanguageBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::TokenDatatype::Language.into()
	}
}

impl crate::XsdValueDomain for XsdLanguageBuf {
	type LexicalForm = XsdLanguage;

	fn to_lexical_form(&self) -> XsdLanguageBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValueDomain};

	use super::XsdLanguageBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "en-US";
		let value = XsdLanguageBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert!(XsdLanguageBuf::xsd_from_str(lexical.as_str()).unwrap() == value);
	}
}
