use crate::{
	format_nanoseconds,
	lexical::{InvalidDuration, Lexical, LexicalFormOf},
	Datatype, DurationDatatype, ParseXsd, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

pub mod day_time_duration;
pub use day_time_duration::*;

pub mod year_month_duration;
pub use year_month_duration::*;

#[derive(Debug, Clone, Copy)]
pub struct Duration {
	is_negative: bool,
	months: u32,
	seconds: u32,
	nano_seconds: u32,
}

impl Duration {
	pub fn new(is_negative: bool, months: u32, mut seconds: u32, mut nano_seconds: u32) -> Self {
		// Normalize nanoseconds.
		let s = nano_seconds / 1_000_000_000;
		if s > 0 {
			seconds += s;
			nano_seconds -= s * 1_000_000_000;
		}

		Self {
			is_negative,
			months,
			seconds,
			nano_seconds,
		}
	}

	pub fn into_string(self) -> String {
		self.to_string()
	}

	/// Returns the number of months in this duration, negative if this
	/// duration is negative.
	pub fn signed_months(&self) -> i64 {
		if self.is_negative {
			-(self.months as i64)
		} else {
			self.months as i64
		}
	}

	/// Returns the whole number of seconds in this duration, negative if this
	/// duration is negative.
	pub fn signed_seconds(&self) -> i64 {
		if self.is_negative {
			-(self.seconds as i64)
		} else {
			self.seconds as i64
		}
	}

	/// Returns the sub-second nanoseconds in this duration, negative if this
	/// duration is negative.
	pub fn signed_nano_seconds(&self) -> i32 {
		if self.is_negative {
			-(self.nano_seconds as i32)
		} else {
			self.nano_seconds as i32
		}
	}
}

/// Adds this duration to `reference`, following the `dateTimePlusDuration`
/// algorithm: <https://www.w3.org/TR/xmlschema11-2/#adding-durations-to-dateTimes>.
impl std::ops::Add<Duration> for time::PrimitiveDateTime {
	type Output = Self;

	fn add(self, rhs: Duration) -> Self::Output {
		let date = self.date();

		let total_months = i64::from(date.year()) * 12
			+ i64::from(u8::from(date.month()) - 1)
			+ rhs.signed_months();
		let year = i32::try_from(total_months.div_euclid(12)).expect("duration out of range");
		let month = time::Month::try_from((total_months.rem_euclid(12) + 1) as u8).unwrap();
		let day = date.day().min(time::util::days_in_month(month, year));

		let date_time = time::PrimitiveDateTime::new(
			time::Date::from_calendar_date(year, month, day).expect("duration out of range"),
			self.time(),
		);

		date_time + time::Duration::new(rhs.signed_seconds(), rhs.signed_nano_seconds())
	}
}

const fn unwrap_date(result: Result<time::Date, time::error::ComponentRange>) -> time::Date {
	match result {
		Ok(date) => date,
		Err(_) => panic!("invalid reference date"),
	}
}

/// The four reference date/times used by the order relation on `duration`:
/// <https://www.w3.org/TR/xmlschema11-2/#duration>.
const DURATION_ORDER_REFERENCES: [time::PrimitiveDateTime; 4] = [
	time::PrimitiveDateTime::new(
		unwrap_date(time::Date::from_calendar_date(
			1696,
			time::Month::September,
			1,
		)),
		time::Time::MIDNIGHT,
	),
	time::PrimitiveDateTime::new(
		unwrap_date(time::Date::from_calendar_date(
			1697,
			time::Month::February,
			1,
		)),
		time::Time::MIDNIGHT,
	),
	time::PrimitiveDateTime::new(
		unwrap_date(time::Date::from_calendar_date(1903, time::Month::March, 1)),
		time::Time::MIDNIGHT,
	),
	time::PrimitiveDateTime::new(
		unwrap_date(time::Date::from_calendar_date(1903, time::Month::July, 1)),
		time::Time::MIDNIGHT,
	),
];

impl PartialEq for Duration {
	fn eq(&self, other: &Self) -> bool {
		self.signed_months() == other.signed_months()
			&& self.signed_seconds() == other.signed_seconds()
			&& self.signed_nano_seconds() == other.signed_nano_seconds()
	}
}

impl Eq for Duration {}

impl Hash for Duration {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.signed_months().hash(state);
		self.signed_seconds().hash(state);
		self.signed_nano_seconds().hash(state);
	}
}

/// `duration` only has a partial order: two values are ordered the same way
/// as one another if and only if adding each of them to every one of the
/// four reference date/times produces results in that same relative order.
/// See <https://www.w3.org/TR/xmlschema11-2/#duration>.
impl PartialOrd for Duration {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let mut result = None;

		for reference in DURATION_ORDER_REFERENCES {
			let cmp = (reference + *self).cmp(&(reference + *other));

			match result {
				None => result = Some(cmp),
				Some(previous) if previous == cmp => (),
				Some(_) => return None,
			}
		}

		result
	}
}

impl XsdValue for Duration {
	fn datatype(&self) -> Datatype {
		Datatype::Duration(DurationDatatype::Duration)
	}
}

impl ParseXsd for Duration {
	type LexicalForm = crate::lexical::Duration;
}

impl FromStr for Duration {
	type Err = InvalidDuration<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::Duration::parse(s)?;
		Ok(lexical_value.as_value())
	}
}

impl fmt::Display for Duration {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let year = self.months / 12;
		let month = self.months - year * 12;

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

		if year > 0 {
			write!(f, "{year}Y")?;
		}

		if month > 0 {
			write!(f, "{month}M")?;
		}

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

#[cfg(feature = "serde")]
impl serde::Serialize for Duration {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Duration {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = Duration;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#duration")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				v.parse().map_err(|e| E::custom(e))
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::str::FromStr;

	#[test]
	fn equality_ignores_sign_of_zero() {
		let a = Duration::from_str("PT0S").unwrap();
		let b = Duration::from_str("-PT0S").unwrap();
		assert_eq!(a, b);
	}

	#[test]
	fn one_year_equals_twelve_months() {
		let a = Duration::from_str("P1Y").unwrap();
		let b = Duration::from_str("P12M").unwrap();
		assert_eq!(a, b);
		assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
	}

	#[test]
	fn one_day_is_less_than_two_days() {
		let a = Duration::from_str("P1D").unwrap();
		let b = Duration::from_str("P2D").unwrap();
		assert!(a < b);
	}

	// From the spec's note on order (§2.2.3 and §3.3.6.1):
	// 1697-02-01T00:00:00Z + P1M < 1697-02-01T00:00:00Z + P30D, but
	// 1903-03-01T00:00:00Z + P1M > 1903-03-01T00:00:00Z + P30D, so P1M <> P30D.
	#[test]
	fn one_month_and_thirty_days_are_incomparable() {
		let one_month = Duration::from_str("P1M").unwrap();
		let thirty_days = Duration::from_str("P30D").unwrap();
		assert_eq!(one_month.partial_cmp(&thirty_days), None);
		assert_ne!(one_month, thirty_days);
	}

	#[test]
	fn negative_duration_is_less_than_positive() {
		let a = Duration::from_str("-P1D").unwrap();
		let b = Duration::from_str("P1D").unwrap();
		assert!(a < b);
	}
}
