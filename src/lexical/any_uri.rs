use iref::{InvalidIri, Iri, IriBuf};

use super::{Lexical, LexicalFormOf};

impl Lexical for Iri {
	type Error = InvalidIri<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Iri::new(value).map_err(|_| InvalidIri(value.to_owned()))
	}
}

impl LexicalFormOf<IriBuf> for Iri {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<IriBuf, Self::ValueError> {
		Ok(self.to_owned())
	}
}
