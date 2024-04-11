use crate::lexical::date_time::parse_seconds_decimal;

use super::{Lexical, LexicalFormOf};
use static_regular_grammar::RegularGrammar;

/// Day Time Duration.
///
/// ```abnf
/// duration = [ "-" ] %s"P" day-time
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
#[grammar(sized(DayTimeDurationBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct DayTimeDuration(str);

impl DayTimeDuration {
	pub fn parts(&self) -> Parts {
		enum State {
			Sign,
			DateNumber,
			TimeNumber,
		}

		let mut state = State::Sign;
		let mut result = Parts {
			is_negative: false,
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

impl Lexical for DayTimeDuration {
	type Error = InvalidDayTimeDuration<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidDayTimeDuration(value.to_owned()))
	}
}

impl LexicalFormOf<crate::DayTimeDuration> for DayTimeDuration {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<crate::DayTimeDuration, Self::ValueError> {
		Ok(self.parts().to_duration())
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a> {
	pub is_negative: bool,
	pub day: Option<&'a str>,
	pub hour: Option<&'a str>,
	pub minute: Option<&'a str>,
	pub second: Option<&'a str>,
}

impl<'a> Parts<'a> {
	pub fn new(
		is_negative: bool,
		day: Option<&'a str>,
		hour: Option<&'a str>,
		minute: Option<&'a str>,
		second: Option<&'a str>,
	) -> Self {
		Self {
			is_negative,
			day,
			hour,
			minute,
			second,
		}
	}
	fn to_duration(&self) -> crate::DayTimeDuration {
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
			let (s, ns) = parse_seconds_decimal(s);
			seconds += s;
			nano_seconds = ns;
		}

		crate::DayTimeDuration::new(self.is_negative, seconds, nano_seconds)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parsing() {
		let vectors = [(
			"-P1DT24H01M1.0001S",
			Parts::new(true, Some("1"), Some("24"), Some("01"), Some("1.0001")),
			"-P2DT1M1.0001S",
		)];

		for (input, parts, normalized) in vectors {
			let lexical_repr = DayTimeDuration::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value = lexical_repr.try_as_value().unwrap();
			assert_eq!(value.to_string().as_str(), normalized)
		}
	}
}
