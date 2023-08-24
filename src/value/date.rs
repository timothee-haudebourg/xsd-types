use crate::{Datatype, XsdDatatype};
use core::fmt;

#[derive(Debug, Clone)]
pub struct Date(());

impl XsdDatatype for Date {
	fn type_(&self) -> Datatype {
		Datatype::Date
	}
}

impl fmt::Display for Date {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
