use static_automata::Validate;
use str_newtype::StrNewType;

use crate::{lexical::parse_timezone, XsdError};

use super::XsdLexicalForm;

/// GDay.
///
/// This is the `gDayLexicalRep` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-gDayRep>.
///
/// ```abnf
/// gDayLexicalRep = "---" dayFrag [ timezoneFrag ]
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::GDay)]
#[newtype(owned(GDayBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct GDay(str);

impl GDay {
	pub fn parts(&self) -> Parts<'_> {
		Parts {
			day: &self.0[3..5],
			timezone: if self.0.len() > 5 {
				Some(&self.0[5..])
			} else {
				None
			},
		}
	}
}

impl XsdLexicalForm for GDay {
	type Value = crate::GDay;
}

impl<'a> crate::XsdFromStr<'a> for &'a GDay {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		GDay::new(value).map_err(|_| XsdError::lexical_form(InvalidGDay(value)))
	}
}

impl<'a> From<&'a GDay> for crate::GDay {
	fn from(value: &'a GDay) -> crate::GDay {
		value.parts().to_g_day()
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a> {
	pub day: &'a str,
	pub timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	pub fn new(day: &'a str, timezone: Option<&'a str>) -> Self {
		Self { day, timezone }
	}

	fn to_g_day(&self) -> crate::GDay {
		crate::GDay::new(
			self.day.parse().unwrap(),
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
			("---12", Parts::new("12", None)),
			("---31Z", Parts::new("31", Some("Z"))),
			("---20+05:00", Parts::new("20", Some("+05:00"))),
		];

		for (input, parts) in vectors {
			let lexical_repr = GDay::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value: crate::GDay = lexical_repr.into();
			assert_eq!(value.to_string().as_str(), input)
		}
	}
}
