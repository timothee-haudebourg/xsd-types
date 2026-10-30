use static_automata::Validate;
use str_newtype::StrNewType;

use crate::{XsdError, lexical::parse_timezone, utils::byte_index_of};

use super::XsdLexicalForm;

/// XsdDateStr.
///
/// This is the `dateLexicalRep` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-dateRep>.
///
/// ```abnf
/// dateLexicalRep = yearFrag "-" monthFrag "-" dayFrag [ timezoneFrag ]
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::Date)]
#[newtype(owned(XsdDateString, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct XsdDateStr(str);

impl XsdDateStr {
	pub fn parts(&self) -> Parts<'_> {
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

impl XsdLexicalForm for XsdDateStr {
	type Value = crate::XsdDate;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdDateStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdDateStr::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdDateStr(value)))
	}
}

impl<'a> TryFrom<&'a XsdDateStr> for crate::XsdDate {
	type Error = crate::InvalidXsdDateValue;

	fn try_from(value: &'a XsdDateStr) -> Result<crate::XsdDate, Self::Error> {
		value.parts().to_date()
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a> {
	pub year: &'a str,
	pub month: &'a str,
	pub day: &'a str,
	pub timezone: Option<&'a str>,
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

	fn to_date(&self) -> Result<crate::XsdDate, crate::InvalidXsdDateValue> {
		let year = self.year.parse().map_err(|_| crate::InvalidXsdDateValue)?;
		let day = self.day.parse().map_err(|_| crate::InvalidXsdDateValue)?;

		let month = self
			.month
			.parse::<u8>()
			.map_err(|_| crate::InvalidXsdDateValue)?;
		let month = time::Month::try_from(month).map_err(|_| crate::InvalidXsdDateValue)?;

		let date = time::Date::from_calendar_date(year, month, day)
			.map_err(|_| crate::InvalidXsdDateValue)?;

		let offset = match self.timezone {
			Some(tz) => Some(parse_timezone(tz).ok_or(crate::InvalidXsdDateValue)?),
			None => None,
		};

		crate::XsdDate::new(date, offset).ok_or(crate::InvalidXsdDateValue)
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
		];

		for (input, parts) in vectors {
			let lexical_repr = XsdDateStr::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value = crate::XsdDate::try_from(lexical_repr).unwrap();
			assert_eq!(value.to_string().as_str(), input)
		}
	}

	/// `yearFrag` requires at least 4 digits, with a leading `0` allowed
	/// only to reach exactly 4 digits (year `0000`, i.e. 1 BCE). See:
	/// <https://www.w3.org/TR/xmlschema11-2/#nt-yrFrag>.
	#[test]
	fn year_zero_accepted() {
		assert!(XsdDateStr::new("0000-01-01").is_ok());
	}

	/// A year fragment shorter than 4 digits is not a valid `yearFrag`,
	/// even though it starts with a nonzero digit.
	#[test]
	fn short_year_rejected() {
		assert!(XsdDateStr::new("5-01-01").is_err());
		assert!(XsdDateStr::new("05-01-01").is_err());
		assert!(XsdDateStr::new("005-01-01").is_err());
	}

	/// Years outside of `-9999..=9999` require the `large-dates` feature of
	/// the `time` crate.
	#[cfg(feature = "large-dates")]
	#[test]
	fn parsing_large_dates() {
		let vectors = [(
			"202002-10-10-05:00",
			Parts::new("202002", "10", "10", Some("-05:00")),
		)];

		for (input, parts) in vectors {
			let lexical_repr = XsdDateStr::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value = crate::XsdDate::try_from(lexical_repr).unwrap();
			assert_eq!(value.to_string().as_str(), input)
		}
	}
}
