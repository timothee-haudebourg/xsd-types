use chrono::FixedOffset;

use crate::{Datatype, XsdDatatype};

pub type DateTime = chrono::DateTime<FixedOffset>;

impl XsdDatatype for DateTime {
	fn type_(&self) -> Datatype {
		Datatype::DateTime
	}
}
