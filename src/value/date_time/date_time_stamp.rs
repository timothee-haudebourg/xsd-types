use std::{cmp::Ordering, fmt, hash::Hash, str::FromStr};

use crate::{
	DisplayYear, XsdDatatype, XsdDateTimeDatatype, XsdFromStr, XsdTyped, XsdValueDomain,
	format_nanoseconds, format_timezone, lexical::InvalidXsdDateTimeStampStr,
};

/// Error raised when the components of a [`XsdDateTimeStamp`] do not form a
/// valid date and time (e.g. an out-of-range day for the given month, or a
/// timezone offset outside the range permitted by XSD).
#[derive(Debug, thiserror::Error)]
#[error("invalid datetimestamp value")]
pub struct InvalidXsdDateTimeStampValue;

/// A date and time with a required timezone offset.
///
/// This is the value space of the XSD `dateTimeStamp` datatype: a decoded
/// `time::PrimitiveDateTime` paired with a mandatory XSD timezone offset, as
/// opposed to its lexical (string) representation,
/// [`lexical::XsdDateTimeStampStr`](crate::lexical::XsdDateTimeStampStr). Unlike
/// [`XsdDateTime`](crate::XsdDateTime), the offset is never absent, so two
/// `XsdDateTimeStamp` values are always totally ordered by the instant they
/// represent.
/// See: <https://www.w3.org/TR/xmlschema11-2/#dateTimeStamp>.
#[derive(Debug, Clone, Copy)]
pub struct XsdDateTimeStamp {
	date_time: time::PrimitiveDateTime,
	offset: time::UtcOffset,
}

impl XsdDateTimeStamp {
	/// Creates a new `XsdDateTimeStamp`, or returns `None` if `offset` is
	/// outside the `-14:00..=+14:00` range permitted by XSD.
	pub fn new(date_time: time::PrimitiveDateTime, offset: time::UtcOffset) -> Option<Self> {
		if crate::is_valid_offset(offset) {
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

	/// Returns the timezone offset.
	pub fn offset(&self) -> time::UtcOffset {
		self.offset
	}

	/// Deconstructs this `XsdDateTimeStamp` into its date/time and timezone
	/// offset.
	pub fn into_parts(self) -> (time::PrimitiveDateTime, time::UtcOffset) {
		(self.date_time, self.offset)
	}

	/// Returns a `XsdDateTimeStamp` which corresponds to the current time and
	/// date.
	pub fn now() -> Self {
		// `OffsetDateTime::now_utc` always has a zero offset, which is always
		// within the valid range.
		time::OffsetDateTime::now_utc().try_into().unwrap()
	}

	/// Returns a `XsdDateTimeStamp` which corresponds to the current time and
	/// date, with millisecond precision (at most).
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

	/// Converts this `XsdDateTimeStamp` into its lexical (string) representation.
	pub fn into_string(self) -> String {
		self.to_string()
	}

	/// Converts this `XsdDateTimeStamp` to a `time::OffsetDateTime`.
	pub fn to_offset_date_time(&self) -> time::OffsetDateTime {
		self.date_time.assume_offset(self.offset)
	}

	/// Converts this `XsdDateTimeStamp` to a `chrono::DateTime<chrono::FixedOffset>`.
	#[cfg(feature = "chrono")]
	pub fn to_chrono_date_time(&self) -> chrono::DateTime<chrono::FixedOffset> {
		use chrono::TimeZone;

		let odt = self.to_offset_date_time();
		let offset = chrono::FixedOffset::east_opt(odt.offset().whole_seconds()).unwrap();
		offset.timestamp_nanos(odt.unix_timestamp_nanos() as i64)
	}
}

impl PartialEq for XsdDateTimeStamp {
	fn eq(&self, other: &Self) -> bool {
		self.to_offset_date_time() == other.to_offset_date_time()
	}
}

#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> PartialEq<chrono::DateTime<Tz>> for XsdDateTimeStamp {
	fn eq(&self, other: &chrono::DateTime<Tz>) -> bool {
		self.to_chrono_date_time() == *other
	}
}

#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> PartialEq<XsdDateTimeStamp> for chrono::DateTime<Tz> {
	fn eq(&self, other: &XsdDateTimeStamp) -> bool {
		*self == other.to_chrono_date_time()
	}
}

impl Eq for XsdDateTimeStamp {}

impl Hash for XsdDateTimeStamp {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.date_time.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for XsdDateTimeStamp {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> PartialOrd<chrono::DateTime<Tz>> for XsdDateTimeStamp {
	fn partial_cmp(&self, other: &chrono::DateTime<Tz>) -> Option<Ordering> {
		self.to_chrono_date_time().partial_cmp(other)
	}
}

#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> PartialOrd<XsdDateTimeStamp> for chrono::DateTime<Tz> {
	fn partial_cmp(&self, other: &XsdDateTimeStamp) -> Option<Ordering> {
		self.partial_cmp(&other.to_chrono_date_time())
	}
}

impl Ord for XsdDateTimeStamp {
	fn cmp(&self, other: &Self) -> Ordering {
		self.to_offset_date_time().cmp(&other.to_offset_date_time())
	}
}

impl XsdTyped for XsdDateTimeStamp {
	fn datatype(&self) -> XsdDatatype {
		XsdDatatype::DateTime(XsdDateTimeDatatype::DateTimeStamp)
	}
}

impl XsdValueDomain for XsdDateTimeStamp {
	type LexicalForm = crate::lexical::XsdDateTimeStampStr;

