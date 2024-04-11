use crate::{format_nanoseconds, Datatype, DurationDatatype, ParseXsd, XsdValue};
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct DayTimeDuration {
	is_negative: bool,
	seconds: u32,
	nano_seconds: u32,
}

impl DayTimeDuration {
	pub fn new(is_negative: bool, mut seconds: u32, mut nano_seconds: u32) -> Self {
		// Normalize nanoseconds.
		let s = nano_seconds / 1_000_000_000;
		if s > 0 {
			seconds += s;
			nano_seconds -= s * 1_000_000_000;
		}

		Self {
			is_negative,
			seconds,
			nano_seconds,
		}
	}
}

impl XsdValue for DayTimeDuration {
	fn datatype(&self) -> Datatype {
		Datatype::Duration(DurationDatatype::Duration)
	}
}

impl ParseXsd for DayTimeDuration {
	type LexicalForm = crate::lexical::DayTimeDuration;
}

impl fmt::Display for DayTimeDuration {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut minute = self.seconds / 60;
		let second = self.seconds - minute * 60;

		let mut hour = minute / 60;
		minute -= hour * 60;

		let day = hour / 24;
		hour -= day * 24;

		if self.is_negative {
			write!(f, "-")?;
		}

		write!(f, "P")?;

		if day > 0 {
			write!(f, "{day}D")?;
		}

		if hour > 0 || minute > 0 || second > 0 || self.nano_seconds > 0 {
			write!(f, "T")?;

			if hour > 0 {
				write!(f, "{hour}H")?;
			}

			if minute > 0 {
				write!(f, "{minute}M")?;
			}

			if second > 0 || self.nano_seconds > 0 {
				if second > 0 {
					second.fmt(f)?;
				}

				format_nanoseconds(self.nano_seconds, f)?;
				write!(f, "S")?;
			}
		}

		Ok(())
	}
}
