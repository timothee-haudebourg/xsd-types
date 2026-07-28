use crate::{
	format_timezone, is_valid_offset,
	lexical::{InvalidGYearMonth, LexicalFormOf},
	seven_property_model_date_time, seven_property_model_eq, seven_property_model_partial_cmp,
	Datatype, DisplayYear, ParseXsd, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub struct GYearMonth {
	year: i32,
	month: u8,
	offset: Option<time::UtcOffset>,
}

impl GYearMonth {
	pub fn new(year: i32, month: u8, offset: Option<time::UtcOffset>) -> Option<Self> {
		if (1..=12).contains(&month) && offset.is_none_or(is_valid_offset) {
			Some(Self {
				year,
				month,
				offset,
			})
		} else {
			None
		}
	}

	/// Returns the year.
	pub fn year(&self) -> i32 {
		self.year
	}

	/// Returns the month, in `1..=12`.
	pub fn month(&self) -> u8 {
		self.month
	}

	/// Returns the timezone offset, if any.
	pub fn offset(&self) -> Option<time::UtcOffset> {
		self.offset
	}

	/// Deconstructs this `GYearMonth` into its year, month and timezone
	/// offset.
	pub fn into_parts(self) -> (i32, u8, Option<time::UtcOffset>) {
		(self.year, self.month, self.offset)
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		// `self.month` is guaranteed to be in `1..=12` by `Self::new`.
		let month = time::Month::try_from(self.month).unwrap();
		seven_property_model_date_time(Some(self.year), Some(month), None, time::Time::MIDNIGHT)
	}
}

impl FromStr for GYearMonth {
	type Err = InvalidGYearMonth<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::GYearMonth::new(s)
			.map_err(|InvalidGYearMonth(s)| InvalidGYearMonth(s.to_owned()))?;
		Ok(lexical_value.as_value())
	}
}

impl PartialEq for GYearMonth {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for GYearMonth {}

impl Hash for GYearMonth {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.year.hash(state);
		self.month.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for GYearMonth {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for GYearMonth {
	fn cmp(&self, other: &Self) -> Ordering {
		// Two distinct (year, month) pairs are always at least 28 days apart,
		// far more than the `±14:00` offset uncertainty window (28 hours), so
		// `GYearMonth` values are always comparable.
		seven_property_model_partial_cmp(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
		.unwrap()
	}
}

impl XsdValue for GYearMonth {
	fn datatype(&self) -> Datatype {
		Datatype::GYearMonth
	}
}

impl ParseXsd for GYearMonth {
	type LexicalForm = crate::lexical::GYearMonth;
}

impl fmt::Display for GYearMonth {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}-{:02}", DisplayYear(self.year), self.month)?;

		format_timezone(self.offset, f)
	}
}

#[cfg(test)]
mod tests {
	use super::GYearMonth;

	#[test]
	fn ord_without_offset_is_direct() {
		let a = GYearMonth::new(2024, 2, None).unwrap();
		let b = GYearMonth::new(2024, 3, None).unwrap();

		assert!(a < b);
	}

	// Two distinct `GYearMonth` values are always at least 28 days apart,
	// far more than the `±14:00` offset uncertainty window (28 hours), so
	// they are always totally ordered, even with extreme offsets.
	#[test]
	fn ord_is_always_definite() {
		let a = GYearMonth::new(2024, 2, None).unwrap();
		let b =
			GYearMonth::new(2024, 3, Some(time::UtcOffset::from_hms(14, 0, 0).unwrap())).unwrap();

		assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
		assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less));
	}

	#[test]
	fn eq_ignores_representation() {
		let a = GYearMonth::new(2024, 2, None).unwrap();
		let b = GYearMonth::new(2024, 2, None).unwrap();

		assert_eq!(a, b);
	}

	#[test]
	fn from_str_roundtrip() {
		let ym: GYearMonth = "2024-02+05:00".parse().unwrap();
		assert_eq!(ym.year(), 2024);
		assert_eq!(ym.month(), 2);
		assert_eq!(
			ym.offset(),
			Some(time::UtcOffset::from_hms(5, 0, 0).unwrap())
		);
		assert_eq!(ym.to_string(), "2024-02+05:00");
	}

	#[test]
	fn new_rejects_out_of_range_offset() {
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();
		assert!(GYearMonth::new(2024, 2, Some(offset)).is_none());
	}

	#[test]
	fn into_parts_roundtrip() {
		let offset = time::UtcOffset::from_hms(5, 0, 0).unwrap();
		let ym = GYearMonth::new(2024, 2, Some(offset)).unwrap();

		assert_eq!(ym.into_parts(), (2024, 2, Some(offset)));
	}
}
