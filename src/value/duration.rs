use crate::{Datatype, XsdDatatype};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Duration(());

impl XsdDatatype for Duration {
	fn type_(&self) -> Datatype {
		Datatype::Duration
	}
}

impl fmt::Display for Duration {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
