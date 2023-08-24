use crate::{Datatype, XsdDatatype};
use core::fmt;

#[derive(Debug, Clone)]
pub struct GYear(());

impl XsdDatatype for GYear {
	fn type_(&self) -> Datatype {
		Datatype::GYear
	}
}

impl fmt::Display for GYear {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
