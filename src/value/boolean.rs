use crate::{
	lexical::{self, LexicalFormOf},
	Datatype, ParseRdf, XsdDatatype,
};

pub type Boolean = bool;

impl XsdDatatype for Boolean {
	fn type_(&self) -> Datatype {
		Datatype::Boolean
	}
}

impl LexicalFormOf<Boolean> for lexical::Boolean {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<Boolean, Self::ValueError> {
		Ok(self.value())
	}
}

impl ParseRdf for Boolean {
	type LexicalForm = lexical::Boolean;
}
