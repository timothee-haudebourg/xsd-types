use crate::{
	format_nanoseconds,
	lexical::{duration::InvalidDayTimeDuration, Lexical, LexicalFormOf},
	Datatype, DurationDatatype, ParseXsd, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub struct DayTimeDuration {
	is_negative: bool,
	seconds: u32,
	nano_seconds: u32,
}

impl DayTimeDuration {
	pub fn new(is_negative: bool, mut seconds: u32, mut nano_seconds: u32) -> Self {
		// Normalize nanoseconds.
		let s = nano_seconds / 1_000_000_000;
		if s > 0 {
			seconds += s;
			nano_seconds -= s * 1_000_000_000;
		}

		Self {
			is_negative,
			seconds,
			nano_seconds,
		}
	}

	pub fn into_string(self) -> String {
		self.to_string()
	}

	/// Returns the whole number of seconds in this duration, negative if
	/// this duration is negative.
	pub fn signed_seconds(&self) -> i64 {
		if self.is_negative {
			-(self.seconds as i64)
		} else {
			self.seconds as i64
		}
	}

	/// Returns the sub-second nanoseconds in this duration, negative if this
	/// duration is negative.
	pub fn signed_nano_seconds(&self) -> i32 {
		if self.is_negative {
			-(self.nano_seconds as i32)
		} else {
			self.nano_seconds as i32
		}
	}
}

impl PartialEq for DayTimeDuration {
	fn eq(&self, other: &Self) -> bool {
		self.signed_seconds() == other.signed_seconds()
			&& self.signed_nano_seconds() == other.signed_nano_seconds()
	}
}

impl Eq for DayTimeDuration {}

impl Hash for DayTimeDuration {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.signed_seconds().hash(state);
		self.signed_nano_seconds().hash(state);
	}
}

/// Since `months` is always zero, `dayTimeDuration` is totally ordered by its
/// number of seconds: <https://www.w3.org/TR/xmlschema11-2/#dayTimeDuration>.
impl PartialOrd for DayTimeDuration {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for DayTimeDuration {
	fn cmp(&self, other: &Self) -> Ordering {
		self.signed_seconds()
			.cmp(&other.signed_seconds())
			.then_with(|| self.signed_nano_seconds().cmp(&other.signed_nano_seconds()))
	}
}

impl XsdValue for DayTimeDuration {
	fn datatype(&self) -> Datatype {
		Datatype::Duration(DurationDatatype::DayTimeDuration)
	}
}

impl ParseXsd for DayTimeDuration {
	type LexicalForm = crate::lexical::DayTimeDuration;
}

impl FromStr for DayTimeDuration {
	type Err = InvalidDayTimeDuration<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::DayTimeDuration::parse(s)?;
		Ok(lexical_value.as_value())
	}
}

impl fmt::Display for DayTimeDuration {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
impl serde::Serialize for DayTimeDuration {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DayTimeDuration {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = DayTimeDuration;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#dayTimeDuration")
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
