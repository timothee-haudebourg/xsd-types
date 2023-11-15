use crate::{Datatype, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct GMonthDay(());

impl XsdValue for GMonthDay {
	fn datatype(&self) -> Datatype {
		Datatype::GMonthDay
	}
}

impl fmt::Display for GMonthDay {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
