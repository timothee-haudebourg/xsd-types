use static_regular_grammar::RegularGrammar;

use crate::lexical::parse_timezone;

use super::{Lexical, LexicalFormOf};

/// GDay.
///
/// ```abnf
/// g-day = "---" day [timezone]
///
/// day = "0" NZDIGIT
///     / ("1" / "2") DIGIT
///     / "3" ("0" / "1")
///
/// minute = ("0" / "1" / "2" / "3" / "4" / "5") DIGIT
///
/// timezone = ("+" / "-") ((("0" DIGIT / "1" ("0" / "1" / "2" / "3")) ":" minute) / "14:00")
///          / %s"Z"
///
/// NZDIGIT = "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9"
/// ```
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(sized(GDayBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct GDay(str);

impl GDay {
	pub fn parts(&self) -> Parts {
		Parts {
			day: &self.0[3..5],
			timezone: if self.0.len() > 5 {
				None
			} else {
				Some(&self.0[5..])
			},
		}
	}
}

impl Lexical for GDay {
	type Error = InvalidGDay<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidGDay(value.to_owned()))
	}
}

impl LexicalFormOf<crate::GDay> for GDay {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<crate::GDay, Self::ValueError> {
		Ok(self.parts().to_g_day())
	}
}

pub struct Parts<'a> {
	day: &'a str,
	timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	fn to_g_day(&self) -> crate::GDay {
		crate::GDay::new(self.day.parse().unwrap(), self.timezone.map(parse_timezone)).unwrap()
	}
}
