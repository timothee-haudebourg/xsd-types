use crate::{format_timezone, Datatype, ParseXsd, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct GMonth {
	month: u8,
	offset: Option<time::UtcOffset>,
}

impl GMonth {
	pub fn new(month: u8, offset: Option<time::UtcOffset>) -> Option<Self> {
		if (1..=12).contains(&month) {
			Some(Self { month, offset })
		} else {
			None
		}
	}
}

impl XsdValue for GMonth {
	fn datatype(&self) -> Datatype {
		Datatype::GMonth
	}
}

impl ParseXsd for GMonth {
	type LexicalForm = crate::lexical::GMonth;
}

impl fmt::Display for GMonth {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "--{:02}", self.month)?;

		format_timezone(self.offset, f)
	}
}
