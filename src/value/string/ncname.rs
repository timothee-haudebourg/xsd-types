/// NCName value.
///
/// Value space representation of the `XsdNCName` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#NCName>.
pub use crate::lexical::XsdNCName;
pub use crate::lexical::{InvalidXsdNCName, XsdNCNameBuf};

impl<'a> crate::XsdFromStr<'a> for XsdNCNameBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<XsdNCName, Self>(lexical_value)
	}
}

impl crate::XsdTyped for XsdNCNameBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::NCNameDatatype::NCName.into()
	}
}

impl crate::XsdValue for XsdNCNameBuf {
	type LexicalForm = XsdNCName;

	fn to_lexical_form(&self) -> XsdNCNameBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	use super::XsdNCNameBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "ncname-1.a_b";
		let value = XsdNCNameBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert!(XsdNCNameBuf::xsd_from_str(lexical.as_str()).unwrap() == value);
	}
}
