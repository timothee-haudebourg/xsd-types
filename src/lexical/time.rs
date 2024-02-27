use static_regular_grammar::RegularGrammar;

use crate::{utils::byte_index_of, InvalidTimeValue};

use super::{date_time::parse_seconds_decimal, parse_timezone, Lexical, LexicalFormOf};

/// Time.
///
/// ```abnf
/// xsd-time = time [timezone]
///
/// time = hour ":" minute ":" second ["." fraction]
///      / "24:00:00" ["." 1*"0"]
///
/// hour = ("0" / "1") DIGIT
///      / "2" ("0" / "1" / "2" / "3")
///
/// minute = ("0" / "1" / "2" / "3" / "4" / "5") DIGIT
///
/// second = ("0" / "1" / "2" / "3" / "4" / "5") DIGIT
///
/// fraction = 1*DIGIT
///
/// timezone = ("+" / "-") ((("0" DIGIT / "1" ("0" / "1" / "2" / "3")) ":" minute) / "14:00")
///          / %s"Z"
/// ```
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(sized(TimeBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct Time(str);

impl Time {
	fn parts(&self) -> Parts {
		let seconds_end = byte_index_of(self.0.as_bytes(), 8, [b'+', b'-']).unwrap_or(self.0.len());
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

impl Lexical for Time {
	type Error = InvalidTime<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidTime(value.to_owned()))
	}
}

impl LexicalFormOf<crate::Time> for Time {
	type ValueError = InvalidTimeValue;

	fn try_as_value(&self) -> Result<crate::Time, Self::ValueError> {
		self.parts().to_time()
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

	fn to_time(&self) -> Result<crate::Time, crate::InvalidTimeValue> {
		let (seconds, nanoseconds) = parse_seconds_decimal(self.seconds);

		let time = chrono::NaiveTime::from_hms_nano_opt(
			self.hours.parse().unwrap(),
			self.minutes.parse().unwrap(),
			seconds,
			nanoseconds,
		)
		.ok_or(crate::InvalidTimeValue)?;

		Ok(crate::Time::new(time, self.timezone.map(parse_timezone)))
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
			let lexical_repr = Time::new(input).unwrap();
			assert_eq!(lexical_repr.parts(), parts);

			let value = lexical_repr.try_as_value().unwrap();
			assert_eq!(value.to_string().as_str(), input)
		}
	}
}
