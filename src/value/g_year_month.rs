use crate::{Datatype, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct GYearMonth(());

impl XsdValue for GYearMonth {
	fn datatype(&self) -> Datatype {
		Datatype::GYearMonth
	}
}

impl fmt::Display for GYearMonth {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
