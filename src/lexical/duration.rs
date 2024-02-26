use super::{Lexical, LexicalFormOf};
use static_regular_grammar::RegularGrammar;

/// Duration.
///
/// ```abnf
/// duration = [ "-" ] %s"P" ((year-month [ day-time ]) / day-time)
///
/// year-month = (year [ month ]) / month
///
/// year = 1*DIGIT %s"Y"
///
/// month = 1*DIGIT %s"M"
///
/// day-time = (day [ time ]) / time
///
/// day = 1*DIGIT %s"D"
///
/// time = %s"T" ((hour [ minute ] [ second ]) / (minute [ second ]) / second)
///
/// hour = 1*DIGIT %s"H"
///
/// minute = 1*DIGIT %s"M"
///
/// second = ((1*DIGIT ["." *DIGIT] ) / "." 1*DIGIT) %s"S"
/// ```
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(sized(DurationBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct Duration(str);

impl Duration {
	pub fn parts(&self) -> Parts {
		enum State {
			Sign,
			DateNumber,
			TimeNumber,
		}

		let mut state = State::Sign;
		let mut result = Parts {
			is_negative: false,
			year: None,
			month: None,
			day: None,
			hour: None,
			minute: None,
			second: None,
		};

		let mut start = 0;

		for (i, c) in self.0.char_indices() {
			state = match state {
				State::Sign => match c {
					'-' => {
						result.is_negative = true;
						State::Sign
					}
					'P' => {
						start = i + 1;
						State::DateNumber
					}
					_ => unreachable!(),
				},
				State::DateNumber => match c {
					'Y' => {
						result.year = Some(&self.0[start..i]);
						start = i + 1;
						State::DateNumber
					}
					'M' => {
						result.month = Some(&self.0[start..i]);
						start = i + 1;
						State::DateNumber
					}
					'D' => {
						result.day = Some(&self.0[start..i]);
						start = i + 1;
						State::DateNumber
					}
					'T' => {
						start = i + 1;
						State::TimeNumber
					}
					_ => State::DateNumber,
				},
				State::TimeNumber => match c {
					'H' => {
						result.hour = Some(&self.0[start..i]);
						start = i + 1;
						State::TimeNumber
					}
					'M' => {
						result.minute = Some(&self.0[start..i]);
						start = i + 1;
						State::TimeNumber
					}
					'S' => {
						result.second = Some(&self.0[start..i]);
						start = i + 1;
						State::TimeNumber
					}
					_ => State::TimeNumber,
				},
			}
		}

		result
	}
}

impl Lexical for Duration {
	type Error = InvalidDuration<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidDuration(value.to_owned()))
	}
}

impl LexicalFormOf<crate::Duration> for Duration {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<crate::Duration, Self::ValueError> {
		Ok(self.parts().to_duration())
	}
}

pub struct Parts<'a> {
	is_negative: bool,
	year: Option<&'a str>,
	month: Option<&'a str>,
	day: Option<&'a str>,
	hour: Option<&'a str>,
	minute: Option<&'a str>,
	second: Option<&'a str>,
}

impl<'a> Parts<'a> {
	fn to_duration(&self) -> crate::Duration {
		let mut months = 0u32;

		if let Some(y) = self.year {
			let y: u32 = y.parse().unwrap();
			months += y * 12;
		}

		if let Some(m) = self.month {
			let m: u32 = m.parse().unwrap();
			months += m;
		}

		let mut seconds = 0u32;

		if let Some(d) = self.day {
			let d: u32 = d.parse().unwrap();
			seconds += d * 24 * 60 * 60;
		}

		if let Some(h) = self.hour {
			let h: u32 = h.parse().unwrap();
			seconds += h * 60 * 60;
		}

		if let Some(m) = self.minute {
			let m: u32 = m.parse().unwrap();
			seconds += m * 60;
		}

		let mut nano_seconds = 0u32;

		if let Some(s) = self.second {
			match s.split_once('.') {
				Some((s, fract)) => {
					let s: u32 = s.parse().unwrap();
					seconds += s;

					let fract = if fract.len() > 9 { &fract[..9] } else { fract };

					nano_seconds =
						fract.parse::<u32>().unwrap() * 10u32.pow(9 - fract.len() as u32);
				}
				None => {
					let s: u32 = s.parse().unwrap();
					seconds += s;
				}
			}
		}

		crate::Duration::new(self.is_negative, months, seconds, nano_seconds)
	}
}
