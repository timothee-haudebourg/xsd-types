use crate::lexical::{XsdNMToken, XsdNMTokenBuf};

impl<'a> crate::XsdFromStr<'a> for XsdNMTokenBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<XsdNMToken, Self>(lexical_value)
	}
}

impl crate::XsdTyped for XsdNMTokenBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::TokenDatatype::NMToken.into()
	}
}

impl crate::XsdValue for XsdNMTokenBuf {
	type LexicalForm = XsdNMToken;

	fn to_lexical_form(&self) -> XsdNMTokenBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	use super::XsdNMTokenBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "1-token:a.b_c";
		let value = XsdNMTokenBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert!(XsdNMTokenBuf::xsd_from_str(lexical.as_str()).unwrap() == value);
	}
}
