use crate::{Datatype, XsdValue};
use core::fmt;

#[derive(Debug, Clone)]
pub struct Notation(());

impl XsdValue for Notation {
	fn datatype(&self) -> Datatype {
		Datatype::Notation
	}
}

impl fmt::Display for Notation {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
