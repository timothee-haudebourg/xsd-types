use iref::{InvalidIri, Iri, IriBuf};

use super::XsdLexicalForm;
use crate::XsdError;

impl XsdLexicalForm for Iri {
	type Value = IriBuf;
}

impl<'a> crate::XsdFromStr<'a> for &'a Iri {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		Iri::new(value).map_err(|_| XsdError::lexical_form(InvalidIri(value)))
	}
}
