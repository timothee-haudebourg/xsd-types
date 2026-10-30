use static_automata::Validate;
use str_newtype::StrNewType;

use crate::{InvalidTimeValue, XsdError, utils::byte_index_of};

use super::{XsdLexicalForm, date_time::parse_seconds_decimal, parse_timezone};

/// XsdTimeStr.
///
/// This is the `timeLexicalRep` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-timeRep>.
///
/// ```abnf
/// timeLexicalRep = primitiveTimeFrag [ timezoneFrag ]
///
/// primitiveTimeFrag = (hourFrag ":" minuteFrag ":" secondFrag) / endOfDayFrag
/// ```
#[derive(Validate, StrNewType, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[automaton(super::grammar::Time)]
#[newtype(owned(XsdTimeString, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct XsdTimeStr(str);

impl XsdTimeStr {
	fn parts(&self) -> Parts<'_> {
		let seconds_end =
			byte_index_of(self.0.as_bytes(), 8, [b'+', b'-', b'Z']).unwrap_or(self.0.len());
		Parts {
			hours: &self.0[..2],
			minutes: &self.0[3..5],
			seconds: &self.0[6..seconds_end],
			timezone: if seconds_end == self.0.len() {
				None
			} else {
				Some(&self.0[seconds_end..])
			},
		}
	}
}

impl XsdLexicalForm for XsdTimeStr {
	type Value = crate::XsdTime;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdTimeStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdTimeStr::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdTimeStr(value)))
	}
}

impl<'a> TryFrom<&'a XsdTimeStr> for crate::XsdTime {
	type Error = InvalidTimeValue;

	fn try_from(value: &'a XsdTimeStr) -> Result<crate::XsdTime, Self::Error> {
		value.parts().to_time()
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a> {
	pub hours: &'a str,
	pub minutes: &'a str,
	pub seconds: &'a str,
	pub timezone: Option<&'a str>,
}

impl<'a> Parts<'a> {
	pub fn new(
		hours: &'a str,
		minutes: &'a str,
		seconds: &'a str,
		timezone: Option<&'a str>,
	) -> Self {
		Self {
			hours,
			minutes,
			seconds,
			timezone,
		}
	}

	fn to_time(&self) -> Result<crate::XsdTime, crate::InvalidTimeValue> {
		let (seconds, nanoseconds) = parse_seconds_decimal(self.seconds);

		let time = time::Time::from_hms_nano(
			self.hours.parse().unwrap(),
			self.minutes.parse().unwrap(),
			seconds as u8,
			nanoseconds,
		)
		.map_err(|_| crate::InvalidTimeValue)?;

		crate::XsdTime::new(time, self.timezone.map(|tz| parse_timezone(tz).unwrap()))
			.ok_or(crate::InvalidTimeValue)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parsing() {
		let vectors = [
			(
				"13:07:12+01:00",
				Parts::new("13", "07", "12", Some("+01:00")),
			),
			(
				"12:00:00-05:00",
				Parts::new("12", "00", "00", Some("-05:00")),
			),
			(
				"12:00:00.00001-05:00",
				Parts::new("12", "00", "00.00001", Some("-05:00")),
			),
		];

		for (input, parts) in vectors {
			let lexical_repr = XsdTimeStr::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value = crate::XsdTime::try_from(lexical_repr).unwrap();
			assert_eq!(value.to_string().as_str(), input)
		}
	}
}
