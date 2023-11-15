use crate::{Datatype, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct GDay(());

impl XsdValue for GDay {
	fn datatype(&self) -> Datatype {
		Datatype::GDay
	}
}

impl fmt::Display for GDay {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
