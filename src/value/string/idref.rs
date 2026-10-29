/// IDREF value.
///
/// Value space representation of the `IDREF` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#IDREF>.
pub use crate::lexical::IdRef;
pub use crate::lexical::{IdRefBuf, InvalidIdRef};

impl<'a> crate::XsdFromStr<'a> for IdRefBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<IdRef, Self>(lexical_value)
	}
}

impl crate::XsdTyped for IdRefBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::NCNameDatatype::IdRef.into()
	}
}

impl crate::XsdValue for IdRefBuf {
	type LexicalForm = IdRef;

	fn to_lexical_form(&self) -> IdRefBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	use super::IdRefBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "ref-1.a_b";
		let value = IdRefBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert!(IdRefBuf::xsd_from_str(lexical.as_str()).unwrap() == value);
	}
}
