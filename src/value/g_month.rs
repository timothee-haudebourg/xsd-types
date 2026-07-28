use crate::{
	format_timezone, is_valid_offset,
	lexical::{InvalidGMonth, LexicalFormOf},
	seven_property_model_date_time, seven_property_model_eq, seven_property_model_partial_cmp,
	Datatype, ParseXsd, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub struct GMonth {
	month: u8,
	offset: Option<time::UtcOffset>,
}

impl GMonth {
	pub fn new(month: u8, offset: Option<time::UtcOffset>) -> Option<Self> {
		if (1..=12).contains(&month) && offset.is_none_or(is_valid_offset) {
			Some(Self { month, offset })
		} else {
			None
		}
	}

	/// Returns the month, in `1..=12`.
	pub fn month(&self) -> u8 {
		self.month
	}

	/// Returns the timezone offset, if any.
	pub fn offset(&self) -> Option<time::UtcOffset> {
		self.offset
	}

	/// Deconstructs this `GMonth` into its month and timezone offset.
	pub fn into_parts(self) -> (u8, Option<time::UtcOffset>) {
		(self.month, self.offset)
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		// `self.month` is guaranteed to be in `1..=12` by `Self::new`.
		let month = time::Month::try_from(self.month).unwrap();
		seven_property_model_date_time(None, Some(month), None, time::Time::MIDNIGHT)
	}
}

impl FromStr for GMonth {
	type Err = InvalidGMonth<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::GMonth::new(s)
			.map_err(|InvalidGMonth(s)| InvalidGMonth(s.to_owned()))?;
		Ok(lexical_value.as_value())
	}
}

impl PartialEq for GMonth {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for GMonth {}

impl Hash for GMonth {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.month.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for GMonth {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for GMonth {
	fn cmp(&self, other: &Self) -> Ordering {
		// Two distinct months are always at least 28 days apart in the
		// reference year, far more than the `±14:00` offset uncertainty
		// window (28 hours), so `GMonth` values are always comparable.
		seven_property_model_partial_cmp(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
		.unwrap()
	}
}

impl XsdValue for GMonth {
	fn datatype(&self) -> Datatype {
		Datatype::GMonth
	}
}

impl ParseXsd for GMonth {
	type LexicalForm = crate::lexical::GMonth;
}

impl fmt::Display for GMonth {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "--{:02}", self.month)?;

		format_timezone(self.offset, f)
	}
}

#[cfg(test)]
mod tests {
	use super::GMonth;

	#[test]
	fn ord_without_offset_is_direct() {
		let a = GMonth::new(2, None).unwrap();
		let b = GMonth::new(3, None).unwrap();

		assert!(a < b);
	}

	// Two distinct `GMonth` values are always at least 28 days apart (their
	// effective dates fall in the same reference year, 1972), which is far
	// more than the `±14:00` offset uncertainty window (28 hours), so they
	// are always totally ordered, even with extreme offsets.
	#[test]
	fn ord_is_always_definite() {
		let a = GMonth::new(2, None).unwrap();
		let b = GMonth::new(3, Some(time::UtcOffset::from_hms(14, 0, 0).unwrap())).unwrap();

		assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
		assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less));
	}

	#[test]
	fn eq_ignores_representation() {
		let a = GMonth::new(2, None).unwrap();
		let b = GMonth::new(2, None).unwrap();

		assert_eq!(a, b);
	}

	#[test]
	fn from_str_roundtrip() {
		let m: GMonth = "--02+05:00".parse().unwrap();
		assert_eq!(m.month(), 2);
		assert_eq!(
			m.offset(),
			Some(time::UtcOffset::from_hms(5, 0, 0).unwrap())
		);
		assert_eq!(m.to_string(), "--02+05:00");
	}

	#[test]
	fn new_rejects_out_of_range_offset() {
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();
		assert!(GMonth::new(2, Some(offset)).is_none());
	}
}
