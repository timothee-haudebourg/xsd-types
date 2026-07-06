use crate::{format_timezone, Datatype, DisplayYear, ParseXsd, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct GYear {
	year: i32,
	offset: Option<time::UtcOffset>,
}

impl GYear {
	pub fn new(year: i32, offset: Option<time::UtcOffset>) -> Self {
		Self { year, offset }
	}
}

impl XsdValue for GYear {
	fn datatype(&self) -> Datatype {
		Datatype::GYear
	}
}

impl ParseXsd for GYear {
	type LexicalForm = crate::lexical::GYear;
}

impl fmt::Display for GYear {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		DisplayYear(self.year).fmt(f)?;
		format_timezone(self.offset, f)
	}
}
