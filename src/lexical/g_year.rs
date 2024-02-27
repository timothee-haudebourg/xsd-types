use static_regular_grammar::RegularGrammar;

use crate::{lexical::parse_timezone, utils::byte_index_of};

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
		let year_end =
			byte_index_of(self.0.as_bytes(), 4, [b'+', b'-', b'Z']).unwrap_or(self.0.len());

		Parts {
			year: &self.0[..year_end],
			timezone: if self.0.len() > year_end {
				Some(&self.0[year_end..])
			} else {
				None
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

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a> {
	pub year: &'a str,
	pub timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	pub fn new(year: &'a str, timezone: Option<&'a str>) -> Self {
		Self { year, timezone }
	}

	fn to_g_year_month(&self) -> crate::GYear {
		crate::GYear::new(
			self.year.parse().unwrap(),
			self.timezone.map(parse_timezone),
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parsing() {
		let vectors = [
			("2014", Parts::new("2014", None)),
			("-0001Z", Parts::new("-0001", Some("Z"))),
			("10000+05:00", Parts::new("10000", Some("+05:00"))),
		];

		for (input, parts) in vectors {
			let lexical_repr = GYear::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value = lexical_repr.try_as_value().unwrap();
			assert_eq!(value.to_string().as_str(), input)
		}
	}
}
