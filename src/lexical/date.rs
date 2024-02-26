use static_regular_grammar::RegularGrammar;

use crate::lexical::parse_timezone;

use super::{Lexical, LexicalFormOf};

/// Date.
///
/// ```abnf
/// date = year "-" month "-" day [timezone]
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
/// minute = "0" NZDIGIT
///        / ("1" / "2" / "3" / "4" / "5") DIGIT
///
/// timezone = ("+" / "-") ((("0" DIGIT / "1" ("0" / "1" / "2" / "3")) ":" minute) / "14:00")
///          / %s"Z"
///
/// NZDIGIT = "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9"
/// ```
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(sized(DateBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct Date(str);

impl Date {
	pub fn parts(&self) -> Parts {
		enum State {
			Year,
			Month,
			Day,
		}

		let mut state = State::Year;

		let mut month = 0;
		let mut day = 0;
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
						timezone = day;
						State::Day
					}
					_ => State::Month,
				},
				State::Day => match c {
					'0'..='9' => {
						timezone = i;
						State::Day
					}
					_ => {
						timezone = i;
						break;
					}
				},
			};
		}

		Parts {
			year: &self.0[..(month - 1)],
			month: &self.0[month..(day - 1)],
			day: &self.0[day..timezone],
			timezone: if timezone == self.0.len() {
				None
			} else {
				Some(&self.0[timezone..])
			},
		}
	}
}

impl Lexical for Date {
	type Error = InvalidDate<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidDate(value.to_owned()))
	}
}

impl LexicalFormOf<crate::Date> for Date {
	type ValueError = crate::InvalidDateValue;

	fn try_as_value(&self) -> Result<crate::Date, Self::ValueError> {
		self.parts().to_date()
	}
}

pub struct Parts<'a> {
	year: &'a str,
	month: &'a str,
	day: &'a str,
	timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	fn to_date(&self) -> Result<crate::Date, crate::InvalidDateValue> {
		let date = chrono::NaiveDate::from_ymd_opt(
			self.year.parse().unwrap(),
			self.month.parse().unwrap(),
			self.day.parse().unwrap(),
		)
		.ok_or(crate::InvalidDateValue)?;

		Ok(crate::Date::new(date, self.timezone.map(parse_timezone)))
	}
}
