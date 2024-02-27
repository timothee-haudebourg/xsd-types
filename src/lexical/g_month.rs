use static_regular_grammar::RegularGrammar;

use crate::lexical::parse_timezone;

use super::{Lexical, LexicalFormOf};

/// GMonth.
///
/// ```abnf
/// g-month = "--" month [timezone]
///
/// month = "0" NZDIGIT
///       / "1" ( "0" / "1" / "2" )
///
/// minute = ("0" / "1" / "2" / "3" / "4" / "5") DIGIT
///
/// timezone = ("+" / "-") ((("0" DIGIT / "1" ("0" / "1" / "2" / "3")) ":" minute) / "14:00")
///          / %s"Z"
///
/// NZDIGIT = "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9"
/// ```
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(sized(GMonthBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct GMonth(str);

impl GMonth {
	pub fn parts(&self) -> Parts {
		Parts {
			month: &self.0[2..4],
			timezone: if self.0.len() > 4 {
				Some(&self.0[4..])
			} else {
				None
			},
		}
	}
}

impl Lexical for GMonth {
	type Error = InvalidGMonth<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidGMonth(value.to_owned()))
	}
}

impl LexicalFormOf<crate::GMonth> for GMonth {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<crate::GMonth, Self::ValueError> {
		Ok(self.parts().to_g_month())
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a> {
	pub month: &'a str,
	pub timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	pub fn new(month: &'a str, timezone: Option<&'a str>) -> Self {
		Self { month, timezone }
	}

	fn to_g_month(&self) -> crate::GMonth {
		crate::GMonth::new(
			self.month.parse().unwrap(),
			self.timezone.map(parse_timezone),
		)
		.unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parsing() {
		let vectors = [
			("--01", Parts::new("01", None)),
			("--02Z", Parts::new("02", Some("Z"))),
			("--03+05:00", Parts::new("03", Some("+05:00"))),
		];

		for (input, parts) in vectors {
			let lexical_repr = GMonth::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value = lexical_repr.try_as_value().unwrap();
			assert_eq!(value.to_string().as_str(), input)
		}
	}
}
