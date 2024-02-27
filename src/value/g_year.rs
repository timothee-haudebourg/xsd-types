use chrono::FixedOffset;

use crate::{format_timezone, Datatype, DisplayYear, ParseRdf, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct GYear {
	year: i32,
	offset: Option<FixedOffset>,
}

impl GYear {
	pub fn new(year: i32, offset: Option<FixedOffset>) -> Self {
		Self { year, offset }
	}
}

impl XsdValue for GYear {
	fn datatype(&self) -> Datatype {
		Datatype::GYear
	}
}

impl ParseRdf for GYear {
	type LexicalForm = crate::lexical::GYear;
}

impl fmt::Display for GYear {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		DisplayYear(self.year).fmt(f)?;
		format_timezone(self.offset, f)
	}
}
