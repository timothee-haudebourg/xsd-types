use static_automata::Validate;
use str_newtype::StrNewType;

use crate::{lexical::parse_timezone, utils::byte_index_of, XsdError};

use super::XsdLexicalForm;

/// XsdGYearStr.
///
/// This is the `gYearLexicalRep` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-gYearRep>.
///
/// ```abnf
/// gYearLexicalRep = yearFrag [ timezoneFrag ]
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::GYear)]
#[newtype(owned(XsdGYearString, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct XsdGYearStr(str);

impl XsdGYearStr {
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

impl XsdLexicalForm for XsdGYearStr {
	type Value = crate::XsdGYear;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdGYearStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdGYearStr::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdGYearStr(value)))
	}
}

impl<'a> From<&'a XsdGYearStr> for crate::XsdGYear {
	fn from(value: &'a XsdGYearStr) -> crate::XsdGYear {
		value.parts().to_g_year_month()
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

	fn to_g_year_month(&self) -> crate::XsdGYear {
		// The offset comes from the grammar-validated lexical form, so it is
		// always within the range `XsdGYearStr::new` requires.
		crate::XsdGYear::new(
			self.year.parse().unwrap(),
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
			("2014", Parts::new("2014", None)),
			("-0001Z", Parts::new("-0001", Some("Z"))),
			("10000+05:00", Parts::new("10000", Some("+05:00"))),
		];

		for (input, parts) in vectors {
			let lexical_repr = XsdGYearStr::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value: crate::XsdGYear = lexical_repr.into();
			assert_eq!(value.to_string().as_str(), input)
		}
	}

	/// `yearFrag` requires at least 4 digits, with a leading `0` allowed
	/// only to reach exactly 4 digits (year `0000`, i.e. 1 BCE). See:
	/// <https://www.w3.org/TR/xmlschema11-2/#nt-yrFrag>.
	#[test]
	fn year_zero_accepted() {
		assert!(XsdGYearStr::new("0000").is_ok());
	}

	/// A year fragment shorter than 4 digits is not a valid `yearFrag`,
	/// even though it starts with a nonzero digit.
	#[test]
	fn short_year_rejected() {
		assert!(XsdGYearStr::new("5").is_err());
		assert!(XsdGYearStr::new("05").is_err());
		assert!(XsdGYearStr::new("005").is_err());
	}
}
