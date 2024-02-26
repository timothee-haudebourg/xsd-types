use static_regular_grammar::RegularGrammar;

use crate::lexical::parse_timezone;

use super::{Lexical, LexicalFormOf};

/// GYearMonth.
///
/// ```abnf
/// date = year "-" month [timezone]
///
/// year = [ "-" ] year-number
///
/// year-number = *3DIGIT NZDIGIT
///             / *2DIGIT NZDIGIT DIGIT
///             / *1DIGIT NZDIGIT 2DIGIT
///             / NZDIGIT 3*DIGIT
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
#[grammar(sized(GYearMonthBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct GYearMonth(str);

impl GYearMonth {
	pub fn parts(&self) -> Parts {
		let year_end = self.as_bytes()[4..]
			.iter()
			.position(|&c| c == b'-')
			.unwrap() + 4;
		let month_end = year_end + 3;

		Parts {
			year: &self.0[..year_end],
			month: &self.0[(year_end + 1)..month_end],
			timezone: if self.0.len() > month_end {
				None
			} else {
				Some(&self.0[month_end..])
			},
		}
	}
}

impl Lexical for GYearMonth {
	type Error = InvalidGYearMonth<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidGYearMonth(value.to_owned()))
	}
}

impl LexicalFormOf<crate::GYearMonth> for GYearMonth {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<crate::GYearMonth, Self::ValueError> {
		Ok(self.parts().to_g_year_month())
	}
}

pub struct Parts<'a> {
	year: &'a str,
	month: &'a str,
	timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	fn to_g_year_month(&self) -> crate::GYearMonth {
		crate::GYearMonth::new(
			self.year.parse().unwrap(),
			self.month.parse().unwrap(),
			self.timezone.map(parse_timezone),
		)
		.unwrap()
	}
}
