use crate::{Datatype, XsdDatatype};
use core::fmt;

#[derive(Debug, Clone)]
pub struct Notation(());

impl XsdDatatype for Notation {
	fn type_(&self) -> Datatype {
		Datatype::Notation
	}
}

impl fmt::Display for Notation {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
