use static_regular_grammar::RegularGrammar;

use crate::lexical::parse_timezone;

use super::{Lexical, LexicalFormOf};

/// GMonth.
///
/// ```abnf
/// date = "--" month [timezone]
///
/// month = "0" NZDIGIT
///       / "1" ( "0" / "1" / "2" )
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
#[grammar(sized(GMonthBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct GMonth(str);

impl GMonth {
	pub fn parts(&self) -> Parts {
		Parts {
			month: &self.0[2..4],
			timezone: if self.0.len() > 4 {
				None
			} else {
				Some(&self.0[4..])
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

pub struct Parts<'a> {
	month: &'a str,
	timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	fn to_g_month(&self) -> crate::GMonth {
		crate::GMonth::new(
			self.month.parse().unwrap(),
			self.timezone.map(parse_timezone),
		)
		.unwrap()
	}
}
