use crate::{
	format_nanoseconds,
	lexical::{InvalidDuration, LexicalFormOf},
	Datatype, DurationDatatype, ParseXsd, XsdValue,
};
use core::fmt;
use std::str::FromStr;

pub mod day_time_duration;
pub use day_time_duration::*;

pub mod year_month_duration;
pub use year_month_duration::*;

#[derive(Debug, Clone, Copy)]
pub struct Duration {
	is_negative: bool,
	months: u32,
	seconds: u32,
	nano_seconds: u32,
}

impl Duration {
	pub fn new(is_negative: bool, months: u32, mut seconds: u32, mut nano_seconds: u32) -> Self {
		// Normalize nanoseconds.
		let s = nano_seconds / 1_000_000_000;
		if s > 0 {
			seconds += s;
			nano_seconds -= s * 1_000_000_000;
		}

		Self {
			is_negative,
			months,
			seconds,
			nano_seconds,
		}
	}

	pub fn into_string(self) -> String {
		self.to_string()
	}
}

impl XsdValue for Duration {
	fn datatype(&self) -> Datatype {
		Datatype::Duration(DurationDatatype::Duration)
	}
}

impl ParseXsd for Duration {
	type LexicalForm = crate::lexical::Duration;
}

impl FromStr for Duration {
	type Err = InvalidDuration<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::Duration::new(s)
			.map_err(|InvalidDuration(s)| InvalidDuration(s.to_owned()))?;
		Ok(lexical_value.as_value())
	}
}

impl fmt::Display for Duration {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let year = self.months / 12;
		let month = self.months - year * 12;

		let mut minute = self.seconds / 60;
		let second = self.seconds - minute * 60;

		let mut hour = minute / 60;
		minute -= hour * 60;

		let day = hour / 24;
		hour -= day * 24;

		if self.is_negative {
			write!(f, "-")?;
		}

		write!(f, "P")?;

		if year > 0 {
			write!(f, "{year}Y")?;
		}

		if month > 0 {
			write!(f, "{month}M")?;
		}

		if day > 0 {
			write!(f, "{day}D")?;
		}

		if hour > 0 || minute > 0 || second > 0 || self.nano_seconds > 0 {
			write!(f, "T")?;

			if hour > 0 {
				write!(f, "{hour}H")?;
			}

			if minute > 0 {
				write!(f, "{minute}M")?;
			}

			if second > 0 || self.nano_seconds > 0 {
				if second > 0 {
					second.fmt(f)?;
				}

				format_nanoseconds(self.nano_seconds, f)?;
				write!(f, "S")?;
			}
		}

		Ok(())
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for Duration {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.into_string().serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Duration {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = Duration;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#duration")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				v.parse().map_err(|e| E::custom(e))
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}
