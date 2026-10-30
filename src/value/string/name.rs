use crate::lexical::{XsdName, XsdNameBuf};

impl<'a> crate::XsdFromStr<'a> for XsdNameBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<XsdName, Self>(lexical_value)
	}
}

impl crate::XsdTyped for XsdNameBuf {
	fn datatype(&self) -> crate::XsdDatatype {
		crate::XsdNameDatatype::Name.into()
	}
}

impl crate::XsdValueDomain for XsdNameBuf {
	type LexicalForm = XsdName;

	fn to_lexical_form(&self) -> XsdNameBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValueDomain};

	use super::XsdNameBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "xs:name-1.a_b";
		let value = XsdNameBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert!(XsdNameBuf::xsd_from_str(lexical.as_str()).unwrap() == value);
	}
}
