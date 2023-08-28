use chrono::{FixedOffset, NaiveDate};

use crate::{Datatype, XsdDatatype};
use core::fmt;

#[derive(Debug, Clone)]
pub struct Date {
	pub date: NaiveDate,
	pub offset: FixedOffset,
}

impl Date {
	pub fn new(date: NaiveDate, offset: FixedOffset) -> Self {
		Self { date, offset }
	}
}

impl XsdDatatype for Date {
	fn type_(&self) -> Datatype {
		Datatype::Date
	}
}

impl fmt::Display for Date {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
