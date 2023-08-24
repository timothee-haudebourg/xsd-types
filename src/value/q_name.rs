use crate::{Datatype, XsdDatatype};
use core::fmt;

#[derive(Debug, Clone)]
pub struct QName(());

impl XsdDatatype for QName {
	fn type_(&self) -> Datatype {
		Datatype::QName
	}
}

impl fmt::Display for QName {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
