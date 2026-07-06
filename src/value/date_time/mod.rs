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
#[error("invalid datetime value")]
pub struct InvalidDateTimeValue;

#[derive(Debug, Clone, Copy)]
pub struct DateTime {
	pub date_time: time::PrimitiveDateTime,
	pub offset: Option<time::UtcOffset>,
}

impl DateTime {
	pub fn new(date_time: time::PrimitiveDateTime, offset: Option<time::UtcOffset>) -> Self {
		Self { date_time, offset }
	}

	/// Returns a `DateTime` which corresponds to the current time and date.
	pub fn now() -> Self {
		time::OffsetDateTime::now_utc().into()
	}

	/// Returns a `DateTime` which corresponds to the current time and date,
	/// with millisecond precision (at most).
	pub fn now_ms() -> Self {
		let now = time::OffsetDateTime::now_utc();
		let ms = now.millisecond();
		let ns = ms as u32 * 1_000_000;
		now.replace_nanosecond(ns).unwrap_or(now).into()
	}

	pub fn into_string(self) -> String {
		self.to_string()
	}

	/// Returns this `DateTime` as a `DateTimeStamp`, using the given offset
	/// if this `DateTime` has none of its own.
	fn with_offset(&self, default_offset: time::UtcOffset) -> DateTimeStamp {
		DateTimeStamp::new(self.date_time, self.offset.unwrap_or(default_offset))
	}

	/// Returns the earliest date/time with offset represented by this
	/// date/time.
	///
	/// The instant represented by an offset-less `DateTime` is
	/// `self.date_time - offset` for some unknown `offset` in
	/// `-14:00..=+14:00`. The earliest (smallest) possible instant is
	/// therefore obtained with the largest offset, `+14:00`.
	pub fn earliest(&self) -> DateTimeStamp {
		self.with_offset(time::UtcOffset::from_whole_seconds(14 * 60 * 60).unwrap())
	}

