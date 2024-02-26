use static_regular_grammar::RegularGrammar;

use crate::lexical::parse_timezone;

use super::{Lexical, LexicalFormOf};

/// GMonthDay.
///
/// ```abnf
/// date = "--" month "-" day [timezone]
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
#[grammar(sized(GMonthDayBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct GMonthDay(str);

impl GMonthDay {
	pub fn parts(&self) -> Parts {
		Parts {
			month: &self.0[2..4],
			day: &self.0[5..7],
			timezone: if self.0.len() > 7 {
				None
			} else {
				Some(&self.0[7..])
			},
		}
	}
}

impl Lexical for GMonthDay {
	type Error = InvalidGMonthDay<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidGMonthDay(value.to_owned()))
	}
}

impl LexicalFormOf<crate::GMonthDay> for GMonthDay {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<crate::GMonthDay, Self::ValueError> {
		Ok(self.parts().to_g_month_day())
	}
}

pub struct Parts<'a> {
	month: &'a str,
	day: &'a str,
	timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	fn to_g_month_day(&self) -> crate::GMonthDay {
		crate::GMonthDay::new(
			self.month.parse().unwrap(),
			self.day.parse().unwrap(),
			self.timezone.map(parse_timezone),
		)
		.unwrap()
	}
}
