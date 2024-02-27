use static_regular_grammar::RegularGrammar;

use crate::{lexical::parse_timezone, utils::byte_index_of};

use super::{Lexical, LexicalFormOf};

/// GYearMonth.
///
/// ```abnf
/// g-year-month = year "-" month [timezone]
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
/// minute = ("0" / "1" / "2" / "3" / "4" / "5") DIGIT
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
		let year_end = byte_index_of(self.0.as_bytes(), 4, b'-').unwrap();
		let month_end = year_end + 3;

		Parts {
			year: &self.0[..year_end],
			month: &self.0[(year_end + 1)..month_end],
			timezone: if self.0.len() > month_end {
				Some(&self.0[month_end..])
			} else {
				None
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

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a> {
	pub year: &'a str,
	pub month: &'a str,
	pub timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	pub fn new(year: &'a str, month: &'a str, timezone: Option<&'a str>) -> Self {
		Self {
			year,
			month,
			timezone,
		}
	}

	fn to_g_year_month(&self) -> crate::GYearMonth {
		crate::GYearMonth::new(
			self.year.parse().unwrap(),
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
			("2014-01", Parts::new("2014", "01", None)),
			("-0001-02Z", Parts::new("-0001", "02", Some("Z"))),
			("10000-03+05:00", Parts::new("10000", "03", Some("+05:00"))),
		];

		for (input, parts) in vectors {
			let lexical_repr = GYearMonth::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value = lexical_repr.try_as_value().unwrap();
			assert_eq!(value.to_string().as_str(), input)
		}
	}
}
