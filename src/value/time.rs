use crate::{
	format_nanoseconds, format_timezone, is_valid_offset,
	lexical::{InvalidTime, Lexical, LexicalFormOf},
	seven_property_model_date_time, seven_property_model_eq, seven_property_model_partial_cmp,
	Datatype, ParseXsd, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

/// Error raised when the components of a [`Time`] do not form a valid time
/// (e.g. an out-of-range hour/minute/second, or a timezone offset outside
/// the range permitted by XSD).
#[derive(Debug, thiserror::Error)]
#[error("invalid time value")]
pub struct InvalidTimeValue;

/// A time of day, optionally with a timezone offset.
///
/// This is the value space of the XSD `time` datatype: a decoded `time::Time`
/// paired with an optional XSD timezone offset, as opposed to its lexical
/// (string) representation, [`lexical::Time`](crate::lexical::Time).
/// Equality and ordering follow XSD's partial order over timezone offsets: an
/// absent offset stands for the full `-14:00..=+14:00` range, so two times
/// can compare as neither equal nor ordered.
/// See: <https://www.w3.org/TR/xmlschema11-2/#time>.
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

/// Error raised when parsing a [`Time`] from a string, either because the
/// input is not a syntactically valid `time` lexical representation, or
/// because it does not denote a valid time value.
#[derive(Debug, thiserror::Error)]
pub enum TimeFromStrError {
	#[error("invalid time syntax")]
	Syntax(#[from] InvalidTime<String>),

	#[error(transparent)]
	Value(#[from] InvalidTimeValue),
}

impl FromStr for Time {
	type Err = TimeFromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::Time::parse(s)?;
		lexical_value.try_as_value().map_err(Into::into)
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

#[cfg(feature = "serde")]
impl serde::Serialize for Time {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Time {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = Time;

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

	#[test]
	fn from_str_roundtrip() {
		let t: Time = "13:07:12+01:00".parse().unwrap();
		assert_eq!(t.time(), time(13, 7, 12));
		assert_eq!(
			t.offset(),
			Some(time::UtcOffset::from_hms(1, 0, 0).unwrap())
		);
		assert_eq!(t.to_string(), "13:07:12+01:00");
	}
}
