use super::{Lexical, LexicalFormOf};
use static_regular_grammar::RegularGrammar;

/// Year Month Duration.
///
/// ```abnf
/// duration = [ "-" ] %s"P" year-month
///
/// year-month = (year [ month ]) / month
///
/// year = 1*DIGIT %s"Y"
///
/// month = 1*DIGIT %s"M"
/// ```
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(sized(YearMonthDurationBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct YearMonthDuration(str);

impl YearMonthDuration {
	pub fn parts(&self) -> Parts {
		enum State {
			Sign,
			DateNumber,
		}

		let mut state = State::Sign;
		let mut result = Parts {
			is_negative: false,
			year: None,
			month: None,
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
					_ => State::DateNumber,
				},
			}
		}

		result
	}
}

impl Lexical for YearMonthDuration {
	type Error = InvalidYearMonthDuration<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidYearMonthDuration(value.to_owned()))
	}
}

impl LexicalFormOf<crate::YearMonthDuration> for YearMonthDuration {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<crate::YearMonthDuration, Self::ValueError> {
		Ok(self.parts().to_duration())
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a> {
	pub is_negative: bool,
	pub year: Option<&'a str>,
	pub month: Option<&'a str>,
}

impl<'a> Parts<'a> {
	pub fn new(is_negative: bool, year: Option<&'a str>, month: Option<&'a str>) -> Self {
		Self {
			is_negative,
			year,
			month,
		}
	}
	fn to_duration(&self) -> crate::YearMonthDuration {
		let mut months = 0u32;

		if let Some(y) = self.year {
			let y: u32 = y.parse().unwrap();
			months += y * 12;
		}

		if let Some(m) = self.month {
			let m: u32 = m.parse().unwrap();
			months += m;
		}

		crate::YearMonthDuration::new(self.is_negative, months)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parsing() {
		let vectors = [
			("P9Y11M", Parts::new(false, Some("9"), Some("11")), "P9Y11M"),
			("P9Y12M", Parts::new(false, Some("9"), Some("12")), "P10Y"),
			("-P9Y12M", Parts::new(true, Some("9"), Some("12")), "-P10Y"),
		];

		for (input, parts, normalized) in vectors {
			let lexical_repr = YearMonthDuration::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value = lexical_repr.try_as_value().unwrap();
			assert_eq!(value.to_string().as_str(), normalized)
		}
	}
}
