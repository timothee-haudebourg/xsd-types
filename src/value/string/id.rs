/// ID value.
///
/// Value space representation of the `ID` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#ID>.
pub use crate::lexical::Id;
pub use crate::lexical::{IdBuf, InvalidId};

impl<'a> crate::XsdFromStr<'a> for IdBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<Id, Self>(lexical_value)
	}
}

impl crate::XsdTyped for IdBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::NCNameDatatype::Id.into()
	}
}

impl crate::XsdValue for IdBuf {
	type LexicalForm = Id;

	fn to_lexical_form(&self) -> IdBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	use super::IdBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "id-1.a_b";
		let value = IdBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert!(IdBuf::xsd_from_str(lexical.as_str()).unwrap() == value);
	}
}
