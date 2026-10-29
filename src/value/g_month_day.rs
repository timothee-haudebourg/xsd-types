use crate::{
	format_timezone, is_valid_offset, lexical::InvalidGMonthDay, seven_property_model_date_time,
	seven_property_model_eq, seven_property_model_partial_cmp, Datatype, XsdFromStr, XsdTyped,
	XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

const MONTH_MAX_LEN: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// A recurring month and day, optionally with a timezone offset.
///
/// This is the value space of the XSD `gMonthDay` datatype: decoded month and
/// day numbers paired with an optional XSD timezone offset, as opposed to its
/// lexical (string) representation,
/// [`lexical::GMonthDay`](crate::lexical::GMonthDay).
/// Equality and ordering follow XSD's partial order over timezone offsets: an
/// absent offset stands for the full `-14:00..=+14:00` range, so two values
/// can compare as neither equal nor ordered.
/// See: <https://www.w3.org/TR/xmlschema11-2/#gMonthDay>.
#[derive(Debug, Clone, Copy)]
pub struct GMonthDay {
	month: u8,
	day: u8,
	offset: Option<time::UtcOffset>,
}

impl GMonthDay {
	/// Creates a new `GMonthDay`, or returns `None` if `month` is not in
	/// `1..=12`, `day` is not a valid day for that month, or `offset` is
	/// outside the `-14:00..=+14:00` range permitted by XSD.
	pub fn new(month: u8, day: u8, offset: Option<time::UtcOffset>) -> Option<Self> {
		if !offset.is_none_or(is_valid_offset) {
			return None;
		}

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

	/// Returns the month, in `1..=12`.
	pub fn month(&self) -> u8 {
		self.month
	}

	/// Returns the day of the month, valid for that month.
	pub fn day(&self) -> u8 {
		self.day
	}

	/// Returns the timezone offset, if any.
	pub fn offset(&self) -> Option<time::UtcOffset> {
		self.offset
	}

	/// Deconstructs this `GMonthDay` into its month, day and timezone offset.
	pub fn into_parts(self) -> (u8, u8, Option<time::UtcOffset>) {
		(self.month, self.day, self.offset)
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		// `self.month` is guaranteed to be in `1..=12` by `Self::new`.
		let month = time::Month::try_from(self.month).unwrap();
		seven_property_model_date_time(None, Some(month), Some(self.day), time::Time::MIDNIGHT)
	}
}

impl FromStr for GMonthDay {
	type Err = InvalidGMonthDay<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value =
			crate::lexical::GMonthDay::new(s).map_err(|_| InvalidGMonthDay(s.to_owned()))?;
		Ok(lexical_value.into())
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

impl XsdTyped for GMonthDay {
	fn datatype(&self) -> Datatype {
		Datatype::GMonthDay
	}
}

impl XsdValue for GMonthDay {
	type LexicalForm = crate::lexical::GMonthDay;

	fn to_lexical_form(&self) -> crate::lexical::GMonthDayBuf {
		// `Display` emits the canonical lexical form by contract, so it is
		// always valid and this cannot panic.
		crate::lexical::GMonthDayBuf::new(self.to_string())
			.expect("Display emits a valid gMonthDay lexical form")
	}
}

impl<'a> XsdFromStr<'a> for GMonthDay {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<crate::lexical::GMonthDay, Self>(lexical_value)
	}
}

impl fmt::Display for GMonthDay {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "--{:02}-{:02}", self.month, self.day)?;

		format_timezone(self.offset, f)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for GMonthDay {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GMonthDay {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = GMonthDay;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#gMonthDay")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				v.parse().map_err(|e| E::custom(e))
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}

#[cfg(test)]
mod tests {
	use super::GMonthDay;
	use crate::XsdValue;

	#[test]
	fn to_lexical_form_roundtrip() {
		for s in ["--02-29", "--12-31Z", "--01-01-05:30", "--07-04+14:00"] {
			let value: GMonthDay = s.parse().unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), s);
			assert_eq!(lexical.as_str().parse::<GMonthDay>().unwrap(), value);
		}
	}

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

	#[test]
	fn from_str_roundtrip() {
		let d: GMonthDay = "--02-15+05:00".parse().unwrap();
		assert_eq!(d.month(), 2);
		assert_eq!(d.day(), 15);
		assert_eq!(
			d.offset(),
			Some(time::UtcOffset::from_hms(5, 0, 0).unwrap())
		);
		assert_eq!(d.to_string(), "--02-15+05:00");
	}

	#[test]
	fn new_rejects_out_of_range_offset() {
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();
		assert!(GMonthDay::new(2, 15, Some(offset)).is_none());
	}
}
