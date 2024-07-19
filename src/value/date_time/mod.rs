use chrono::{Datelike, FixedOffset, Timelike, Utc};
use std::{cmp::Ordering, fmt, hash::Hash, str::FromStr};

use crate::{
	lexical::{InvalidDateTime, LexicalFormOf},
	utils::div_rem,
	Datatype, DateTimeDatatype, ParseXsd, XsdValue,
};

mod date_time_stamp;
pub use date_time_stamp::*;

#[derive(Debug, thiserror::Error)]
#[error("missing timezone")]
pub struct MissingTimezone;

#[derive(Debug, thiserror::Error)]
#[error("invalid timezone")]
pub struct InvalidTimezone(chrono::NaiveDateTime, FixedOffset);

#[derive(Debug, thiserror::Error)]
pub enum TimezoneError {
	#[error(transparent)]
	Missing(#[from] MissingTimezone),

	#[error(transparent)]
	Invalid(#[from] InvalidTimezone),
}

#[derive(Debug, thiserror::Error)]
#[error("invalid datetime value")]
pub struct InvalidDateTimeValue;

#[derive(Debug, Clone, Copy)]
pub struct DateTime {
	date_time: chrono::NaiveDateTime,
	offset: Option<FixedOffset>,
}

impl DateTime {
	pub fn new(date_time: chrono::NaiveDateTime, offset: Option<FixedOffset>) -> Self {
		Self { date_time, offset }
	}

	/// Returns a `DateTime` which corresponds to the current time and date.
	pub fn now() -> Self {
		Utc::now().into()
	}

	/// Returns a `DateTime` which corresponds to the current time and date,
	/// with millisecond precision (at most).
	pub fn now_ms() -> Self {
		let now = Utc::now();
		let ms = now.timestamp_subsec_millis();
		let ns = ms * 1_000_000;
		now.with_nanosecond(ns).unwrap_or(now).into()
	}

	pub fn into_string(self) -> String {
		self.to_string()
	}

	/// Returns the earliest date/time with offset represented by this
	/// date/time.
	pub fn earliest(&self) -> chrono::DateTime<FixedOffset> {
		match self.offset {
			Some(offset) => self.date_time.and_local_timezone(offset).unwrap(),
			None => self
				.date_time
				.and_local_timezone(FixedOffset::west_opt(14 * 60 * 60).unwrap())
				.unwrap(),
		}
	}

	/// Returns the latest date/time with offset represented by this
	/// date/time.
	pub fn latest(&self) -> chrono::DateTime<FixedOffset> {
		match self.offset {
			Some(offset) => self.date_time.and_local_timezone(offset).unwrap(),
			None => self
				.date_time
				.and_local_timezone(FixedOffset::east_opt(14 * 60 * 60).unwrap())
				.unwrap(),
		}
	}
}

impl PartialEq for DateTime {
	fn eq(&self, other: &Self) -> bool {
		self.earliest() == other.earliest() && self.latest() == other.latest()
	}
}

impl Eq for DateTime {}

impl Hash for DateTime {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.date_time.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for DateTime {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (
			self.earliest().cmp(&other.latest()),
			self.latest().cmp(&other.earliest()),
		) {
			(Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
			(Ordering::Less, Ordering::Less) => Some(Ordering::Less),
			(Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
			_ => None,
		}
	}
}

impl XsdValue for DateTime {
	fn datatype(&self) -> Datatype {
		Datatype::DateTime(DateTimeDatatype::DateTime)
	}
}

impl ParseXsd for DateTime {
	type LexicalForm = crate::lexical::DateTime;
}

impl fmt::Display for DateTime {
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
		format_timezone(self.offset, f)
	}
}

pub(crate) fn format_nanoseconds(ns: u32, f: &mut fmt::Formatter) -> fmt::Result {
	let mut nano = ns % 1_000_000_000;

	if nano == 0 {
		Ok(())
	} else {
		let mut buffer = *b".000000000";
		let mut i = 10;
		let mut trailing = true;
		let mut end = 10;
		while nano > 0 {
			i -= 1;
			let (rest, d) = div_rem(nano, 10);
			nano = rest;

			if trailing {
				if d == 0 {
					end = i;
					continue;
				} else {
					trailing = false;
				}
			}

			buffer[i] = b'0' + d as u8;
		}

		let string = unsafe { std::str::from_utf8_unchecked(&buffer[..end]) };

		f.write_str(string)
	}
}

pub(crate) fn format_timezone(tz: Option<FixedOffset>, f: &mut fmt::Formatter) -> fmt::Result {
	match tz {
		Some(tz) => {
			if tz.local_minus_utc() == 0 {
				write!(f, "Z")
			} else {
				let tz = if tz.local_minus_utc() > 0 {
					write!(f, "+")?;
					tz.local_minus_utc() as u32
				} else {
					write!(f, "-")?;
					-tz.local_minus_utc() as u32
				};

				let tz_minutes = tz / 60;
				let hours = tz_minutes / 60;
				let minutes = tz_minutes % 60;
				write!(f, "{hours:02}:{minutes:02}")
			}
		}
		None => Ok(()),
	}
}

pub(crate) struct DisplayYear(pub i32);

impl fmt::Display for DisplayYear {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.0.is_negative() {
			write!(f, "-{:04}", -self.0)
		} else {
			write!(f, "{:04}", self.0)
		}
	}
}

#[derive(Debug, thiserror::Error)]
pub enum DateTimeFromStrError {
	#[error("invalid date syntax")]
	Syntax(#[from] InvalidDateTime<String>),

	#[error(transparent)]
	Value(#[from] InvalidDateTimeValue),
}

impl FromStr for DateTime {
	type Err = DateTimeFromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::DateTime::new(s)
			.map_err(|InvalidDateTime(s)| InvalidDateTime(s.to_owned()))?;
		lexical_value.try_as_value().map_err(Into::into)
	}
}

impl From<chrono::DateTime<FixedOffset>> for DateTime {
	fn from(value: chrono::DateTime<FixedOffset>) -> Self {
		let naive_date_time = value.naive_utc();
		let offset = *value.offset();
		Self::new(naive_date_time, Some(offset))
	}
}

impl From<chrono::DateTime<Utc>> for DateTime {
	fn from(value: chrono::DateTime<Utc>) -> Self {
		let naive_date_time = value.naive_utc();
		let offset = FixedOffset::east_opt(0).unwrap();
		Self::new(naive_date_time, Some(offset))
	}
}

impl TryFrom<DateTime> for chrono::DateTime<FixedOffset> {
	type Error = MissingTimezone;

	fn try_from(value: DateTime) -> Result<Self, MissingTimezone> {
		match value.offset {
			Some(offset) => Ok(value.date_time.and_local_timezone(offset).unwrap()),
			None => Err(MissingTimezone),
		}
	}
}

impl TryFrom<DateTime> for chrono::DateTime<Utc> {
	type Error = TimezoneError;

	fn try_from(value: DateTime) -> Result<Self, TimezoneError> {
		let fixed: chrono::DateTime<FixedOffset> = value.try_into()?;
		Ok(fixed.into())
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for DateTime {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.into_string().serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DateTime {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = DateTime;

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
