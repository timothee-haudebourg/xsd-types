use static_regular_grammar::RegularGrammar;

use crate::InvalidDateTimeValue;

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
/// hour = "0" NZDIGIT
///      / "1" DIGIT
///      / "2" ("0" / "1" / "2" / "3")
///
/// minute = "0" NZDIGIT
///        / ("1" / "2" / "3" / "4" / "5") DIGIT
///
/// second = "0" NZDIGIT
///        / ("1" / "2" / "3" / "4" / "5") DIGIT
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
		enum State {
			Year,
			Month,
			Day,
			Hours,
			Minutes,
			Seconds,
		}

		let mut state = State::Year;

		let mut month = 0;
		let mut day = 0;
		let mut hours = 0;
		let mut minutes = 0;
		let mut seconds = 0;
		let mut timezone = 0;

		for (i, c) in self.0.char_indices() {
			state = match state {
				State::Year => match c {
					'-' if i > 0 => {
						month = i + 1;
						State::Month
					}
					_ => State::Year,
				},
				State::Month => match c {
					'-' => {
						day = i + 1;
						State::Day
					}
					_ => State::Month,
				},
				State::Day => match c {
					'T' => {
						hours = i + 1;
						State::Hours
					}
					_ => State::Day,
				},
				State::Hours => match c {
					':' => {
						minutes = i + 1;
						State::Minutes
					}
					_ => State::Hours,
				},
				State::Minutes => match c {
					':' => {
						seconds = i + 1;
						State::Seconds
					}
					_ => State::Minutes,
				},
				State::Seconds => {
					timezone = i;

					if !matches!(c, '0'..='9' | '.') {
						break;
					}

					State::Seconds
				}
			};
		}

		Parts {
			year: &self.0[..(month - 1)],
			month: &self.0[month..(day - 1)],
			day: &self.0[day..(hours - 1)],
			hours: &self.0[hours..(minutes - 1)],
			minutes: &self.0[minutes..(seconds - 1)],
			seconds: &self.0[seconds..timezone],
			timezone: if timezone == self.0.len() {
				None
			} else {
				Some(&self.0[timezone..])
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

pub struct Parts<'a> {
	year: &'a str,
	month: &'a str,
	day: &'a str,
	hours: &'a str,
	minutes: &'a str,
	seconds: &'a str,
	timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	fn to_datetime(&self) -> Result<crate::DateTime, crate::InvalidDateTimeValue> {
		let date = chrono::NaiveDate::from_ymd_opt(
			self.year.parse().unwrap(),
			self.month.parse().unwrap(),
			self.day.parse().unwrap(),
		)
		.ok_or(crate::InvalidDateTimeValue)?;

		let time = chrono::NaiveTime::from_hms_opt(
			self.hours.parse().unwrap(),
			self.minutes.parse().unwrap(),
			self.seconds.parse().unwrap(),
		)
		.ok_or(crate::InvalidDateTimeValue)?;

		let datetime = chrono::NaiveDateTime::new(date, time);

		Ok(crate::DateTime::new(
			datetime,
			self.timezone.map(parse_timezone),
		))
	}
}

pub(crate) fn parse_timezone(tz: &str) -> chrono::FixedOffset {
	const HOUR: i32 = 3600;
	const MINUTE: i32 = 60;

	match tz {
		"Z" => chrono::FixedOffset::east_opt(0).unwrap(),
		"14:00" => chrono::FixedOffset::east_opt(14 * HOUR).unwrap(),
		n => {
			let (h, m) = n.split_once(":").unwrap();
			chrono::FixedOffset::east_opt(
				h.parse::<i32>().unwrap() * HOUR + m.parse::<i32>().unwrap() * MINUTE,
			)
			.unwrap()
		}
	}
}
