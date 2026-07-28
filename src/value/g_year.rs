use crate::{
	format_timezone, seven_property_model_date_time, seven_property_model_eq,
	seven_property_model_partial_cmp, Datatype, DisplayYear, ParseXsd, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash};

#[derive(Debug, Clone, Copy)]
pub struct GYear {
	year: i32,
	offset: Option<time::UtcOffset>,
}

impl GYear {
	pub fn new(year: i32, offset: Option<time::UtcOffset>) -> Self {
		Self { year, offset }
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		seven_property_model_date_time(Some(self.year), None, None, time::Time::MIDNIGHT)
	}
}

impl PartialEq for GYear {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for GYear {}

impl Hash for GYear {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.year.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for GYear {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for GYear {
	fn cmp(&self, other: &Self) -> Ordering {
		// Two distinct years are always roughly a year apart, far more than
		// the `±14:00` offset uncertainty window (28 hours), so `GYear`
		// values are always comparable.
		seven_property_model_partial_cmp(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
		.unwrap()
	}
}

impl XsdValue for GYear {
	fn datatype(&self) -> Datatype {
		Datatype::GYear
	}
}

impl ParseXsd for GYear {
	type LexicalForm = crate::lexical::GYear;
}

impl fmt::Display for GYear {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		DisplayYear(self.year).fmt(f)?;
		format_timezone(self.offset, f)
	}
}

#[cfg(test)]
mod tests {
	use super::GYear;

	#[test]
	fn ord_without_offset_is_direct() {
		let a = GYear::new(2023, None);
		let b = GYear::new(2024, None);

		assert!(a < b);
	}

	// Two distinct `GYear` values are always roughly a year apart, far more
	// than the `±14:00` offset uncertainty window (28 hours), so they are
	// always totally ordered, even with extreme offsets.
	#[test]
	fn ord_is_always_definite() {
		let a = GYear::new(2023, None);
		let b = GYear::new(2024, Some(time::UtcOffset::from_hms(14, 0, 0).unwrap()));

		assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
		assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less));
	}

	#[test]
	fn eq_ignores_representation() {
		let a = GYear::new(2023, None);
		let b = GYear::new(2023, None);

		assert_eq!(a, b);
	}
}
