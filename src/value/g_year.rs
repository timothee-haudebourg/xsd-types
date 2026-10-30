use crate::{
	Datatype, DisplayYear, XsdFromStr, XsdTyped, XsdValue, format_timezone, is_valid_offset,
	lexical::InvalidXsdGYearStr, seven_property_model_date_time, seven_property_model_eq,
	seven_property_model_partial_cmp,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

/// A year, optionally with a timezone offset.
///
/// This is the value space of the XSD `gYear` datatype: a decoded year number
/// paired with an optional XSD timezone offset, as opposed to its lexical
/// (string) representation, [`lexical::XsdGYearStr`](crate::lexical::XsdGYearStr).
/// Two distinct years are always far enough apart that they remain totally
/// ordered even accounting for the `±14:00` offset uncertainty window.
/// See: <https://www.w3.org/TR/xmlschema11-2/#gYear>.
#[derive(Debug, Clone, Copy)]
pub struct XsdGYear {
	year: i32,
	offset: Option<time::UtcOffset>,
}

impl XsdGYear {
	/// Creates a new `XsdGYear`, or returns `None` if `offset` is outside the
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

	/// Deconstructs this `XsdGYear` into its year and timezone offset.
	pub fn into_parts(self) -> (i32, Option<time::UtcOffset>) {
		(self.year, self.offset)
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		seven_property_model_date_time(Some(self.year), None, None, time::Time::MIDNIGHT)
	}
}

impl FromStr for XsdGYear {
	type Err = InvalidXsdGYearStr<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value =
			crate::lexical::XsdGYearStr::new(s).map_err(|_| InvalidXsdGYearStr(s.to_owned()))?;
		Ok(lexical_value.into())
	}
}

impl PartialEq for XsdGYear {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for XsdGYear {}

impl Hash for XsdGYear {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.year.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for XsdGYear {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for XsdGYear {
	fn cmp(&self, other: &Self) -> Ordering {
		// Two distinct years are always roughly a year apart, far more than
		// the `±14:00` offset uncertainty window (28 hours), so `XsdGYear`
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

impl XsdTyped for XsdGYear {
	fn datatype(&self) -> Datatype {
		Datatype::GYear
	}
}

impl XsdValue for XsdGYear {
	type LexicalForm = crate::lexical::XsdGYearStr;

	fn to_lexical_form(&self) -> crate::lexical::XsdGYearString {
		// `Display` emits the canonical lexical form by contract, so it is
		// always valid and this cannot panic.
		crate::lexical::XsdGYearString::new(self.to_string())
			.expect("Display emits a valid gYear lexical form")
	}
}

impl<'a> XsdFromStr<'a> for XsdGYear {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<crate::lexical::XsdGYearStr, Self>(lexical_value)
	}
}

impl fmt::Display for XsdGYear {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		DisplayYear(self.year).fmt(f)?;
		format_timezone(self.offset, f)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdGYear {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdGYear {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdGYear;

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
	use super::XsdGYear;
	use crate::XsdValue;

	#[test]
	fn to_lexical_form_roundtrip() {
		for s in ["2024", "-0044Z", "0001+14:00", "1999-05:30"] {
			let value: XsdGYear = s.parse().unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), s);
			assert_eq!(lexical.as_str().parse::<XsdGYear>().unwrap(), value);
		}
	}

	#[test]
	fn ord_without_offset_is_direct() {
		let a = XsdGYear::new(2023, None).unwrap();
		let b = XsdGYear::new(2024, None).unwrap();

		assert!(a < b);
	}

	// Two distinct `XsdGYear` values are always roughly a year apart, far more
	// than the `±14:00` offset uncertainty window (28 hours), so they are
	// always totally ordered, even with extreme offsets.
	#[test]
	fn ord_is_always_definite() {
		let a = XsdGYear::new(2023, None).unwrap();
		let b = XsdGYear::new(2024, Some(time::UtcOffset::from_hms(14, 0, 0).unwrap())).unwrap();

		assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
		assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less));
	}

	#[test]
	fn eq_ignores_representation() {
		let a = XsdGYear::new(2023, None).unwrap();
		let b = XsdGYear::new(2023, None).unwrap();

		assert_eq!(a, b);
	}

	#[test]
	fn new_rejects_out_of_range_offset() {
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();
		assert!(XsdGYear::new(2023, Some(offset)).is_none());
	}

	#[test]
	fn from_str_roundtrip() {
		let y: XsdGYear = "2023+05:00".parse().unwrap();
		assert_eq!(y.year(), 2023);
		assert_eq!(
			y.offset(),
			Some(time::UtcOffset::from_hms(5, 0, 0).unwrap())
		);
		assert_eq!(y.to_string(), "2023+05:00");
	}
}
