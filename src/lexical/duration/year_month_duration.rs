use super::XsdLexicalForm;
use crate::XsdError;
use static_automata::Validate;
use str_newtype::StrNewType;

/// Year Month XsdDurationStr.
///
/// This is the `yearMonthDurationLexicalRep` production of the XSD 1.1
/// Datatypes specification:
/// <https://www.w3.org/TR/xmlschema11-2/#nt-yearMonthDurationRep>.
///
/// ```abnf
/// yearMonthDurationLexicalRep = [ "-" ] %s"P" duYearMonthFrag
///
/// duYearMonthFrag = (duYearFrag [ duMonthFrag ]) / duMonthFrag
///
/// duYearFrag = 1*DIGIT %s"Y"
///
/// duMonthFrag = 1*DIGIT %s"M"
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::super::grammar::YearMonthDuration)]
#[newtype(owned(
	XsdYearMonthDurationString,
	derive(PartialEq, Eq, PartialOrd, Ord, Hash)
))]
pub struct XsdYearMonthDurationStr(str);

impl XsdYearMonthDurationStr {
	pub fn parts(&self) -> XsdYearMonthDurationParts<'_> {
		enum State {
			Sign,
			DateNumber,
		}

		let mut state = State::Sign;
		let mut result = XsdYearMonthDurationParts {
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

impl XsdLexicalForm for XsdYearMonthDurationStr {
	type Value = crate::XsdYearMonthDuration;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdYearMonthDurationStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdYearMonthDurationStr::new(value)
			.map_err(|_| XsdError::lexical_form(InvalidXsdYearMonthDurationStr(value)))
	}
}

impl<'a> From<&'a XsdYearMonthDurationStr> for crate::XsdYearMonthDuration {
	fn from(value: &'a XsdYearMonthDurationStr) -> crate::XsdYearMonthDuration {
		value.parts().to_duration()
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct XsdYearMonthDurationParts<'a> {
	pub is_negative: bool,
	pub year: Option<&'a str>,
	pub month: Option<&'a str>,
}

impl<'a> XsdYearMonthDurationParts<'a> {
	pub fn new(is_negative: bool, year: Option<&'a str>, month: Option<&'a str>) -> Self {
		Self {
			is_negative,
			year,
			month,
		}
	}
	fn to_duration(&self) -> crate::XsdYearMonthDuration {
		let mut months = 0u32;

		if let Some(y) = self.year {
			let y: u32 = y.parse().unwrap();
			months += y * 12;
		}

		if let Some(m) = self.month {
			let m: u32 = m.parse().unwrap();
			months += m;
		}

		crate::XsdYearMonthDuration::new(self.is_negative, months)
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
				XsdYearMonthDurationParts::new(false, Some("9"), Some("11")),
				"P9Y11M",
			),
			(
				"P9Y12M",
				XsdYearMonthDurationParts::new(false, Some("9"), Some("12")),
				"P10Y",
			),
			(
				"-P9Y12M",
				XsdYearMonthDurationParts::new(true, Some("9"), Some("12")),
				"-P10Y",
			),
		];

		for (input, parts, normalized) in vectors {
			let lexical_repr = XsdYearMonthDurationStr::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value: crate::XsdYearMonthDuration = lexical_repr.into();
			assert_eq!(value.to_string().as_str(), normalized)
		}
	}
}
