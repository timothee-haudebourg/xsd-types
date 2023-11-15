use crate::{Datatype, XsdValue};
use core::fmt;

#[derive(Debug, Clone)]
pub struct QName(());

impl XsdValue for QName {
	fn datatype(&self) -> Datatype {
		Datatype::QName
	}
}

impl fmt::Display for QName {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
