use chrono::{FixedOffset, Utc};
use std::{fmt, str::FromStr};

use crate::{Datatype, XsdDatatype};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime(chrono::DateTime<FixedOffset>);

impl DateTime {
	pub fn into_string(self) -> String {
		self.0.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true)
	}
}

impl XsdDatatype for DateTime {
	fn type_(&self) -> Datatype {
		Datatype::DateTime
	}
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
