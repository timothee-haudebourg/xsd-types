use crate::lexical::{XsdIdRef, XsdIdRefBuf};

impl<'a> crate::XsdFromStr<'a> for XsdIdRefBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<XsdIdRef, Self>(lexical_value)
	}
}

impl crate::XsdTyped for XsdIdRefBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::NCNameDatatype::IdRef.into()
	}
}

impl crate::XsdValue for XsdIdRefBuf {
	type LexicalForm = XsdIdRef;

	fn to_lexical_form(&self) -> XsdIdRefBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	use super::XsdIdRefBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "ref-1.a_b";
		let value = XsdIdRefBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert!(XsdIdRefBuf::xsd_from_str(lexical.as_str()).unwrap() == value);
	}
}
