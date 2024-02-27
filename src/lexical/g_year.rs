use static_regular_grammar::RegularGrammar;

use crate::lexical::parse_timezone;

use super::{Lexical, LexicalFormOf};

/// GYear.
///
/// ```abnf
/// g-year = year [timezone]
///
/// year = [ "-" ] year-number
///
/// year-number = *3DIGIT NZDIGIT
///             / *2DIGIT NZDIGIT DIGIT
///             / *1DIGIT NZDIGIT 2DIGIT
///             / NZDIGIT 3*DIGIT
///
/// minute = ("0" / "1" / "2" / "3" / "4" / "5") DIGIT
///
/// timezone = ("+" / "-") ((("0" DIGIT / "1" ("0" / "1" / "2" / "3")) ":" minute) / "14:00")
///          / %s"Z"
///
/// NZDIGIT = "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9"
/// ```
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(sized(GYearBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct GYear(str);

impl GYear {
	pub fn parts(&self) -> Parts {
		let year_end = self.as_bytes()[4..]
			.iter()
			.position(|&c| matches!(c, b'+' | b'-' | b'Z'))
			.map(|i| i + 4)
			.unwrap_or(self.0.len());

		Parts {
			year: &self.0[..year_end],
			timezone: if self.0.len() > year_end {
				None
			} else {
				Some(&self.0[year_end..])
			},
		}
	}
}

impl Lexical for GYear {
	type Error = InvalidGYear<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidGYear(value.to_owned()))
	}
}

impl LexicalFormOf<crate::GYear> for GYear {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<crate::GYear, Self::ValueError> {
		Ok(self.parts().to_g_year_month())
	}
}

pub struct Parts<'a> {
	year: &'a str,
	timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	fn to_g_year_month(&self) -> crate::GYear {
		crate::GYear::new(
			self.year.parse().unwrap(),
			self.timezone.map(parse_timezone),
		)
	}
}
