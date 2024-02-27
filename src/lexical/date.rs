use static_regular_grammar::RegularGrammar;

use crate::{lexical::parse_timezone, utils::byte_index_of};

use super::{Lexical, LexicalFormOf};

/// Date.
///
/// ```abnf
/// date = year "-" month "-" day [timezone]
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
#[grammar(sized(DateBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct Date(str);

impl Date {
	pub fn parts(&self) -> Parts {
		let year_end = byte_index_of(self.0.as_bytes(), 4, b'-').unwrap();
		let month_end = year_end + 3;
		let day_end = month_end + 3;

		Parts {
			year: &self.0[..year_end],
			month: &self.0[(year_end + 1)..month_end],
			day: &self.0[(month_end + 1)..day_end],
			timezone: if day_end == self.0.len() {
				None
			} else {
				Some(&self.0[day_end..])
			},
		}
	}
}

impl Lexical for Date {
	type Error = InvalidDate<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidDate(value.to_owned()))
	}
}

impl LexicalFormOf<crate::Date> for Date {
	type ValueError = crate::InvalidDateValue;

	fn try_as_value(&self) -> Result<crate::Date, Self::ValueError> {
		self.parts().to_date()
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a> {
	year: &'a str,
	month: &'a str,
	day: &'a str,
	timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	pub fn new(year: &'a str, month: &'a str, day: &'a str, timezone: Option<&'a str>) -> Self {
		Self {
			year,
			month,
			day,
			timezone,
		}
	}

	fn to_date(&self) -> Result<crate::Date, crate::InvalidDateValue> {
		let date = chrono::NaiveDate::from_ymd_opt(
			self.year.parse().unwrap(),
			self.month.parse().unwrap(),
			self.day.parse().unwrap(),
		)
		.ok_or(crate::InvalidDateValue)?;

		Ok(crate::Date::new(date, self.timezone.map(parse_timezone)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parsing() {
		let vectors = [
			(
				"2002-05-31+01:00",
				Parts::new("2002", "05", "31", Some("+01:00")),
			),
			(
				"2002-10-10-05:00",
				Parts::new("2002", "10", "10", Some("-05:00")),
			),
			(
				"202002-10-10-05:00",
				Parts::new("202002", "10", "10", Some("-05:00")),
			),
		];

		for (input, parts) in vectors {
			let lexical_repr = Date::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value = lexical_repr.try_as_value().unwrap();
			assert_eq!(value.to_string().as_str(), input)
		}
	}
}
