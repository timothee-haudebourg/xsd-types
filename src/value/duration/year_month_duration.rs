use crate::{
	lexical::{duration::InvalidYearMonthDuration, Lexical, LexicalFormOf},
	Datatype, DurationDatatype, ParseXsd, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

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

	/// Returns the number of months in this duration, negative if this
	/// duration is negative.
	pub fn signed_months(&self) -> i64 {
		if self.is_negative {
			-(self.months as i64)
		} else {
			self.months as i64
		}
	}
}

impl PartialEq for YearMonthDuration {
	fn eq(&self, other: &Self) -> bool {
		self.signed_months() == other.signed_months()
	}
}

impl Eq for YearMonthDuration {}

impl Hash for YearMonthDuration {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.signed_months().hash(state);
	}
}

/// Since `seconds` is always zero, `yearMonthDuration` is totally ordered by
/// its number of months: <https://www.w3.org/TR/xmlschema11-2/#yearMonthDuration>.
impl PartialOrd for YearMonthDuration {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for YearMonthDuration {
	fn cmp(&self, other: &Self) -> Ordering {
		self.signed_months().cmp(&other.signed_months())
	}
}

impl XsdValue for YearMonthDuration {
	fn datatype(&self) -> Datatype {
		Datatype::Duration(DurationDatatype::YearMonthDuration)
	}
}

impl ParseXsd for YearMonthDuration {
	type LexicalForm = crate::lexical::YearMonthDuration;
}

impl FromStr for YearMonthDuration {
	type Err = InvalidYearMonthDuration<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::YearMonthDuration::parse(s)?;
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
		serializer.collect_str(self)
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
