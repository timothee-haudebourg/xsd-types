use crate::{format_nanoseconds, format_timezone, Datatype, ParseXsd, XsdValue};
use core::fmt;

#[derive(Debug, thiserror::Error)]
#[error("invalid time value")]
pub struct InvalidTimeValue;

#[derive(Debug, Clone, Copy)]
pub struct Time {
	pub time: time::Time,
	pub offset: Option<time::UtcOffset>,
}

impl Time {
	pub fn new(time: time::Time, offset: Option<time::UtcOffset>) -> Self {
		Self { time, offset }
	}
}

impl XsdValue for Time {
	fn datatype(&self) -> Datatype {
		Datatype::Time
	}
}

impl ParseXsd for Time {
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
