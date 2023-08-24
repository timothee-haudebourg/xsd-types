use crate::{lexical::LexicalFormOf, Datatype, ParseRdf, XsdDatatype};

pub type String = std::string::String;

impl XsdDatatype for String {
	fn type_(&self) -> Datatype {
		Datatype::String(None)
	}
}

impl LexicalFormOf<String> for str {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<String, Self::ValueError> {
		Ok(self.to_string())
	}
}

impl ParseRdf for String {
	type LexicalForm = str;
}
