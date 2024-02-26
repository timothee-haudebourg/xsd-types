use chrono::FixedOffset;

use crate::{format_timezone, Datatype, ParseRdf, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct GYearMonth {
	year: i32,
	month: u8,
	offset: Option<FixedOffset>,
}

impl GYearMonth {
	pub fn new(year: i32, month: u8, offset: Option<FixedOffset>) -> Option<Self> {
		if (1..=12).contains(&month) {
			Some(Self {
				year,
				month,
				offset,
			})
		} else {
			None
		}
	}
}

impl XsdValue for GYearMonth {
	fn datatype(&self) -> Datatype {
		Datatype::GYearMonth
	}
}

impl ParseRdf for GYearMonth {
	type LexicalForm = crate::lexical::GYearMonth;
}

impl fmt::Display for GYearMonth {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:04}-{:02}", self.year, self.month)?;

		format_timezone(self.offset, f)
	}
}
