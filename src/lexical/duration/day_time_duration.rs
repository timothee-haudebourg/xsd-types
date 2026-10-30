use crate::{lexical::date_time::parse_seconds_decimal, XsdError};

use super::XsdLexicalForm;
use static_automata::Validate;
use str_newtype::StrNewType;

/// Day XsdTimeStr XsdDurationStr.
///
/// This is the `dayTimeDurationLexicalRep` production of the XSD 1.1
/// Datatypes specification:
/// <https://www.w3.org/TR/xmlschema11-2/#nt-dayTimeDurationRep>.
///
/// ```abnf
/// dayTimeDurationLexicalRep = [ "-" ] %s"P" duDayTimeFrag
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
#[automaton(super::super::grammar::DayTimeDuration)]
#[newtype(owned(XsdDayTimeDurationString, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct XsdDayTimeDurationStr(str);

impl XsdDayTimeDurationStr {
	pub fn parts(&self) -> Parts<'_> {
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

impl XsdLexicalForm for XsdDayTimeDurationStr {
	type Value = crate::XsdDayTimeDuration;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdDayTimeDurationStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdDayTimeDurationStr::new(value)
			.map_err(|_| XsdError::lexical_form(InvalidXsdDayTimeDurationStr(value)))
	}
}

impl<'a> From<&'a XsdDayTimeDurationStr> for crate::XsdDayTimeDuration {
	fn from(value: &'a XsdDayTimeDurationStr) -> crate::XsdDayTimeDuration {
		value.parts().to_duration()
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
	fn to_duration(&self) -> crate::XsdDayTimeDuration {
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

		crate::XsdDayTimeDuration::new(self.is_negative, seconds, nano_seconds)
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
			let lexical_repr = XsdDayTimeDurationStr::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value: crate::XsdDayTimeDuration = lexical_repr.into();
			assert_eq!(value.to_string().as_str(), normalized)
		}
	}
}
