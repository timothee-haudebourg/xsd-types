use chrono::{FixedOffset, NaiveDate};

use crate::{Datatype, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Date {
	pub date: NaiveDate,
	pub offset: FixedOffset,
}

impl Date {
	pub fn new(date: NaiveDate, offset: FixedOffset) -> Self {
		Self { date, offset }
	}
}

impl XsdValue for Date {
	fn datatype(&self) -> Datatype {
		Datatype::Date
	}
}

impl fmt::Display for Date {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
