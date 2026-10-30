use crate::lexical::{XsdId, XsdIdBuf};

impl<'a> crate::XsdFromStr<'a> for XsdIdBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<XsdId, Self>(lexical_value)
	}
}

impl crate::XsdTyped for XsdIdBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::NCNameDatatype::Id.into()
	}
}

impl crate::XsdValueDomain for XsdIdBuf {
	type LexicalForm = XsdId;

	fn to_lexical_form(&self) -> XsdIdBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValueDomain};

	use super::XsdIdBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "id-1.a_b";
		let value = XsdIdBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert!(XsdIdBuf::xsd_from_str(lexical.as_str()).unwrap() == value);
	}
}
