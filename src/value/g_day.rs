use crate::{
	Datatype, XsdFromStr, XsdTyped, XsdValueDomain, format_timezone, is_valid_offset,
	lexical::InvalidXsdGDayStr, seven_property_model_date_time, seven_property_model_eq,
	seven_property_model_partial_cmp,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

/// A recurring day of the month, optionally with a timezone offset.
///
/// This is the value space of the XSD `gDay` datatype: a decoded day number
/// paired with an optional XSD timezone offset, as opposed to its lexical
/// (string) representation, [`lexical::XsdGDayStr`](crate::lexical::XsdGDayStr).
/// Equality and ordering follow XSD's partial order over timezone offsets: an
/// absent offset stands for the full `-14:00..=+14:00` range, so two days
/// can compare as neither equal nor ordered.
/// See: <https://www.w3.org/TR/xmlschema11-2/#gDay>.
#[derive(Debug, Clone, Copy)]
pub struct XsdGDay {
	day: u8,
	offset: Option<time::UtcOffset>,
}

impl XsdGDay {
	/// Creates a new `XsdGDay`, or returns `None` if `day` is outside `1..=31`,
	/// or `offset` is outside the `-14:00..=+14:00` range permitted by XSD.
	pub fn new(day: u8, offset: Option<time::UtcOffset>) -> Option<Self> {
		if (1..=31).contains(&day) && offset.is_none_or(is_valid_offset) {
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

	/// Deconstructs this `XsdGDay` into its day and timezone offset.
	pub fn into_parts(self) -> (u8, Option<time::UtcOffset>) {
		(self.day, self.offset)
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		seven_property_model_date_time(None, None, Some(self.day), time::Time::MIDNIGHT)
	}
}

impl FromStr for XsdGDay {
	type Err = InvalidXsdGDayStr<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value =
			crate::lexical::XsdGDayStr::new(s).map_err(|_| InvalidXsdGDayStr(s.to_owned()))?;
		Ok(lexical_value.into())
	}
}

impl PartialEq for XsdGDay {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for XsdGDay {}

impl Hash for XsdGDay {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.day.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for XsdGDay {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		seven_property_model_partial_cmp(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl XsdTyped for XsdGDay {
	fn datatype(&self) -> Datatype {
		Datatype::GDay
	}
}

impl XsdValueDomain for XsdGDay {
	type LexicalForm = crate::lexical::XsdGDayStr;

	fn to_lexical_form(&self) -> crate::lexical::XsdGDayString {
		// `Display` emits the canonical lexical form by contract, so it is
		// always valid and this cannot panic.
		crate::lexical::XsdGDayString::new(self.to_string())
			.expect("Display emits a valid gDay lexical form")
	}
}

impl<'a> XsdFromStr<'a> for XsdGDay {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<crate::lexical::XsdGDayStr, Self>(lexical_value)
	}
}

impl fmt::Display for XsdGDay {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "---{:02}", self.day)?;

		format_timezone(self.offset, f)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdGDay {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdGDay {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdGDay;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#gDay")
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
	use super::XsdGDay;
	use crate::XsdValueDomain;

	#[test]
	fn to_lexical_form_roundtrip() {
		for s in ["---01", "---31Z", "---15-14:00", "---09+05:30"] {
			let value: XsdGDay = s.parse().unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), s);
			assert_eq!(lexical.as_str().parse::<XsdGDay>().unwrap(), value);
		}
	}

	fn offset(hours: i8) -> time::UtcOffset {
		time::UtcOffset::from_hms(hours, 0, 0).unwrap()
	}

	#[test]
	fn ord_without_offset_is_direct() {
		let a = XsdGDay::new(15, None).unwrap();
		let b = XsdGDay::new(16, None).unwrap();

		assert!(a < b);
	}

	// The four worked examples from
	// <https://www.w3.org/TR/xmlschema11-2/#gDay>.
	#[test]
	fn spec_worked_example() {
		let d15 = XsdGDay::new(15, None).unwrap();
		let d16 = XsdGDay::new(16, None).unwrap();
		assert!(d15 < d16);

		let d15_minus13 = XsdGDay::new(15, Some(offset(-13))).unwrap();
		let d16_plus13 = XsdGDay::new(16, Some(offset(13))).unwrap();
		assert!(d15_minus13 > d16_plus13);

		let d15_minus11 = XsdGDay::new(15, Some(offset(-11))).unwrap();
		assert_eq!(d15_minus11, d16_plus13);

		assert_eq!(d15_minus13.partial_cmp(&d16), None);
	}

	#[test]
	fn from_str_roundtrip() {
		let d: XsdGDay = "---15+05:00".parse().unwrap();
		assert_eq!(d.day(), 15);
		assert_eq!(d.offset(), Some(offset(5)));
		assert_eq!(d.to_string(), "---15+05:00");
	}

	#[test]
	fn new_rejects_out_of_range_offset() {
		assert!(XsdGDay::new(15, Some(offset(15))).is_none());
	}
}
