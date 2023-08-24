use crate::{Datatype, XsdDatatype};
use core::fmt;

#[derive(Debug, Clone)]
pub struct DateTime(());

impl XsdDatatype for DateTime {
	fn type_(&self) -> Datatype {
		Datatype::DateTime
	}
}

impl fmt::Display for DateTime {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
