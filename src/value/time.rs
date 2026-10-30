use crate::{
	Datatype, XsdFromStr, XsdTyped, XsdValueDomain, format_nanoseconds, format_timezone,
	is_valid_offset, lexical::InvalidXsdTimeStr, seven_property_model_date_time,
	seven_property_model_eq, seven_property_model_partial_cmp,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

/// Error raised when the components of a [`XsdTime`] do not form a valid time
/// (e.g. an out-of-range hour/minute/second, or a timezone offset outside
/// the range permitted by XSD).
#[derive(Debug, thiserror::Error)]
#[error("invalid time value")]
pub struct InvalidTimeValue;

/// A time of day, optionally with a timezone offset.
///
/// This is the value space of the XSD `time` datatype: a decoded `time::Time`
/// paired with an optional XSD timezone offset, as opposed to its lexical
/// (string) representation, [`lexical::XsdTimeStr`](crate::lexical::XsdTimeStr).
/// Equality and ordering follow XSD's partial order over timezone offsets: an
/// absent offset stands for the full `-14:00..=+14:00` range, so two times
/// can compare as neither equal nor ordered.
/// See: <https://www.w3.org/TR/xmlschema11-2/#time>.
#[derive(Debug, Clone, Copy)]
pub struct XsdTime {
	time: time::Time,
	offset: Option<time::UtcOffset>,
}

impl XsdTime {
	/// Creates a new `XsdTime`, or returns `None` if `offset` is outside the
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

	/// Deconstructs this `XsdTime` into its time and timezone offset.
	pub fn into_parts(self) -> (time::Time, Option<time::UtcOffset>) {
		(self.time, self.offset)
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		seven_property_model_date_time(None, None, None, self.time)
	}
}

impl PartialEq for XsdTime {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for XsdTime {}

impl Hash for XsdTime {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.time.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for XsdTime {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		seven_property_model_partial_cmp(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl XsdTyped for XsdTime {
	fn datatype(&self) -> Datatype {
		Datatype::Time
	}
}

impl XsdValueDomain for XsdTime {
	type LexicalForm = crate::lexical::XsdTimeStr;

	fn to_lexical_form(&self) -> crate::lexical::XsdTimeString {
		// `Display` emits the canonical lexical form by contract, so it is
		// always valid and this cannot panic.
		crate::lexical::XsdTimeString::new(self.to_string())
			.expect("Display emits a valid time lexical form")
	}
}

impl<'a> XsdFromStr<'a> for XsdTime {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<crate::lexical::XsdTimeStr, Self>(lexical_value)
	}
}

/// Error raised when parsing a [`XsdTime`] from a string, either because the
/// input is not a syntactically valid `time` lexical representation, or
/// because it does not denote a valid time value.
#[derive(Debug, thiserror::Error)]
pub enum TimeFromStrError {
	#[error("invalid time syntax")]
	Syntax(#[from] InvalidXsdTimeStr<String>),

	#[error(transparent)]
	Value(#[from] InvalidTimeValue),
}

impl FromStr for XsdTime {
	type Err = TimeFromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value =
			crate::lexical::XsdTimeStr::new(s).map_err(|_| InvalidXsdTimeStr(s.to_owned()))?;
		Self::try_from(lexical_value).map_err(Into::into)
	}
}

impl fmt::Display for XsdTime {
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

#[cfg(feature = "serde")]
impl serde::Serialize for XsdTime {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdTime {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdTime;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#time")
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
	use super::XsdTime;
	use crate::XsdValueDomain;

	#[test]
	fn to_lexical_form_roundtrip() {
		for s in [
			"00:00:00",
			"13:07:12.5Z",
			"23:59:59.000000001-05:30",
			"12:30:45.123+14:00",
		] {
			let value: XsdTime = s.parse().unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), s);
			assert_eq!(lexical.as_str().parse::<XsdTime>().unwrap(), value);
		}
	}

	fn time(hour: u8, minute: u8, second: u8) -> time::Time {
		time::Time::from_hms(hour, minute, second).unwrap()
	}

	#[test]
	fn ord_without_offset_is_direct() {
		let a = XsdTime::new(time(12, 0, 0), None).unwrap();
		let b = XsdTime::new(time(13, 0, 0), None).unwrap();

		assert!(a < b);
	}

	#[test]
	fn ord_with_one_offset_can_be_incomparable() {
		let a = XsdTime::new(time(12, 0, 0), None).unwrap();
		let b = XsdTime::new(time(13, 0, 0), Some(time::UtcOffset::UTC)).unwrap();

		assert_eq!(a.partial_cmp(&b), None);
	}

	#[test]
	fn eq_ignores_representation() {
		let a = XsdTime::new(time(12, 0, 0), Some(time::UtcOffset::UTC)).unwrap();
		let b = XsdTime::new(
			time(14, 0, 0),
			Some(time::UtcOffset::from_hms(2, 0, 0).unwrap()),
		)
		.unwrap();

		assert_eq!(a, b);
	}

	#[test]
	fn new_rejects_out_of_range_offset() {
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();
		assert!(XsdTime::new(time(12, 0, 0), Some(offset)).is_none());
	}

	#[test]
	fn from_str_roundtrip() {
		let t: XsdTime = "13:07:12+01:00".parse().unwrap();
		assert_eq!(t.time(), time(13, 7, 12));
		assert_eq!(
			t.offset(),
			Some(time::UtcOffset::from_hms(1, 0, 0).unwrap())
		);
		assert_eq!(t.to_string(), "13:07:12+01:00");
	}
}
