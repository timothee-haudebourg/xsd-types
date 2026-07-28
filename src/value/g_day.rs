use crate::{
	format_timezone,
	lexical::{InvalidGDay, LexicalFormOf},
	seven_property_model_date_time, seven_property_model_eq, seven_property_model_partial_cmp,
	Datatype, ParseXsd, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub struct GDay {
	day: u8,
	offset: Option<time::UtcOffset>,
}

impl GDay {
	pub fn new(day: u8, offset: Option<time::UtcOffset>) -> Option<Self> {
		if (1..=31).contains(&day) {
			Some(Self { day, offset })
		} else {
			None
		}
	}

	/// Returns the day of the month, in `1..=31`.
	pub fn day(&self) -> u8 {
		self.day
	}

	/// Returns the timezone offset, if any.
	pub fn offset(&self) -> Option<time::UtcOffset> {
		self.offset
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		seven_property_model_date_time(None, None, Some(self.day), time::Time::MIDNIGHT)
	}
}

impl FromStr for GDay {
	type Err = InvalidGDay<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value =
			crate::lexical::GDay::new(s).map_err(|InvalidGDay(s)| InvalidGDay(s.to_owned()))?;
		Ok(lexical_value.as_value())
	}
}

impl PartialEq for GDay {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for GDay {}

impl Hash for GDay {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.day.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for GDay {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		seven_property_model_partial_cmp(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl XsdValue for GDay {
	fn datatype(&self) -> Datatype {
		Datatype::GDay
	}
}

impl ParseXsd for GDay {
	type LexicalForm = crate::lexical::GDay;
}

impl fmt::Display for GDay {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "---{:02}", self.day)?;

		format_timezone(self.offset, f)
	}
}

#[cfg(test)]
mod tests {
	use super::GDay;

	fn offset(hours: i8) -> time::UtcOffset {
		time::UtcOffset::from_hms(hours, 0, 0).unwrap()
	}

	#[test]
	fn ord_without_offset_is_direct() {
		let a = GDay::new(15, None).unwrap();
		let b = GDay::new(16, None).unwrap();

		assert!(a < b);
	}

	// The four worked examples from
	// <https://www.w3.org/TR/xmlschema11-2/#gDay>.
	#[test]
	fn spec_worked_example() {
		let d15 = GDay::new(15, None).unwrap();
		let d16 = GDay::new(16, None).unwrap();
		assert!(d15 < d16);

		let d15_minus13 = GDay::new(15, Some(offset(-13))).unwrap();
		let d16_plus13 = GDay::new(16, Some(offset(13))).unwrap();
		assert!(d15_minus13 > d16_plus13);

		let d15_minus11 = GDay::new(15, Some(offset(-11))).unwrap();
		assert_eq!(d15_minus11, d16_plus13);

		assert_eq!(d15_minus13.partial_cmp(&d16), None);
	}

	#[test]
	fn from_str_roundtrip() {
		let d: GDay = "---15+05:00".parse().unwrap();
		assert_eq!(d.day(), 15);
		assert_eq!(d.offset(), Some(offset(5)));
		assert_eq!(d.to_string(), "---15+05:00");
	}
}
