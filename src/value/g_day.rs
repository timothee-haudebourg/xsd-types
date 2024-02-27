use chrono::FixedOffset;

use crate::{format_timezone, Datatype, ParseXsd, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct GDay {
	day: u8,
	offset: Option<FixedOffset>,
}

impl GDay {
	pub fn new(day: u8, offset: Option<FixedOffset>) -> Option<Self> {
		if (1..=31).contains(&day) {
			Some(Self { day, offset })
		} else {
			None
		}
	}
}

impl XsdValue for GDay {
	fn datatype(&self) -> Datatype {
		Datatype::GDay
	}
}

impl ParseXsd for GDay {
	type LexicalForm = crate::lexical::GDay;
}

impl fmt::Display for GDay {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "---{:02}", self.day)?;

		format_timezone(self.offset, f)
	}
}
