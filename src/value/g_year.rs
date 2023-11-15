use crate::{Datatype, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct GYear(());

impl XsdValue for GYear {
	fn datatype(&self) -> Datatype {
		Datatype::GYear
	}
}

impl fmt::Display for GYear {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
