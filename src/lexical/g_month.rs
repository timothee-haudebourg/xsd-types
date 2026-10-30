use static_automata::Validate;
use str_newtype::StrNewType;

use crate::{XsdError, lexical::parse_timezone};

use super::XsdLexicalForm;

/// XsdGMonthStr.
///
/// This is the `gMonthLexicalRep` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-gMonthRep>.
///
/// ```abnf
/// gMonthLexicalRep = "--" monthFrag [ timezoneFrag ]
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::GMonth)]
#[newtype(owned(XsdGMonthString, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct XsdGMonthStr(str);

impl XsdGMonthStr {
	pub fn parts(&self) -> Parts<'_> {
		Parts {
			month: &self.0[2..4],
			timezone: if self.0.len() > 4 {
				Some(&self.0[4..])
			} else {
				None
			},
		}
	}
}

impl XsdLexicalForm for XsdGMonthStr {
	type Value = crate::XsdGMonth;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdGMonthStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdGMonthStr::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdGMonthStr(value)))
	}
}

impl<'a> From<&'a XsdGMonthStr> for crate::XsdGMonth {
	fn from(value: &'a XsdGMonthStr) -> crate::XsdGMonth {
		value.parts().to_g_month()
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a> {
	pub month: &'a str,
	pub timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	pub fn new(month: &'a str, timezone: Option<&'a str>) -> Self {
		Self { month, timezone }
	}

	fn to_g_month(&self) -> crate::XsdGMonth {
		crate::XsdGMonth::new(
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
			("--01", Parts::new("01", None)),
			("--02Z", Parts::new("02", Some("Z"))),
			("--03+05:00", Parts::new("03", Some("+05:00"))),
		];

		for (input, parts) in vectors {
			let lexical_repr = XsdGMonthStr::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value: crate::XsdGMonth = lexical_repr.into();
			assert_eq!(value.to_string().as_str(), input)
		}
	}
}
