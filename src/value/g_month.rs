use crate::{Datatype, XsdDatatype};
use core::fmt;

#[derive(Debug, Clone)]
pub struct GMonth;

impl XsdDatatype for GMonth {
	fn type_(&self) -> Datatype {
		Datatype::GMonth
	}
}

impl fmt::Display for GMonth {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