	/// Returns the latest date/time with offset represented by this
	/// date/time.
	///
	/// The latest (largest) possible instant is obtained with the smallest
	/// offset, `-14:00`.
	pub fn latest(&self) -> DateTimeStamp {
		self.with_offset(time::UtcOffset::from_whole_seconds(-14 * 60 * 60).unwrap())
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
			u8::from(self.date_time.month()),
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

pub(crate) fn format_timezone(tz: Option<time::UtcOffset>, f: &mut fmt::Formatter) -> fmt::Result {
	match tz {
		Some(tz) => {
			let total_seconds = tz.whole_seconds();
			if total_seconds == 0 {
				write!(f, "Z")
			} else {
				let abs_seconds = if total_seconds > 0 {
					write!(f, "+")?;
					total_seconds as u32
				} else {
					write!(f, "-")?;
					-total_seconds as u32
				};

				let tz_minutes = abs_seconds / 60;
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

impl From<time::OffsetDateTime> for DateTime {
	fn from(value: time::OffsetDateTime) -> Self {
		let date_time = time::PrimitiveDateTime::new(value.date(), value.time());
		Self::new(date_time, Some(value.offset()))
	}
}

impl TryFrom<DateTime> for time::OffsetDateTime {
	type Error = MissingTimezone;

	fn try_from(value: DateTime) -> Result<Self, MissingTimezone> {
		match value.offset {
			Some(offset) => Ok(value.date_time.assume_offset(offset)),
			None => Err(MissingTimezone),
		}
	}
}

#[cfg(feature = "chrono")]
impl From<chrono::DateTime<chrono::FixedOffset>> for DateTime {
	fn from(value: chrono::DateTime<chrono::FixedOffset>) -> Self {
		let offset = time::UtcOffset::from_whole_seconds(value.offset().local_minus_utc()).unwrap();
		let odt = match value.timestamp_nanos_opt() {
			Some(t) => time::OffsetDateTime::from_unix_timestamp_nanos(t as i128).unwrap(),
			None => time::OffsetDateTime::from_unix_timestamp_nanos(
				value.timestamp_micros() as i128 * 1000,
			)
			.unwrap(),
		};

		odt.to_offset(offset).into()
	}
}

#[cfg(feature = "chrono")]
impl From<chrono::DateTime<chrono::Utc>> for DateTime {
	fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
		value.fixed_offset().into()
	}
}

#[cfg(feature = "chrono")]
impl TryFrom<DateTime> for chrono::DateTime<chrono::FixedOffset> {
	type Error = MissingTimezone;

	fn try_from(value: DateTime) -> Result<Self, MissingTimezone> {
		use chrono::TimeZone;

		let odt: time::OffsetDateTime = value.try_into()?;
		let offset = chrono::FixedOffset::east_opt(odt.offset().whole_seconds()).unwrap();
		Ok(offset.timestamp_nanos(odt.unix_timestamp_nanos() as i64))
	}
}

#[cfg(feature = "chrono")]
impl TryFrom<DateTime> for chrono::DateTime<chrono::Utc> {
	type Error = MissingTimezone;

	fn try_from(value: DateTime) -> Result<Self, MissingTimezone> {
		let fixed: chrono::DateTime<chrono::FixedOffset> = value.try_into()?;
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

#[cfg(test)]
mod tests {
	use super::{DateTime, DateTimeStamp};

	#[cfg(feature = "chrono")]
	#[test]
	fn chrono_fixed_offset_roundtrip() {
		// A non-zero, non-symmetric offset is essential here: with a `+00:00`
		// offset, an inverted sign conversion would go unnoticed.
		let xsd: DateTime = "2024-01-01T12:00:00+05:00".parse().unwrap();
		let chrono: chrono::DateTime<chrono::FixedOffset> = xsd.try_into().unwrap();

		assert_eq!(chrono.offset().local_minus_utc(), 5 * 3600);

		let back: DateTime = chrono.into();
		assert_eq!(xsd, back);
	}

	#[cfg(feature = "chrono")]
	#[test]
	fn chrono_time_roundtrip() {
		let expected_time =
			time::OffsetDateTime::from_unix_timestamp_nanos(1726661641326000001).unwrap();
		let xsd: DateTime = expected_time.into();
		let chrono: chrono::DateTime<chrono::Utc> = xsd.try_into().unwrap();
		let expected_chrono = chrono::DateTime::from_timestamp_millis(1726661641326).unwrap()
			+ std::time::Duration::from_nanos(1);
		assert_eq!(chrono, expected_chrono);

		let xsd: DateTime = chrono.into();
		let time: time::OffsetDateTime = xsd.try_into().unwrap();
		assert_eq!(time, expected_time);
	}

	#[test]
	fn earliest_latest_with_offset() {
		let dt: DateTime = "2024-01-01T12:00:00+02:00".parse().unwrap();
		let offset = time::UtcOffset::from_whole_seconds(2 * 60 * 60).unwrap();
		let expected = DateTimeStamp::new(dt.date_time, offset);

		// When the offset is known, `earliest` and `latest` both resolve to
		// that single, unambiguous instant.
		assert_eq!(dt.earliest(), expected);
		assert_eq!(dt.latest(), expected);
	}

	#[test]
	fn earliest_latest_without_offset() {
		let dt: DateTime = "2024-01-01T12:00:00".parse().unwrap();

		let west = time::UtcOffset::from_whole_seconds(-14 * 60 * 60).unwrap();
		let east = time::UtcOffset::from_whole_seconds(14 * 60 * 60).unwrap();

		// The earliest possible instant is obtained with the `+14:00` offset,
		// the latest with the `-14:00` offset.
		assert_eq!(dt.earliest(), DateTimeStamp::new(dt.date_time, east));
		assert_eq!(dt.latest(), DateTimeStamp::new(dt.date_time, west));

		// The two bounds must be 28 hours apart (the full `-14:00`..=`+14:00`
		// timezone range), with `latest` after `earliest`.
		let earliest = dt.earliest().to_offset_date_time();
		let latest = dt.latest().to_offset_date_time();
		assert_eq!(latest - earliest, time::Duration::hours(28));
	}

	#[test]
	fn eq_uses_earliest_and_latest() {
		let a: DateTime = "2024-01-01T12:00:00+02:00".parse().unwrap();
		let b: DateTime = "2024-01-01T12:00:00+02:00".parse().unwrap();
		let c: DateTime = "2024-01-01T12:00:00+03:00".parse().unwrap();

		assert_eq!(a, b);
		assert_ne!(a, c);
	}
}
