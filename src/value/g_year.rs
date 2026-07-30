use crate::{
	format_timezone, is_valid_offset,
	lexical::{InvalidGYear, Lexical, LexicalFormOf},
	seven_property_model_date_time, seven_property_model_eq, seven_property_model_partial_cmp,
	Datatype, DisplayYear, ParseXsd, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

/// A year, optionally with a timezone offset.
///
/// This is the value space of the XSD `gYear` datatype: a decoded year number
/// paired with an optional XSD timezone offset, as opposed to its lexical
/// (string) representation, [`lexical::GYear`](crate::lexical::GYear).
/// Two distinct years are always far enough apart that they remain totally
/// ordered even accounting for the `±14:00` offset uncertainty window.
/// See: <https://www.w3.org/TR/xmlschema11-2/#gYear>.
#[derive(Debug, Clone, Copy)]
pub struct GYear {
	year: i32,
	offset: Option<time::UtcOffset>,
}

impl GYear {
	/// Creates a new `GYear`, or returns `None` if `offset` is outside the
	/// `-14:00..=+14:00` range permitted by XSD.
	pub fn new(year: i32, offset: Option<time::UtcOffset>) -> Option<Self> {
		if offset.is_none_or(is_valid_offset) {
			Some(Self { year, offset })
		} else {
			None
		}
	}

	/// Returns the year.
	pub fn year(&self) -> i32 {
		self.year
	}

	/// Returns the timezone offset, if any.
	pub fn offset(&self) -> Option<time::UtcOffset> {
		self.offset
	}

	/// Deconstructs this `GYear` into its year and timezone offset.
	pub fn into_parts(self) -> (i32, Option<time::UtcOffset>) {
		(self.year, self.offset)
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		seven_property_model_date_time(Some(self.year), None, None, time::Time::MIDNIGHT)
	}
}

impl FromStr for GYear {
	type Err = InvalidGYear<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::GYear::parse(s)?;
		Ok(lexical_value.as_value())
	}
}

impl PartialEq for GYear {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for GYear {}

impl Hash for GYear {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.year.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for GYear {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for GYear {
	fn cmp(&self, other: &Self) -> Ordering {
		// Two distinct years are always roughly a year apart, far more than
		// the `±14:00` offset uncertainty window (28 hours), so `GYear`
		// values are always comparable.
		seven_property_model_partial_cmp(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
		.unwrap()
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

#[cfg(feature = "serde")]
impl serde::Serialize for GYear {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GYear {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = GYear;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#gYear")
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
	use super::GYear;

	#[test]
	fn ord_without_offset_is_direct() {
		let a = GYear::new(2023, None).unwrap();
		let b = GYear::new(2024, None).unwrap();

		assert!(a < b);
	}

	// Two distinct `GYear` values are always roughly a year apart, far more
	// than the `±14:00` offset uncertainty window (28 hours), so they are
	// always totally ordered, even with extreme offsets.
	#[test]
	fn ord_is_always_definite() {
		let a = GYear::new(2023, None).unwrap();
		let b = GYear::new(2024, Some(time::UtcOffset::from_hms(14, 0, 0).unwrap())).unwrap();

		assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
		assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less));
	}

	#[test]
	fn eq_ignores_representation() {
		let a = GYear::new(2023, None).unwrap();
		let b = GYear::new(2023, None).unwrap();

		assert_eq!(a, b);
	}

	#[test]
	fn new_rejects_out_of_range_offset() {
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();
		assert!(GYear::new(2023, Some(offset)).is_none());
	}

	#[test]
	fn from_str_roundtrip() {
		let y: GYear = "2023+05:00".parse().unwrap();
		assert_eq!(y.year(), 2023);
		assert_eq!(
			y.offset(),
			Some(time::UtcOffset::from_hms(5, 0, 0).unwrap())
		);
		assert_eq!(y.to_string(), "2023+05:00");
	}
}
