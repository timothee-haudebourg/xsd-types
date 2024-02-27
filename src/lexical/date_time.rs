use static_regular_grammar::RegularGrammar;

use crate::{utils::byte_index_of, InvalidDateTimeValue};

use super::{Lexical, LexicalFormOf};

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
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(sized(DateTimeBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct DateTime(str);

impl DateTime {
	pub fn parts(&self) -> Parts {
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
		let date = chrono::NaiveDate::from_ymd_opt(
			self.year.parse().unwrap(),
			self.month.parse().unwrap(),
			self.day.parse().unwrap(),
		)
		.ok_or(crate::InvalidDateTimeValue)?;

		let (seconds, nanoseconds) = parse_seconds_decimal(self.seconds);

		let time = chrono::NaiveTime::from_hms_nano_opt(
			self.hours.parse().unwrap(),
			self.minutes.parse().unwrap(),
			seconds,
			nanoseconds,
		)
		.ok_or(crate::InvalidDateTimeValue)?;

		let datetime = chrono::NaiveDateTime::new(date, time);

		Ok(crate::DateTime::new(
			datetime,
			self.timezone.map(parse_timezone),
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

pub(crate) fn parse_timezone(tz: &str) -> chrono::FixedOffset {
	const HOUR: i32 = 3600;
	const MINUTE: i32 = 60;

	match tz {
		"Z" => chrono::FixedOffset::east_opt(0).unwrap(),
		"14:00" => chrono::FixedOffset::east_opt(14 * HOUR).unwrap(),
		n => {
			let (h, m) = n.split_once(':').unwrap();
			chrono::FixedOffset::east_opt(
				h.parse::<i32>().unwrap() * HOUR + m.parse::<i32>().unwrap() * MINUTE,
			)
			.unwrap()
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
