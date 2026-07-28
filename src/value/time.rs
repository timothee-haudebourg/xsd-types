use crate::{
	format_nanoseconds, format_timezone, is_valid_offset, seven_property_model_date_time,
	seven_property_model_eq, seven_property_model_partial_cmp, Datatype, ParseXsd, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash};

#[derive(Debug, thiserror::Error)]
#[error("invalid time value")]
pub struct InvalidTimeValue;

#[derive(Debug, Clone, Copy)]
pub struct Time {
	time: time::Time,
	offset: Option<time::UtcOffset>,
}

impl Time {
	/// Creates a new `Time`, or returns `None` if `offset` is outside the
	/// `-14:00..=+14:00` range permitted by XSD.
	pub fn new(time: time::Time, offset: Option<time::UtcOffset>) -> Option<Self> {
		if offset.is_none_or(is_valid_offset) {
			Some(Self { time, offset })
		} else {
			None
		}
	}

	/// Returns the time, without its timezone offset.
	pub fn time(&self) -> time::Time {
		self.time
	}

	/// Returns the timezone offset, if any.
	pub fn offset(&self) -> Option<time::UtcOffset> {
		self.offset
	}

	/// Deconstructs this `Time` into its time and timezone offset.
	pub fn into_parts(self) -> (time::Time, Option<time::UtcOffset>) {
		(self.time, self.offset)
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
		let a = Time::new(time(12, 0, 0), None).unwrap();
		let b = Time::new(time(13, 0, 0), None).unwrap();

		assert!(a < b);
	}

	#[test]
	fn ord_with_one_offset_can_be_incomparable() {
		let a = Time::new(time(12, 0, 0), None).unwrap();
		let b = Time::new(time(13, 0, 0), Some(time::UtcOffset::UTC)).unwrap();

		assert_eq!(a.partial_cmp(&b), None);
	}

	#[test]
	fn eq_ignores_representation() {
		let a = Time::new(time(12, 0, 0), Some(time::UtcOffset::UTC)).unwrap();
		let b = Time::new(
			time(14, 0, 0),
			Some(time::UtcOffset::from_hms(2, 0, 0).unwrap()),
		)
		.unwrap();

		assert_eq!(a, b);
	}

	#[test]
	fn new_rejects_out_of_range_offset() {
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();
		assert!(Time::new(time(12, 0, 0), Some(offset)).is_none());
	}
}
