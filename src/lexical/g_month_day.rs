use static_automata::Validate;
use str_newtype::StrNewType;

use crate::{lexical::parse_timezone, XsdError};

use super::XsdLexicalForm;

/// GMonthDay.
///
/// This is the `gMonthDayLexicalRep` production of the XSD 1.1
/// Datatypes specification:
/// <https://www.w3.org/TR/xmlschema11-2/#nt-gMonthDayRep>.
///
/// ```abnf
/// gMonthDayLexicalRep = "--" monthFrag "-" dayFrag [ timezoneFrag ]
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::GMonthDay)]
#[newtype(owned(GMonthDayBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct GMonthDay(str);

impl GMonthDay {
	pub fn parts(&self) -> Parts<'_> {
		Parts {
			month: &self.0[2..4],
			day: &self.0[5..7],
			timezone: if self.0.len() > 7 {
				Some(&self.0[7..])
			} else {
				None
			},
		}
	}
}

impl XsdLexicalForm for GMonthDay {
	type Value = crate::GMonthDay;
}

impl<'a> crate::XsdFromStr<'a> for &'a GMonthDay {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		GMonthDay::new(value).map_err(|_| XsdError::lexical_form(InvalidGMonthDay(value)))
	}
}

impl<'a> From<&'a GMonthDay> for crate::GMonthDay {
	fn from(value: &'a GMonthDay) -> crate::GMonthDay {
		value.parts().to_g_month_day()
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a> {
	pub month: &'a str,
	pub day: &'a str,
	pub timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	pub fn new(month: &'a str, day: &'a str, timezone: Option<&'a str>) -> Self {
		Self {
			month,
			day,
			timezone,
		}
	}

	fn to_g_month_day(&self) -> crate::GMonthDay {
		crate::GMonthDay::new(
			self.month.parse().unwrap(),
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
			("--01-12", Parts::new("01", "12", None)),
			("--03-31Z", Parts::new("03", "31", Some("Z"))),
			("--02-20+05:00", Parts::new("02", "20", Some("+05:00"))),
		];

		for (input, parts) in vectors {
			let lexical_repr = GMonthDay::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value: crate::GMonthDay = lexical_repr.into();
			assert_eq!(value.to_string().as_str(), input)
		}
	}
}
