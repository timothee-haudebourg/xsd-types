use chrono::{FixedOffset, Utc};
use std::{fmt, str::FromStr};

use crate::{Datatype, ParseRdf, XsdValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime(chrono::DateTime<FixedOffset>);

impl DateTime {
	/// Returns a `DateTime` which corresponds to the current time and date.
	pub fn now() -> Self {
		Self(Utc::now().into())
	}

	pub fn into_string(self) -> String {
		self.0.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true)
	}
}

impl XsdValue for DateTime {
	fn datatype(&self) -> Datatype {
		Datatype::DateTime
	}
}

impl ParseRdf for DateTime {
	type LexicalForm = crate::lexical::DateTime;
}

impl fmt::Display for DateTime {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.into_string().fmt(f)
	}
}

impl FromStr for DateTime {
	type Err = chrono::format::ParseError;

	fn from_str(date_time: &str) -> Result<Self, Self::Err> {
		Ok(Self(chrono::DateTime::parse_from_rfc3339(date_time)?))
	}
}

impl From<chrono::DateTime<FixedOffset>> for DateTime {
	fn from(value: chrono::DateTime<FixedOffset>) -> Self {
		Self(value)
	}
}

impl From<chrono::DateTime<Utc>> for DateTime {
	fn from(value: chrono::DateTime<Utc>) -> Self {
		Self(value.into())
	}
}

impl From<DateTime> for chrono::DateTime<FixedOffset> {
	fn from(value: DateTime) -> Self {
		value.0
	}
}

impl From<DateTime> for chrono::DateTime<Utc> {
	fn from(value: DateTime) -> Self {
		value.0.into()
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for DateTime {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.into_string().serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DateTime {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = DateTime;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#dateTime")
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
