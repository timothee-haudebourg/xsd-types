use std::{cmp::Ordering, fmt, hash::Hash, str::FromStr};

use crate::{
	lexical::{InvalidDateTime, Lexical, LexicalFormOf},
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

#[derive(Debug, thiserror::Error)]
#[error("timezone offset out of range")]
pub struct InvalidOffset;

/// Returns whether `offset` is within the range permitted by the XSD
/// `timezoneFrag` production (`-14:00` to `+14:00`, inclusive).
pub(crate) fn is_valid_offset(offset: time::UtcOffset) -> bool {
	(-14 * 60 * 60..=14 * 60 * 60).contains(&offset.whole_seconds())
}

#[derive(Debug, Clone, Copy)]
pub struct DateTime {
	date_time: time::PrimitiveDateTime,
	offset: Option<time::UtcOffset>,
}

impl DateTime {
	/// Creates a new `DateTime`, or returns `None` if `offset` is outside
	/// the `-14:00..=+14:00` range permitted by XSD.
	pub fn new(
		date_time: time::PrimitiveDateTime,
		offset: Option<time::UtcOffset>,
	) -> Option<Self> {
		if offset.is_none_or(is_valid_offset) {
			Some(Self { date_time, offset })
		} else {
			None
		}
	}

	/// Returns the date and time, ignoring the timezone offset.
	pub fn date_time(&self) -> time::PrimitiveDateTime {
		self.date_time
	}

	/// Returns the date, ignoring the time of day and timezone offset.
	pub fn date(&self) -> time::Date {
		self.date_time.date()
	}

	/// Returns the time of day, ignoring the date and timezone offset.
	pub fn time(&self) -> time::Time {
		self.date_time.time()
	}

	/// Returns the timezone offset, if any.
	pub fn offset(&self) -> Option<time::UtcOffset> {
		self.offset
	}

	/// Deconstructs this `DateTime` into its date/time and timezone offset.
	pub fn into_parts(self) -> (time::PrimitiveDateTime, Option<time::UtcOffset>) {
		(self.date_time, self.offset)
	}

	/// Returns a `DateTime` which corresponds to the current time and date.
	pub fn now() -> Self {
		// `OffsetDateTime::now_utc` always has a zero offset, which is always
		// within the valid range.
		time::OffsetDateTime::now_utc().try_into().unwrap()
	}

	/// Returns a `DateTime` which corresponds to the current time and date,
	/// with millisecond precision (at most).
	pub fn now_ms() -> Self {
		let now = time::OffsetDateTime::now_utc();
		let ms = now.millisecond();
		let ns = ms as u32 * 1_000_000;
		// Same as `now`: the zero offset is always within the valid range.
		now.replace_nanosecond(ns)
			.unwrap_or(now)
			.try_into()
			.unwrap()
	}

	pub fn into_string(self) -> String {
		self.to_string()
	}

	/// Returns this `DateTime` as a `DateTimeStamp`, using the given offset
	/// if this `DateTime` has none of its own.
	fn with_offset(&self, default_offset: time::UtcOffset) -> DateTimeStamp {
		// `self.offset`, if present, was already validated when `self` was
		// constructed, and `default_offset` is always exactly `±14:00`, the
		// boundary of the valid range, so this is always in range.
		DateTimeStamp::new(self.date_time, self.offset.unwrap_or(default_offset)).unwrap()
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
		seven_property_model_eq(self.date_time, self.offset, other.date_time, other.offset)
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
		seven_property_model_partial_cmp(self.date_time, self.offset, other.date_time, other.offset)
	}
}

/// The largest permitted XSD `timezoneOffset` (`+14:00`).
fn max_offset() -> time::UtcOffset {
	time::UtcOffset::from_whole_seconds(14 * 60 * 60).unwrap()
}

/// The smallest permitted XSD `timezoneOffset` (`-14:00`).
fn min_offset() -> time::UtcOffset {
	time::UtcOffset::from_whole_seconds(-14 * 60 * 60).unwrap()
}

/// Resolves the possibly-absent `year`/`month`/`day` components of a
/// seven-property-model value (`dateTime`, `date`, `time`, `gYearMonth`,
/// `gYear`, `gMonthDay`, `gDay`, `gMonth`) into a full `PrimitiveDateTime`,
/// as required to compare or order such values.
///
/// Per <https://www.w3.org/TR/xmlschema11-2/#dt-dateTime> (Appendix
/// D.2.1), absent components are filled in with those of `1972-12-31`,
/// except that an absent `day` takes the last day permitted in the
/// (possibly itself defaulted) month.
pub(crate) fn seven_property_model_date_time(
	year: Option<i32>,
	month: Option<time::Month>,
	day: Option<u8>,
	time: time::Time,
) -> time::PrimitiveDateTime {
	let year = year.unwrap_or(1972);
	let month = month.unwrap_or(time::Month::December);
	let day = day.unwrap_or_else(|| time::util::days_in_month(month, year));

	// `year`, `month` and `day` are all in range by construction, so this
	// can't fail.
	let date = time::Date::from_calendar_date(year, month, day).unwrap();
	time::PrimitiveDateTime::new(date, time)
}

/// Shared XSD equality relation for the seven-property-model datatypes.
pub(crate) fn seven_property_model_eq(
	a_date_time: time::PrimitiveDateTime,
	a_offset: Option<time::UtcOffset>,
	b_date_time: time::PrimitiveDateTime,
	b_offset: Option<time::UtcOffset>,
) -> bool {
	let a_earliest = a_date_time.assume_offset(a_offset.unwrap_or_else(max_offset));
	let a_latest = a_date_time.assume_offset(a_offset.unwrap_or_else(min_offset));
	let b_earliest = b_date_time.assume_offset(b_offset.unwrap_or_else(max_offset));
	let b_latest = b_date_time.assume_offset(b_offset.unwrap_or_else(min_offset));

	a_earliest == b_earliest && a_latest == b_latest
}

/// Shared XSD order relation for the seven-property-model datatypes: compares
/// the imputed `+14:00`/`-14:00` bounds of each value, except that when
/// neither side has a timezone offset, they are compared directly (the
/// imputed-bounds algorithm only applies when *exactly one* side is missing
/// an offset).
pub(crate) fn seven_property_model_partial_cmp(
	a_date_time: time::PrimitiveDateTime,
	a_offset: Option<time::UtcOffset>,
	b_date_time: time::PrimitiveDateTime,
	b_offset: Option<time::UtcOffset>,
) -> Option<Ordering> {
	if a_offset.is_none() && b_offset.is_none() {
		return Some(a_date_time.cmp(&b_date_time));
	}

	let a_earliest = a_date_time.assume_offset(a_offset.unwrap_or_else(max_offset));
	let a_latest = a_date_time.assume_offset(a_offset.unwrap_or_else(min_offset));
	let b_earliest = b_date_time.assume_offset(b_offset.unwrap_or_else(max_offset));
	let b_latest = b_date_time.assume_offset(b_offset.unwrap_or_else(min_offset));

	match (a_earliest.cmp(&b_latest), a_latest.cmp(&b_earliest)) {
		(Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
		(Ordering::Less, Ordering::Less) => Some(Ordering::Less),
		(Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
		_ => None,
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
		let lexical_value = crate::lexical::DateTime::parse(s)?;
		lexical_value.try_as_value().map_err(Into::into)
	}
}

impl TryFrom<time::OffsetDateTime> for DateTime {
	type Error = InvalidOffset;

	fn try_from(value: time::OffsetDateTime) -> Result<Self, Self::Error> {
		let date_time = time::PrimitiveDateTime::new(value.date(), value.time());
		Self::new(date_time, Some(value.offset())).ok_or(InvalidOffset)
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
impl TryFrom<chrono::DateTime<chrono::FixedOffset>> for DateTime {
	type Error = InvalidOffset;

	fn try_from(value: chrono::DateTime<chrono::FixedOffset>) -> Result<Self, Self::Error> {
		let offset = time::UtcOffset::from_whole_seconds(value.offset().local_minus_utc()).unwrap();
		let odt = match value.timestamp_nanos_opt() {
			Some(t) => time::OffsetDateTime::from_unix_timestamp_nanos(t as i128).unwrap(),
			None => time::OffsetDateTime::from_unix_timestamp_nanos(
				value.timestamp_micros() as i128 * 1000,
			)
			.unwrap(),
		};

		odt.to_offset(offset).try_into()
	}
}

#[cfg(feature = "chrono")]
impl From<chrono::DateTime<chrono::Utc>> for DateTime {
	fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
		// A `chrono::Utc` timestamp always has a zero offset, which is always
		// within the valid range.
		value.fixed_offset().try_into().unwrap()
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
		serializer.collect_str(self)
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
	use super::{seven_property_model_date_time, DateTime, DateTimeStamp};

	/// Per <https://www.w3.org/TR/xmlschema11-2/#dt-dateTime> (Appendix
	/// D.2.1), an absent `day` takes the last day permitted in the
	/// (possibly itself defaulted) month, which must account for leap years.
	#[test]
	fn effective_date_time_defaults_day_to_end_of_month() {
		let feb_2024 = seven_property_model_date_time(
			Some(2024),
			Some(time::Month::February),
			None,
			time::Time::MIDNIGHT,
		);
		assert_eq!(feb_2024.date().day(), 29);

		let feb_2023 = seven_property_model_date_time(
			Some(2023),
			Some(time::Month::February),
			None,
			time::Time::MIDNIGHT,
		);
		assert_eq!(feb_2023.date().day(), 28);
	}

	#[test]
	fn new_rejects_out_of_range_offset() {
		let date_time = time::PrimitiveDateTime::new(
			time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap(),
			time::Time::MIDNIGHT,
		);
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();

		assert!(DateTime::new(date_time, Some(offset)).is_none());
	}

	#[test]
	fn try_from_offset_date_time_rejects_out_of_range_offset() {
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();
		let odt = time::OffsetDateTime::now_utc().to_offset(offset);

		assert!(DateTime::try_from(odt).is_err());
	}

	#[test]
	fn date_and_time_accessors() {
		let date = time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap();
		let time = time::Time::from_hms(12, 30, 0).unwrap();
		let date_time = time::PrimitiveDateTime::new(date, time);

		let dt = DateTime::new(date_time, None).unwrap();

		assert_eq!(dt.date(), date);
		assert_eq!(dt.time(), time);
		assert_eq!(dt.date_time(), date_time);
	}

	/// Per the same rule, a wholly-absent `year`/`month`/`day` (the `time`
	/// datatype's case) defaults to `1972-12-31`.
	#[test]
	fn effective_date_time_defaults_to_1972_12_31() {
		let date_time = seven_property_model_date_time(
			None,
			None,
			None,
			time::Time::from_hms(1, 2, 3).unwrap(),
		);
		assert_eq!(date_time.year(), 1972);
		assert_eq!(date_time.month(), time::Month::December);
		assert_eq!(date_time.day(), 31);
	}

	#[cfg(feature = "chrono")]
	#[test]
	fn chrono_fixed_offset_roundtrip() {
		// A non-zero, non-symmetric offset is essential here: with a `+00:00`
		// offset, an inverted sign conversion would go unnoticed.
		let xsd: DateTime = "2024-01-01T12:00:00+05:00".parse().unwrap();
		let chrono: chrono::DateTime<chrono::FixedOffset> = xsd.try_into().unwrap();

		assert_eq!(chrono.offset().local_minus_utc(), 5 * 3600);

		let back: DateTime = chrono.try_into().unwrap();
		assert_eq!(xsd, back);
	}

	#[cfg(feature = "chrono")]
	#[test]
	fn chrono_time_roundtrip() {
		let expected_time =
			time::OffsetDateTime::from_unix_timestamp_nanos(1726661641326000001).unwrap();
		let xsd: DateTime = expected_time.try_into().unwrap();
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
		let expected = DateTimeStamp::new(dt.date_time, offset).unwrap();

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
		assert_eq!(
			dt.earliest(),
			DateTimeStamp::new(dt.date_time, east).unwrap()
		);
		assert_eq!(dt.latest(), DateTimeStamp::new(dt.date_time, west).unwrap());

		// The two bounds must be 28 hours apart (the full `-14:00`..=`+14:00`
		// timezone range), with `latest` after `earliest`.
		let earliest = dt.earliest().to_offset_date_time();
		let latest = dt.latest().to_offset_date_time();
		assert_eq!(latest - earliest, time::Duration::hours(28));
	}

	/// When neither side has a timezone offset, the imputed `+14:00`/
	/// `-14:00` bounds algorithm must not be used: the two values are
	/// compared directly, as if both had the same (unspecified) offset.
	/// See: <https://www.w3.org/TR/xmlschema11-2/#dt-dateTime>.
	#[test]
	fn ord_without_offset_is_direct() {
		let a: DateTime = "2024-01-01T12:00:00".parse().unwrap();
		let b: DateTime = "2024-01-01T13:00:00".parse().unwrap();

		assert!(a < b);
		assert!(b > a);
		assert_eq!(a.partial_cmp(&a), Some(std::cmp::Ordering::Equal));
	}

	/// When exactly one side has a timezone offset, the imputed bounds
	/// algorithm still applies, and can yield an incomparable result.
	#[test]
	fn ord_with_one_offset_can_be_incomparable() {
		let a: DateTime = "2024-01-01T12:00:00".parse().unwrap();
		let b: DateTime = "2024-01-01T13:00:00+00:00".parse().unwrap();

		assert_eq!(a.partial_cmp(&b), None);
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
