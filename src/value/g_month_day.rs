use chrono::FixedOffset;

use crate::{format_timezone, Datatype, ParseXsd, XsdValue};
use core::fmt;

const MONTH_MAX_LEN: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[derive(Debug, Clone, Copy)]
pub struct GMonthDay {
	month: u8,
	day: u8,
	offset: Option<FixedOffset>,
}

impl GMonthDay {
	pub fn new(month: u8, day: u8, offset: Option<FixedOffset>) -> Option<Self> {
		if month > 0 {
			let max_day = *MONTH_MAX_LEN.get(month as usize - 1)?;
			if (1..=max_day).contains(&day) {
				Some(Self { month, day, offset })
			} else {
				None
			}
		} else {
			None
		}
	}
}

impl XsdValue for GMonthDay {
	fn datatype(&self) -> Datatype {
		Datatype::GMonthDay
	}
}

impl ParseXsd for GMonthDay {
	type LexicalForm = crate::lexical::GMonthDay;
}

impl fmt::Display for GMonthDay {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "--{:02}-{:02}", self.month, self.day)?;

		format_timezone(self.offset, f)
	}
}
