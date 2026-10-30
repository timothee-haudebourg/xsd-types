use static_automata::Validate;
use str_newtype::StrNewType;

use crate::{lexical::parse_timezone, utils::byte_index_of, XsdError};

use super::XsdLexicalForm;

/// XsdGYearMonthStr.
///
/// This is the `gYearMonthLexicalRep` production of the XSD 1.1
/// Datatypes specification:
/// <https://www.w3.org/TR/xmlschema11-2/#nt-gYearMonthRep>.
///
/// ```abnf
/// gYearMonthLexicalRep = yearFrag "-" monthFrag [ timezoneFrag ]
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::GYearMonth)]
#[newtype(owned(XsdGYearMonthString, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct XsdGYearMonthStr(str);

impl XsdGYearMonthStr {
	pub fn parts(&self) -> Parts<'_> {
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

impl XsdLexicalForm for XsdGYearMonthStr {
	type Value = crate::XsdGYearMonth;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdGYearMonthStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdGYearMonthStr::new(value)
			.map_err(|_| XsdError::lexical_form(InvalidXsdGYearMonthStr(value)))
	}
}

impl<'a> From<&'a XsdGYearMonthStr> for crate::XsdGYearMonth {
	fn from(value: &'a XsdGYearMonthStr) -> crate::XsdGYearMonth {
		value.parts().to_g_year_month()
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

	fn to_g_year_month(&self) -> crate::XsdGYearMonth {
		crate::XsdGYearMonth::new(
			self.year.parse().unwrap(),
			self.month.parse().unwrap(),
			self.timezone.map(|tz| parse_timezone(tz).unwrap()),
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
			let lexical_repr = XsdGYearMonthStr::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value: crate::XsdGYearMonth = lexical_repr.into();
			assert_eq!(value.to_string().as_str(), input)
		}
	}
}
