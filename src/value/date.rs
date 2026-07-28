use crate::{
	format_timezone, is_valid_offset,
	lexical::{InvalidDate, LexicalFormOf},
	seven_property_model_eq, seven_property_model_partial_cmp, Datatype, DisplayYear, ParseXsd,
	XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

#[derive(Debug, thiserror::Error)]
#[error("invalid date value")]
pub struct InvalidDateValue;

#[derive(Debug, Clone, Copy)]
pub struct Date {
	date: time::Date,
	offset: Option<time::UtcOffset>,
}

impl Date {
	/// Creates a new `Date`, or returns `None` if `offset` is outside the
	/// `-14:00..=+14:00` range permitted by XSD.
	pub fn new(date: time::Date, offset: Option<time::UtcOffset>) -> Option<Self> {
		if offset.is_none_or(is_valid_offset) {
			Some(Self { date, offset })
		} else {
			None
		}
	}

	/// Returns the date, without its timezone offset.
	pub fn date(&self) -> time::Date {
		self.date
	}

	/// Returns the timezone offset, if any.
	pub fn offset(&self) -> Option<time::UtcOffset> {
		self.offset
	}

	/// Deconstructs this `Date` into its date and timezone offset.
	pub fn into_parts(self) -> (time::Date, Option<time::UtcOffset>) {
		(self.date, self.offset)
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		time::PrimitiveDateTime::new(self.date, time::Time::MIDNIGHT)
	}
}

impl PartialEq for Date {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for Date {}

impl Hash for Date {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.date.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for Date {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		seven_property_model_partial_cmp(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

#[derive(Debug, thiserror::Error)]
pub enum DateFromStrError {
	#[error("invalid date syntax")]
	Syntax(#[from] InvalidDate<String>),

	#[error(transparent)]
	Value(#[from] InvalidDateValue),
}

impl FromStr for Date {
	type Err = DateFromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value =
			crate::lexical::Date::new(s).map_err(|InvalidDate(s)| InvalidDate(s.to_owned()))?;
		lexical_value.try_as_value().map_err(Into::into)
	}
}

impl XsdValue for Date {
	fn datatype(&self) -> Datatype {
		Datatype::Date
	}
}

impl ParseXsd for Date {
	type LexicalForm = crate::lexical::Date;
}

impl fmt::Display for Date {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}-{:02}-{:02}",
			DisplayYear(self.date.year()),
			u8::from(self.date.month()),
			self.date.day()
		)?;

		format_timezone(self.offset, f)
	}
}

#[cfg(test)]
mod tests {
	use super::Date;

	#[test]
	fn new_rejects_out_of_range_offset() {
		let date = time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap();
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();

		assert!(Date::new(date, Some(offset)).is_none());
	}

	#[test]
	fn into_parts_roundtrip() {
		let date = time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap();
		let offset = time::UtcOffset::from_hms(2, 0, 0).unwrap();
		let d = Date::new(date, Some(offset)).unwrap();

		assert_eq!(d.into_parts(), (date, Some(offset)));
	}

	#[test]
	fn ord_without_offset_is_direct() {
		let a: Date = "2024-01-01".parse().unwrap();
		let b: Date = "2024-01-02".parse().unwrap();

		assert!(a < b);
	}

	#[test]
	fn ord_with_one_offset_can_be_incomparable() {
		// The imputed `+14:00`/`-14:00` bounds for `2024-01-01` (no offset)
		// span `[2023-12-31T10:00Z, 2024-01-01T14:00Z]`, which overlaps the
		// single instant `2024-01-02T00:00:00+14:00` resolves to
		// (`2024-01-01T10:00Z`).
		let a: Date = "2024-01-01".parse().unwrap();
		let b: Date = "2024-01-02+14:00".parse().unwrap();

		assert_eq!(a.partial_cmp(&b), None);
	}

	#[test]
	fn eq_ignores_representation() {
		let a: Date = "2024-01-01+02:00".parse().unwrap();
		let b: Date = "2024-01-01+02:00".parse().unwrap();
		let c: Date = "2024-01-01+03:00".parse().unwrap();

		assert_eq!(a, b);
		assert_ne!(a, c);
	}
}
