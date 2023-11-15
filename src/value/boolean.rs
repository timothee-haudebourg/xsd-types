use core::fmt;

use crate::{
	lexical::{self, LexicalFormOf},
	Datatype, ParseRdf, XsdValue,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Boolean(pub bool);

impl From<bool> for Boolean {
	fn from(value: bool) -> Self {
		Self(value)
	}
}

impl From<Boolean> for bool {
	fn from(value: Boolean) -> Self {
		value.0
	}
}

impl XsdValue for Boolean {
	fn datatype(&self) -> Datatype {
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

impl fmt::Display for Boolean {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.0 {
			write!(f, "true")
		} else {
			write!(f, "false")
		}
	}
}
