use chrono::{FixedOffset, NaiveTime, Timelike};

use crate::{format_nanoseconds, format_timezone, Datatype, ParseRdf, XsdValue};
use core::fmt;

#[derive(Debug, thiserror::Error)]
#[error("invalid time value")]
pub struct InvalidTimeValue;

#[derive(Debug, Clone, Copy)]
pub struct Time {
	pub time: NaiveTime,
	pub offset: Option<FixedOffset>,
}

impl Time {
	pub fn new(time: NaiveTime, offset: Option<FixedOffset>) -> Self {
		Self { time, offset }
	}
}

impl XsdValue for Time {
	fn datatype(&self) -> Datatype {
		Datatype::Time
	}
}

impl ParseRdf for Time {
	type LexicalForm = crate::lexical::Time;
}

impl fmt::Display for Time {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{:02}:{:02}:{:02}",
			self.time.hour(),
			self.time.minute(),
			self.time.second()
		)?;

		format_nanoseconds(self.time.nanosecond(), f)?;
		format_timezone(self.offset, f)
	}
}
