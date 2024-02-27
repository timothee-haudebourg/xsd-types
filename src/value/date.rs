use chrono::{Datelike, FixedOffset, NaiveDate};

use crate::{
	format_timezone,
	lexical::{InvalidDate, LexicalFormOf},
	Datatype, DisplayYear, ParseXsd, XsdValue,
};
use core::fmt;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
#[error("invalid date value")]
pub struct InvalidDateValue;

#[derive(Debug, Clone, Copy)]
pub struct Date {
	pub date: NaiveDate,
	pub offset: Option<FixedOffset>,
}

impl Date {
	pub fn new(date: NaiveDate, offset: Option<FixedOffset>) -> Self {
		Self { date, offset }
	}
}

#[derive(Debug, thiserror::Error)]
pub enum DateFromStrError {
	#[error("invalid date syntax")]
	Syntax(#[from] InvalidDate<String>),

	#[error(transparent)]
	Value(#[from] InvalidDateValue),
}

impl FromStr for Date {
	type Err = DateFromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value =
			crate::lexical::Date::new(s).map_err(|InvalidDate(s)| InvalidDate(s.to_owned()))?;
		lexical_value.try_as_value().map_err(Into::into)
	}
}

impl XsdValue for Date {
	fn datatype(&self) -> Datatype {
		Datatype::Date
	}
}

impl ParseXsd for Date {
	type LexicalForm = crate::lexical::Date;
}

impl fmt::Display for Date {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}-{:02}-{:02}",
			DisplayYear(self.date.year()),
			self.date.month(),
			self.date.day()
		)?;

		format_timezone(self.offset, f)
	}
}
