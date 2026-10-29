/// NCName value.
///
/// Value space representation of the `NCName` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#NCName>.
pub use crate::lexical::NCName;
pub use crate::lexical::{InvalidNCName, NCNameBuf};

impl<'a> crate::XsdFromStr<'a> for NCNameBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<NCName, Self>(lexical_value)
	}
}

impl crate::XsdTyped for NCNameBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::NCNameDatatype::NCName.into()
	}
}

impl crate::XsdValue for NCNameBuf {
	type LexicalForm = NCName;

	fn to_lexical_form(&self) -> NCNameBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	use super::NCNameBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "ncname-1.a_b";
		let value = NCNameBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert!(NCNameBuf::xsd_from_str(lexical.as_str()).unwrap() == value);
	}
}
