use crate::{
	lexical::{duration::InvalidYearMonthDuration, LexicalFormOf},
	Datatype, DurationDatatype, ParseXsd, XsdValue,
};
use core::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct YearMonthDuration {
	is_negative: bool,
	months: u32,
}

impl YearMonthDuration {
	pub fn new(is_negative: bool, months: u32) -> Self {
		Self {
			is_negative,
			months,
		}
	}

	pub fn into_string(self) -> String {
		self.to_string()
	}
}

impl XsdValue for YearMonthDuration {
	fn datatype(&self) -> Datatype {
		Datatype::Duration(DurationDatatype::Duration)
	}
}

impl ParseXsd for YearMonthDuration {
	type LexicalForm = crate::lexical::YearMonthDuration;
}

impl FromStr for YearMonthDuration {
	type Err = InvalidYearMonthDuration<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::YearMonthDuration::new(s)
			.map_err(|InvalidYearMonthDuration(s)| InvalidYearMonthDuration(s.to_owned()))?;
		Ok(lexical_value.as_value())
	}
}

impl fmt::Display for YearMonthDuration {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let year = self.months / 12;
		let month = self.months - year * 12;

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

		Ok(())
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for YearMonthDuration {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.into_string().serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for YearMonthDuration {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = YearMonthDuration;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#yearMonthDuration")
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
