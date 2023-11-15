use crate::{Datatype, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Duration(());

impl XsdValue for Duration {
	fn datatype(&self) -> Datatype {
		Datatype::Duration
	}
}

impl fmt::Display for Duration {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
