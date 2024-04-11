use chrono::{Datelike, FixedOffset, Timelike, Utc};
use std::{cmp::Ordering, fmt, hash::Hash, str::FromStr};

use crate::{
	format_nanoseconds, format_timezone,
	lexical::{date_time::InvalidDateTimeStamp, LexicalFormOf},
	Datatype, DateTimeDatatype, DisplayYear, ParseXsd, XsdValue,
};

#[derive(Debug, thiserror::Error)]
#[error("invalid timezone")]
pub struct InvalidTimezone(chrono::NaiveDateTime, FixedOffset);

#[derive(Debug, thiserror::Error)]
#[error("invalid datetimestamp value")]
pub struct InvalidDateTimeStampValue;

#[derive(Debug, Clone, Copy)]
pub struct DateTimeStamp {
	date_time: chrono::NaiveDateTime,
	offset: FixedOffset,
}

impl DateTimeStamp {
	pub fn new(date_time: chrono::NaiveDateTime, offset: FixedOffset) -> Self {
		Self { date_time, offset }
	}

	/// Returns a `DateTimeStamp` which corresponds to the current time and
	/// date.
	pub fn now() -> Self {
		Utc::now().into()
	}

	/// Returns a `DateTimeStamp` which corresponds to the current time and
	/// date, with millisecond precision (at most).
	pub fn now_ms() -> Self {
		let now = Utc::now();
		let ms = now.timestamp_subsec_millis();
		let ns = ms * 1_000_000;
		now.with_nanosecond(ns).unwrap_or(now).into()
	}

	pub fn into_string(self) -> String {
		self.to_string()
	}

	/// Converts this `DateTimeStamp` to a `chrono::DateTime<FixedOffset>`.
	pub fn to_chrono_date_time(&self) -> chrono::DateTime<FixedOffset> {
		self.date_time.and_local_timezone(self.offset).unwrap()
	}
}

impl PartialEq for DateTimeStamp {
	fn eq(&self, other: &Self) -> bool {
		self.to_chrono_date_time() == other.to_chrono_date_time()
	}
}

impl Eq for DateTimeStamp {}

impl Hash for DateTimeStamp {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.date_time.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for DateTimeStamp {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for DateTimeStamp {
	fn cmp(&self, other: &Self) -> Ordering {
		self.to_chrono_date_time().cmp(&other.to_chrono_date_time())
	}
}

impl XsdValue for DateTimeStamp {
	fn datatype(&self) -> Datatype {
		Datatype::DateTime(DateTimeDatatype::DateTimeStamp)
	}
}

impl ParseXsd for DateTimeStamp {
	type LexicalForm = crate::lexical::DateTimeStamp;
}

impl fmt::Display for DateTimeStamp {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}-{:02}-{:02}T{:02}:{:02}:{:02}",
			DisplayYear(self.date_time.year()),
			self.date_time.month(),
			self.date_time.day(),
			self.date_time.hour(),
			self.date_time.minute(),
			self.date_time.second()
		)?;

		format_nanoseconds(self.date_time.nanosecond(), f)?;
		format_timezone(Some(self.offset), f)
	}
}

#[derive(Debug, thiserror::Error)]
pub enum DateTimeStampFromStrError {
	#[error("invalid date syntax")]
	Syntax(#[from] InvalidDateTimeStamp<String>),

	#[error(transparent)]
	Value(#[from] InvalidDateTimeStampValue),
}

impl FromStr for DateTimeStamp {
	type Err = DateTimeStampFromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::DateTimeStamp::new(s)
			.map_err(|InvalidDateTimeStamp(s)| InvalidDateTimeStamp(s.to_owned()))?;
		lexical_value.try_as_value().map_err(Into::into)
	}
}

impl From<chrono::DateTime<FixedOffset>> for DateTimeStamp {
	fn from(value: chrono::DateTime<FixedOffset>) -> Self {
		let naive_date_time = value.naive_utc();
		let offset = *value.offset();
		Self::new(naive_date_time, offset)
	}
}

impl From<chrono::DateTime<Utc>> for DateTimeStamp {
	fn from(value: chrono::DateTime<Utc>) -> Self {
		let naive_date_time = value.naive_utc();
		let offset = FixedOffset::east_opt(0).unwrap();
		Self::new(naive_date_time, offset)
	}
}

impl From<DateTimeStamp> for chrono::DateTime<FixedOffset> {
	fn from(value: DateTimeStamp) -> Self {
		value.to_chrono_date_time()
	}
}

impl From<DateTimeStamp> for chrono::DateTime<Utc> {
	fn from(value: DateTimeStamp) -> Self {
		value.to_chrono_date_time().into()
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for DateTimeStamp {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.into_string().serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DateTimeStamp {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = DateTimeStamp;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#dateTime")
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
