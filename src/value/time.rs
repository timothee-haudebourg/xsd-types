use crate::{
	format_nanoseconds, format_timezone, seven_property_model_date_time, seven_property_model_eq,
	seven_property_model_partial_cmp, Datatype, ParseXsd, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash};

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

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		seven_property_model_date_time(None, None, None, self.time)
	}
}

impl PartialEq for Time {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for Time {}

impl Hash for Time {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.time.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for Time {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		seven_property_model_partial_cmp(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
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

#[cfg(test)]
mod tests {
	use super::Time;

	fn time(hour: u8, minute: u8, second: u8) -> time::Time {
		time::Time::from_hms(hour, minute, second).unwrap()
	}

	#[test]
	fn ord_without_offset_is_direct() {
		let a = Time::new(time(12, 0, 0), None);
		let b = Time::new(time(13, 0, 0), None);

		assert!(a < b);
	}

	#[test]
	fn ord_with_one_offset_can_be_incomparable() {
		let a = Time::new(time(12, 0, 0), None);
		let b = Time::new(time(13, 0, 0), Some(time::UtcOffset::UTC));

		assert_eq!(a.partial_cmp(&b), None);
	}

	#[test]
	fn eq_ignores_representation() {
		let a = Time::new(time(12, 0, 0), Some(time::UtcOffset::UTC));
		let b = Time::new(
			time(14, 0, 0),
			Some(time::UtcOffset::from_hms(2, 0, 0).unwrap()),
		);

		assert_eq!(a, b);
	}
}
