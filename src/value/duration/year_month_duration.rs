use crate::{Datatype, DurationDatatype, ParseXsd, XsdValue};
use core::fmt;

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
}

impl XsdValue for YearMonthDuration {
	fn datatype(&self) -> Datatype {
		Datatype::Duration(DurationDatatype::Duration)
	}
}

impl ParseXsd for YearMonthDuration {
	type LexicalForm = crate::lexical::YearMonthDuration;
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