	fn to_lexical_form(&self) -> crate::lexical::XsdDateTimeStampString {
		// `Display` emits the canonical lexical form by contract, so it is
		// always valid and this cannot panic.
		crate::lexical::XsdDateTimeStampString::new(self.to_string())
			.expect("Display emits a valid dateTimeStamp lexical form")
	}
}

impl<'a> XsdFromStr<'a> for XsdDateTimeStamp {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<crate::lexical::XsdDateTimeStampStr, Self>(lexical_value)
	}
}

impl fmt::Display for XsdDateTimeStamp {
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
		format_timezone(Some(self.offset), f)
	}
}

/// Error raised when parsing a [`XsdDateTimeStamp`] from a string, either
/// because the input is not a syntactically valid `dateTimeStamp` lexical
/// representation, or because it does not denote a valid date/time value.
#[derive(Debug, thiserror::Error)]
pub enum XsdDateTimeStampFromStrError {
	#[error("invalid date syntax")]
	Syntax(#[from] InvalidXsdDateTimeStampStr<String>),

	#[error(transparent)]
	Value(#[from] InvalidXsdDateTimeStampValue),
}

impl FromStr for XsdDateTimeStamp {
	type Err = XsdDateTimeStampFromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::XsdDateTimeStampStr::new(s)
			.map_err(|_| InvalidXsdDateTimeStampStr(s.to_owned()))?;
		Self::try_from(lexical_value).map_err(Into::into)
	}
}

impl TryFrom<time::OffsetDateTime> for XsdDateTimeStamp {
	type Error = crate::InvalidXsdOffset;

	fn try_from(value: time::OffsetDateTime) -> Result<Self, Self::Error> {
		let date_time = time::PrimitiveDateTime::new(value.date(), value.time());
		Self::new(date_time, value.offset()).ok_or(crate::InvalidXsdOffset)
	}
}

impl From<XsdDateTimeStamp> for time::OffsetDateTime {
	fn from(value: XsdDateTimeStamp) -> Self {
		value.to_offset_date_time()
	}
}

#[cfg(feature = "chrono")]
impl TryFrom<chrono::DateTime<chrono::FixedOffset>> for XsdDateTimeStamp {
	type Error = crate::InvalidXsdOffset;

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
impl From<chrono::DateTime<chrono::Utc>> for XsdDateTimeStamp {
	fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
		// A `chrono::Utc` timestamp always has a zero offset, which is always
		// within the valid range.
		value.fixed_offset().try_into().unwrap()
	}
}

#[cfg(feature = "chrono")]
impl From<XsdDateTimeStamp> for chrono::DateTime<chrono::FixedOffset> {
	fn from(value: XsdDateTimeStamp) -> Self {
		value.to_chrono_date_time()
	}
}

#[cfg(feature = "chrono")]
impl From<XsdDateTimeStamp> for chrono::DateTime<chrono::Utc> {
	fn from(value: XsdDateTimeStamp) -> Self {
		value.to_chrono_date_time().into()
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdDateTimeStamp {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdDateTimeStamp {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdDateTimeStamp;

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
	#[test]
	fn to_lexical_form_roundtrip() {
		use super::XsdDateTimeStamp;
		use crate::XsdValueDomain;

		for s in [
			"2024-02-29T12:30:45Z",
			"-0044-03-15T00:00:00+05:30",
			"2000-01-01T23:59:59.000000001-14:00",
			"1999-12-31T13:07:12.5+14:00",
		] {
			let value: XsdDateTimeStamp = s.parse().unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), s);
			assert_eq!(lexical.as_str().parse::<XsdDateTimeStamp>().unwrap(), value);
		}
	}

	#[test]
	fn new_rejects_out_of_range_offset() {
		use super::XsdDateTimeStamp;

		let date_time = time::PrimitiveDateTime::new(
			time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap(),
			time::Time::MIDNIGHT,
		);
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();

		assert!(XsdDateTimeStamp::new(date_time, offset).is_none());
	}

	#[test]
	fn date_and_time_accessors() {
		use super::XsdDateTimeStamp;

		let date = time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap();
		let time = time::Time::from_hms(12, 30, 0).unwrap();
		let date_time = time::PrimitiveDateTime::new(date, time);
		let offset = time::UtcOffset::UTC;

		let dts = XsdDateTimeStamp::new(date_time, offset).unwrap();

		assert_eq!(dts.date(), date);
		assert_eq!(dts.time(), time);
		assert_eq!(dts.date_time(), date_time);
	}

	#[cfg(feature = "chrono")]
	#[test]
	fn chrono_fixed_offset_roundtrip() {
		use super::XsdDateTimeStamp;

		// A non-zero, non-symmetric offset is essential here: with a `+00:00`
		// offset, an inverted sign conversion would go unnoticed.
		let xsd: XsdDateTimeStamp = "2024-01-01T12:00:00+05:00".parse().unwrap();
		let chrono: chrono::DateTime<chrono::FixedOffset> = xsd.into();

		assert_eq!(chrono.offset().local_minus_utc(), 5 * 3600);

		let back: XsdDateTimeStamp = chrono.try_into().unwrap();
		assert_eq!(xsd, back);
	}

	#[cfg(feature = "chrono")]
	#[test]
	fn chrono_time_roundtrip() {
		use super::XsdDateTimeStamp;

		let expected_time =
			time::OffsetDateTime::from_unix_timestamp_nanos(1726661641326000001).unwrap();
		let xsd: XsdDateTimeStamp = expected_time.try_into().unwrap();
		let chrono: chrono::DateTime<chrono::Utc> = xsd.into();
		let expected_chrono = chrono::DateTime::from_timestamp_millis(1726661641326).unwrap()
			+ std::time::Duration::from_nanos(1);
		assert_eq!(chrono, expected_chrono);

		let xsd: XsdDateTimeStamp = chrono.into();
		let time: time::OffsetDateTime = xsd.into();
		assert_eq!(time, expected_time);
	}
}
