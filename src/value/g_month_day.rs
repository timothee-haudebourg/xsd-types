use crate::{Datatype, XsdDatatype};
use core::fmt;

#[derive(Debug, Clone)]
pub struct GMonthDay(());

impl XsdDatatype for GMonthDay {
	fn type_(&self) -> Datatype {
		Datatype::GMonthDay
	}
}

impl fmt::Display for GMonthDay {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
