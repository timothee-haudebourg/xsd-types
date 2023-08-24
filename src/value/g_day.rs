use crate::{Datatype, XsdDatatype};
use core::fmt;

#[derive(Debug, Clone)]
pub struct GDay(());

impl XsdDatatype for GDay {
	fn type_(&self) -> Datatype {
		Datatype::GDay
	}
}

impl fmt::Display for GDay {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
