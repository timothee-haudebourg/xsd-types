/// Name value.
///
/// Value space representation of the `Name` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#Name>.
pub use crate::lexical::Name;
pub use crate::lexical::{InvalidName, NameBuf};

impl<'a> crate::XsdFromStr<'a> for NameBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<Name, Self>(lexical_value)
	}
}

impl crate::XsdTyped for NameBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::NameDatatype::Name.into()
	}
}

impl crate::XsdValue for NameBuf {
	type LexicalForm = Name;

	fn to_lexical_form(&self) -> NameBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	use super::NameBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "xs:name-1.a_b";
		let value = NameBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert!(NameBuf::xsd_from_str(lexical.as_str()).unwrap() == value);
	}
}
