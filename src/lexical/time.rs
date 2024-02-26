use static_regular_grammar::RegularGrammar;

use crate::InvalidTimeValue;

use super::{parse_timezone, Lexical, LexicalFormOf};

/// Time.
///
/// ```abnf
/// time = hour ":" minute ":" second ["." fraction]
///      / "24:00:00" ["." 1*"0"]
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
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(sized(TimeBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct Time(str);

impl Time {
	fn parts(&self) -> Parts {
		enum State {
			Hours,
			Minutes,
			Seconds,
		}

		let mut state = State::Hours;

		let mut minutes = 0;
		let mut seconds = 0;
		let mut timezone = 0;

		for (i, c) in self.0.char_indices() {
			state = match state {
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
			hours: &self.0[..(minutes - 1)],
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

impl Lexical for Time {
	type Error = InvalidTime<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidTime(value.to_owned()))
	}
}

impl LexicalFormOf<crate::Time> for Time {
	type ValueError = InvalidTimeValue;

	fn try_as_value(&self) -> Result<crate::Time, Self::ValueError> {
		self.parts().to_time()
	}
}

pub struct Parts<'a> {
	hours: &'a str,
	minutes: &'a str,
	seconds: &'a str,
	timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	fn to_time(&self) -> Result<crate::Time, crate::InvalidTimeValue> {
		let time = chrono::NaiveTime::from_hms_opt(
			self.hours.parse().unwrap(),
			self.minutes.parse().unwrap(),
			self.seconds.parse().unwrap(),
		)
		.ok_or(crate::InvalidTimeValue)?;

		Ok(crate::Time::new(time, self.timezone.map(parse_timezone)))
	}
}
