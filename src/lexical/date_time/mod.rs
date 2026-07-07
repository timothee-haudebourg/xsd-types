use static_automata::Validate;
use str_newtype::StrNewType;

use crate::{utils::byte_index_of, InvalidDateTimeValue};

use super::{Lexical, LexicalFormOf};

pub mod date_time_stamp;
pub use date_time_stamp::{DateTimeStamp, DateTimeStampBuf, InvalidDateTimeStamp};

/// Date and time.
///
/// ```abnf
/// date-time = year "-" month "-" day %s"T" time [timezone]
///
/// time = hour ":" minute ":" second ["." fraction]
///      / "24:00:00" ["." 1*"0"]
///
/// year = [ "-" ] year-number
///
/// year-number = *3DIGIT NZDIGIT
///             / *2DIGIT NZDIGIT DIGIT
///             / *1DIGIT NZDIGIT 2DIGIT
///             / NZDIGIT 3*DIGIT
///
/// month = "0" NZDIGIT
///       / "1" ( "0" / "1" / "2" )
///
/// day = "0" NZDIGIT
///     / ("1" / "2") DIGIT
///     / "3" ("0" / "1")
///
/// hour = ("0" / "1") DIGIT
///      / "2" ("0" / "1" / "2" / "3")
///
/// minute = ("0" / "1" / "2" / "3" / "4" / "5") DIGIT
///
/// second = ("0" / "1" / "2" / "3" / "4" / "5") DIGIT
///
/// fraction = 1*DIGIT
///
/// timezone = ("+" / "-") ((("0" DIGIT / "1" ("0" / "1" / "2" / "3")) ":" minute) / "14:00")
///          / %s"Z"
///
/// NZDIGIT = "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9"
/// ```
///
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::DateTime)]
#[newtype(owned(DateTimeBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct DateTime(str);

impl DateTime {
	pub fn parts(&self) -> Parts<'_> {
		let year_end = byte_index_of(self.0.as_bytes(), 4, b'-').unwrap();
		let month_end = year_end + 3;
		let day_end = month_end + 3;
		let hour_end = day_end + 3;
		let minute_end = hour_end + 3;
		let second_end = byte_index_of(self.0.as_bytes(), minute_end + 3, [b'+', b'-', b'Z'])
			.unwrap_or(self.0.len());

		Parts {
			year: &self.0[..year_end],
			month: &self.0[(year_end + 1)..month_end],
			day: &self.0[(month_end + 1)..day_end],
			hours: &self.0[(day_end + 1)..hour_end],
			minutes: &self.0[(hour_end + 1)..minute_end],
			seconds: &self.0[(minute_end + 1)..second_end],
			timezone: if second_end == self.0.len() {
				None
			} else {
				Some(&self.0[second_end..])
			},
		}
	}
}

impl Lexical for DateTime {
	type Error = InvalidDateTime<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidDateTime(value.to_owned()))
	}
}

impl LexicalFormOf<crate::DateTime> for DateTime {
	type ValueError = InvalidDateTimeValue;

	fn try_as_value(&self) -> Result<crate::DateTime, Self::ValueError> {
		self.parts().to_datetime()
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a> {
	pub year: &'a str,
	pub month: &'a str,
	pub day: &'a str,
	pub hours: &'a str,
	pub minutes: &'a str,
	pub seconds: &'a str,
	pub timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	pub fn new(
		year: &'a str,
		month: &'a str,
		day: &'a str,
		hours: &'a str,
		minutes: &'a str,
		seconds: &'a str,
		timezone: Option<&'a str>,
	) -> Self {
		Self {
			year,
			month,
			day,
			hours,
			minutes,
			seconds,
			timezone,
		}
	}

	fn to_datetime(&self) -> Result<crate::DateTime, crate::InvalidDateTimeValue> {
		let month = time::Month::try_from(self.month.parse::<u8>().unwrap())
			.map_err(|_| crate::InvalidDateTimeValue)?;

		let date = time::Date::from_calendar_date(
			self.year.parse().unwrap(),
			month,
			self.day.parse().unwrap(),
		)
		.map_err(|_| crate::InvalidDateTimeValue)?;

		let (seconds, nanoseconds) = parse_seconds_decimal(self.seconds);

		let time = time::Time::from_hms_nano(
			self.hours.parse().unwrap(),
			self.minutes.parse().unwrap(),
			seconds as u8,
			nanoseconds,
		)
		.map_err(|_| crate::InvalidDateTimeValue)?;

		let datetime = time::PrimitiveDateTime::new(date, time);

		Ok(crate::DateTime::new(
			datetime,
			self.timezone.map(|tz| parse_timezone(tz).unwrap()),
		))
	}
}

/// Parses a decimal number representing seconds and returns the represented
/// number of seconds and nanoseconds.
pub(crate) fn parse_seconds_decimal(decimal: &str) -> (u32, u32) {
	match decimal.split_once('.') {
		Some((integer, fract)) => {
			let seconds = integer.parse().unwrap();
			let fract = if fract.len() > 9 { &fract[..9] } else { fract };
			let nano_seconds = fract.parse::<u32>().unwrap() * 10u32.pow(9 - fract.len() as u32);

			(seconds, nano_seconds)
		}
		None => (decimal.parse().unwrap(), 0),
	}
}

/// Parses a timezone offset, returning `None` if `tz` is not a valid
/// timezone offset.
pub(crate) fn parse_timezone(tz: &str) -> Option<time::UtcOffset> {
	const HOUR: i32 = 3600;
	const MINUTE: i32 = 60;

	match tz {
		"Z" => Some(time::UtcOffset::UTC),
		"14:00" => time::UtcOffset::from_whole_seconds(14 * HOUR).ok(),
		n => {
			let (h, m) = n.split_once(':')?;
			let seconds = h.parse::<i32>().ok()? * HOUR + m.parse::<i32>().ok()? * MINUTE;
			time::UtcOffset::from_whole_seconds(seconds).ok()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parsing() {
		let vectors = [
			(
				"2002-05-31T13:07:12+01:00",
				Parts::new("2002", "05", "31", "13", "07", "12", Some("+01:00")),
			),
			(
				"2002-05-31T13:07:12",
				Parts::new("2002", "05", "31", "13", "07", "12", None),
			),
			(
				"-2002-05-31T13:07:12+01:00",
				Parts::new("-2002", "05", "31", "13", "07", "12", Some("+01:00")),
			),
			(
				"2002-10-10T12:00:00-05:00",
				Parts::new("2002", "10", "10", "12", "00", "00", Some("-05:00")),
			),
		];

		for (input, parts) in vectors {
			let lexical_repr = DateTime::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value = lexical_repr.try_as_value().unwrap();
			assert_eq!(value.to_string().as_str(), input)
		}
	}

	/// Years outside of `-9999..=9999` require the `large-dates` feature of
	/// the `time` crate.
	#[cfg(feature = "large-dates")]
	#[test]
	fn parsing_large_dates() {
		let vectors = [
			(
				"202002-10-10T12:00:00.00001-05:00",
				Parts::new("202002", "10", "10", "12", "00", "00.00001", Some("-05:00")),
			),
			(
				"-202002-10-10T12:00:00.00001-05:00",
				Parts::new(
					"-202002",
					"10",
					"10",
					"12",
					"00",
					"00.00001",
					Some("-05:00"),
				),
			),
		];

		for (input, parts) in vectors {
			let lexical_repr = DateTime::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value = lexical_repr.try_as_value().unwrap();
			assert_eq!(value.to_string().as_str(), input)
		}
	}
}
