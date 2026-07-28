use static_automata::Validate;
use str_newtype::StrNewType;

use crate::{lexical::parse_timezone, utils::byte_index_of};

use super::{Lexical, LexicalFormOf};

/// GYear.
///
/// This is the `gYearLexicalRep` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-gYearRep>.
///
/// ```abnf
/// gYearLexicalRep = yearFrag [ timezoneFrag ]
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::GYear)]
#[newtype(owned(GYearBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct GYear(str);

impl GYear {
	pub fn parts(&self) -> Parts<'_> {
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
			self.timezone.map(|tz| parse_timezone(tz).unwrap()),
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

	/// `yearFrag` requires at least 4 digits, with a leading `0` allowed
	/// only to reach exactly 4 digits (year `0000`, i.e. 1 BCE). See:
	/// <https://www.w3.org/TR/xmlschema11-2/#nt-yrFrag>.
	#[test]
	fn year_zero_accepted() {
		assert!(GYear::new("0000").is_ok());
	}

	/// A year fragment shorter than 4 digits is not a valid `yearFrag`,
	/// even though it starts with a nonzero digit.
	#[test]
	fn short_year_rejected() {
		assert!(GYear::new("5").is_err());
		assert!(GYear::new("05").is_err());
		assert!(GYear::new("005").is_err());
	}
}
