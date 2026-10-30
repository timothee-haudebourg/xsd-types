use super::{XsdLexicalForm, date_time::parse_seconds_decimal};
use crate::XsdError;
use static_automata::Validate;
use str_newtype::StrNewType;

mod day_time_duration;
pub use day_time_duration::*;

mod year_month_duration;
pub use year_month_duration::*;

/// XsdDurationStr.
///
/// This is the `durationLexicalRep` production of the XSD 1.1
/// Datatypes specification:
/// <https://www.w3.org/TR/xmlschema11-2/#nt-durationRep>.
///
/// ```abnf
/// durationLexicalRep = [ "-" ] %s"P" ((duYearMonthFrag [ duDayTimeFrag ]) / duDayTimeFrag)
///
/// duYearMonthFrag = (duYearFrag [ duMonthFrag ]) / duMonthFrag
///
/// duYearFrag = 1*DIGIT %s"Y"
///
/// duMonthFrag = 1*DIGIT %s"M"
///
/// duDayTimeFrag = (duDayFrag [ duTimeFrag ]) / duTimeFrag
///
/// duDayFrag = 1*DIGIT %s"D"
///
/// duTimeFrag = %s"T" ((duHourFrag [ duMinuteFrag ] [ duSecondFrag ]) / (duMinuteFrag [ duSecondFrag ]) / duSecondFrag)
///
/// duHourFrag = 1*DIGIT %s"H"
///
/// duMinuteFrag = 1*DIGIT %s"M"
///
/// duSecondFrag = ((1*DIGIT ["." *DIGIT]) / "." 1*DIGIT) %s"S"
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::Duration)]
#[newtype(owned(XsdDurationString, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct XsdDurationStr(str);

impl XsdDurationStr {
	pub fn parts(&self) -> XsdDurationParts<'_> {
		enum State {
			Sign,
			DateNumber,
			TimeNumber,
		}

		let mut state = State::Sign;
		let mut result = XsdDurationParts {
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

impl XsdLexicalForm for XsdDurationStr {
	type Value = crate::XsdDuration;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdDurationStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdDurationStr::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdDurationStr(value)))
	}
}

impl<'a> From<&'a XsdDurationStr> for crate::XsdDuration {
	fn from(value: &'a XsdDurationStr) -> crate::XsdDuration {
		value.parts().to_duration()
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct XsdDurationParts<'a> {
	pub is_negative: bool,
	pub year: Option<&'a str>,
	pub month: Option<&'a str>,
	pub day: Option<&'a str>,
	pub hour: Option<&'a str>,
	pub minute: Option<&'a str>,
	pub second: Option<&'a str>,
}

impl<'a> XsdDurationParts<'a> {
	pub fn new(
		is_negative: bool,
		year: Option<&'a str>,
		month: Option<&'a str>,
		day: Option<&'a str>,
		hour: Option<&'a str>,
		minute: Option<&'a str>,
		second: Option<&'a str>,
	) -> Self {
		Self {
			is_negative,
			year,
			month,
			day,
			hour,
			minute,
			second,
		}
	}
	fn to_duration(&self) -> crate::XsdDuration {
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
			let (s, ns) = parse_seconds_decimal(s);
			seconds += s;
			nano_seconds = ns;
		}

		crate::XsdDuration::new(self.is_negative, months, seconds, nano_seconds)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parsing() {
		let vectors = [
			(
				"P9Y11M",
				XsdDurationParts::new(false, Some("9"), Some("11"), None, None, None, None),
				"P9Y11M",
			),
			(
				"P9Y12M",
				XsdDurationParts::new(false, Some("9"), Some("12"), None, None, None, None),
				"P10Y",
			),
			(
				"-P9Y12M1DT24H01M1.0001S",
				XsdDurationParts::new(
					true,
					Some("9"),
					Some("12"),
					Some("1"),
					Some("24"),
					Some("01"),
					Some("1.0001"),
				),
				"-P10Y2DT1M1.0001S",
			),
		];

		for (input, parts, normalized) in vectors {
			let lexical_repr = XsdDurationStr::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value: crate::XsdDuration = lexical_repr.into();
			assert_eq!(value.to_string().as_str(), normalized)
		}
	}
}
