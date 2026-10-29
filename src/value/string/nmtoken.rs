/// NMTOKEN value.
///
/// Value space representation of the `NMTOKEN` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#NMTOKEN>.
pub use crate::lexical::NMToken;
pub use crate::lexical::{InvalidNMToken, NMTokenBuf};

impl<'a> crate::XsdFromStr<'a> for NMTokenBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<NMToken, Self>(lexical_value)
	}
}

impl crate::XsdTyped for NMTokenBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::TokenDatatype::NMToken.into()
	}
}

impl crate::XsdValue for NMTokenBuf {
	type LexicalForm = NMToken;

	fn to_lexical_form(&self) -> NMTokenBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	use super::NMTokenBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "1-token:a.b_c";
		let value = NMTokenBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert!(NMTokenBuf::xsd_from_str(lexical.as_str()).unwrap() == value);
	}
}
