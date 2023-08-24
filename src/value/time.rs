use crate::{Datatype, XsdDatatype};
use core::fmt;

#[derive(Debug, Clone)]
pub struct Time(());

impl XsdDatatype for Time {
	fn type_(&self) -> Datatype {
		Datatype::Time
	}
}

impl fmt::Display for Time {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
