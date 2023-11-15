use crate::{Datatype, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct GMonth;

impl XsdValue for GMonth {
	fn datatype(&self) -> Datatype {
		Datatype::GMonth
	}
}

impl fmt::Display for GMonth {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
