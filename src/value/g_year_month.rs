use crate::{Datatype, XsdDatatype};
use core::fmt;

#[derive(Debug, Clone)]
pub struct GYearMonth(());

impl XsdDatatype for GYearMonth {
	fn type_(&self) -> Datatype {
		Datatype::GYearMonth
	}
}

impl fmt::Display for GYearMonth {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
