use crate::{
	format_timezone, seven_property_model_date_time, seven_property_model_eq,
	seven_property_model_partial_cmp, Datatype, ParseXsd, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash};

const MONTH_MAX_LEN: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[derive(Debug, Clone, Copy)]
pub struct GMonthDay {
	month: u8,
	day: u8,
	offset: Option<time::UtcOffset>,
}

impl GMonthDay {
	pub fn new(month: u8, day: u8, offset: Option<time::UtcOffset>) -> Option<Self> {
		if month > 0 {
			let max_day = *MONTH_MAX_LEN.get(month as usize - 1)?;
			if (1..=max_day).contains(&day) {
				Some(Self { month, day, offset })
			} else {
				None
			}
		} else {
			None
		}
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		// `self.month` is guaranteed to be in `1..=12` by `Self::new`.
		let month = time::Month::try_from(self.month).unwrap();
		seven_property_model_date_time(None, Some(month), Some(self.day), time::Time::MIDNIGHT)
	}
}

impl PartialEq for GMonthDay {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for GMonthDay {}

impl Hash for GMonthDay {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.month.hash(state);
		self.day.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for GMonthDay {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		seven_property_model_partial_cmp(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl XsdValue for GMonthDay {
	fn datatype(&self) -> Datatype {
		Datatype::GMonthDay
	}
}

impl ParseXsd for GMonthDay {
	type LexicalForm = crate::lexical::GMonthDay;
}

impl fmt::Display for GMonthDay {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "--{:02}-{:02}", self.month, self.day)?;

		format_timezone(self.offset, f)
	}
}

#[cfg(test)]
mod tests {
	use super::GMonthDay;

	#[test]
	fn ord_without_offset_is_direct() {
		let a = GMonthDay::new(2, 15, None).unwrap();
		let b = GMonthDay::new(2, 16, None).unwrap();

		assert!(a < b);
	}

	#[test]
	fn ord_with_one_offset_can_be_incomparable() {
		let a = GMonthDay::new(2, 15, None).unwrap();
		let b = GMonthDay::new(2, 16, Some(time::UtcOffset::from_hms(14, 0, 0).unwrap())).unwrap();

		assert_eq!(a.partial_cmp(&b), None);
	}

	#[test]
	fn eq_ignores_representation() {
		let a = GMonthDay::new(2, 15, None).unwrap();
		let b = GMonthDay::new(2, 15, None).unwrap();

		assert_eq!(a, b);
	}
}
