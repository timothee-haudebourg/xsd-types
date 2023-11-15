use chrono::{FixedOffset, NaiveTime};

use crate::{Datatype, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Time {
	pub time: NaiveTime,
	pub time_zone: FixedOffset,
}

impl Time {
	pub fn new(time: NaiveTime, time_zone: FixedOffset) -> Self {
		Self { time, time_zone }
	}
}

impl XsdValue for Time {
	fn datatype(&self) -> Datatype {
		Datatype::Time
	}
}

impl fmt::Display for Time {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unimplemented!()
	}
}
