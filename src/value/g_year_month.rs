use crate::{
	DisplayYear, XsdDatatype, XsdFromStr, XsdTyped, XsdValueDomain, format_timezone,
	is_valid_offset, lexical::InvalidXsdGYearMonthStr, seven_property_model_date_time,
	seven_property_model_eq, seven_property_model_partial_cmp,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

/// A year and month, optionally with a timezone offset.
///
/// This is the value space of the XSD `gYearMonth` datatype: decoded year
/// and month numbers paired with an optional XSD timezone offset, as opposed
/// to its lexical (string) representation,
/// [`lexical::XsdGYearMonthStr`](crate::lexical::XsdGYearMonthStr).
/// Two distinct (year, month) pairs are always far enough apart (at least 28
/// days) that they remain totally ordered even accounting for the `±14:00`
/// offset uncertainty window.
/// See: <https://www.w3.org/TR/xmlschema11-2/#gYearMonth>.
#[derive(Debug, Clone, Copy)]
pub struct XsdGYearMonth {
	year: i32,
	month: u8,
	offset: Option<time::UtcOffset>,
}

impl XsdGYearMonth {
	/// Creates a new `XsdGYearMonth`, or returns `None` if `month` is outside
	/// `1..=12`, or `offset` is outside the `-14:00..=+14:00` range permitted
	/// by XSD.
	pub fn new(year: i32, month: u8, offset: Option<time::UtcOffset>) -> Option<Self> {
		if (1..=12).contains(&month) && offset.is_none_or(is_valid_offset) {
			Some(Self {
				year,
				month,
				offset,
			})
		} else {
			None
		}
	}

	/// Returns the year.
	pub fn year(&self) -> i32 {
		self.year
	}

	/// Returns the month, in `1..=12`.
	pub fn month(&self) -> u8 {
		self.month
	}

	/// Returns the timezone offset, if any.
	pub fn offset(&self) -> Option<time::UtcOffset> {
		self.offset
	}

	/// Deconstructs this `XsdGYearMonth` into its year, month and timezone
	/// offset.
	pub fn into_parts(self) -> (i32, u8, Option<time::UtcOffset>) {
		(self.year, self.month, self.offset)
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		// `self.month` is guaranteed to be in `1..=12` by `Self::new`.
		let month = time::Month::try_from(self.month).unwrap();
		seven_property_model_date_time(Some(self.year), Some(month), None, time::Time::MIDNIGHT)
	}
}

impl FromStr for XsdGYearMonth {
	type Err = InvalidXsdGYearMonthStr<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::XsdGYearMonthStr::new(s)
			.map_err(|_| InvalidXsdGYearMonthStr(s.to_owned()))?;
		Ok(lexical_value.into())
	}
}

impl PartialEq for XsdGYearMonth {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for XsdGYearMonth {}

impl Hash for XsdGYearMonth {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.year.hash(state);
		self.month.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for XsdGYearMonth {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for XsdGYearMonth {
	fn cmp(&self, other: &Self) -> Ordering {
		// Two distinct (year, month) pairs are always at least 28 days apart,
		// far more than the `±14:00` offset uncertainty window (28 hours), so
		// `XsdGYearMonth` values are always comparable.
		seven_property_model_partial_cmp(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
		.unwrap()
	}
}

impl XsdTyped for XsdGYearMonth {
	fn datatype(&self) -> XsdDatatype {
		XsdDatatype::GYearMonth
	}
}

impl XsdValueDomain for XsdGYearMonth {
	type LexicalForm = crate::lexical::XsdGYearMonthStr;

	fn to_lexical_form(&self) -> crate::lexical::XsdGYearMonthString {
		// `Display` emits the canonical lexical form by contract, so it is
		// always valid and this cannot panic.
		crate::lexical::XsdGYearMonthString::new(self.to_string())
			.expect("Display emits a valid gYearMonth lexical form")
	}
}

impl<'a> XsdFromStr<'a> for XsdGYearMonth {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<crate::lexical::XsdGYearMonthStr, Self>(lexical_value)
	}
}

impl fmt::Display for XsdGYearMonth {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}-{:02}", DisplayYear(self.year), self.month)?;

		format_timezone(self.offset, f)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdGYearMonth {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdGYearMonth {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdGYearMonth;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#gYearMonth")
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
	use super::XsdGYearMonth;
	use crate::XsdValueDomain;

	#[test]
	fn to_lexical_form_roundtrip() {
		for s in ["2024-02", "-0044-12Z", "1999-01-05:30", "0001-07+14:00"] {
			let value: XsdGYearMonth = s.parse().unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), s);
			assert_eq!(lexical.as_str().parse::<XsdGYearMonth>().unwrap(), value);
		}
	}

	#[test]
	fn ord_without_offset_is_direct() {
		let a = XsdGYearMonth::new(2024, 2, None).unwrap();
		let b = XsdGYearMonth::new(2024, 3, None).unwrap();

		assert!(a < b);
	}

	// Two distinct `XsdGYearMonth` values are always at least 28 days apart,
	// far more than the `±14:00` offset uncertainty window (28 hours), so
	// they are always totally ordered, even with extreme offsets.
	#[test]
	fn ord_is_always_definite() {
		let a = XsdGYearMonth::new(2024, 2, None).unwrap();
		let b = XsdGYearMonth::new(2024, 3, Some(time::UtcOffset::from_hms(14, 0, 0).unwrap()))
			.unwrap();

		assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
		assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less));
	}

	#[test]
	fn eq_ignores_representation() {
		let a = XsdGYearMonth::new(2024, 2, None).unwrap();
		let b = XsdGYearMonth::new(2024, 2, None).unwrap();

		assert_eq!(a, b);
	}

	#[test]
	fn from_str_roundtrip() {
		let ym: XsdGYearMonth = "2024-02+05:00".parse().unwrap();
		assert_eq!(ym.year(), 2024);
		assert_eq!(ym.month(), 2);
		assert_eq!(
			ym.offset(),
			Some(time::UtcOffset::from_hms(5, 0, 0).unwrap())
		);
		assert_eq!(ym.to_string(), "2024-02+05:00");
	}

	#[test]
	fn new_rejects_out_of_range_offset() {
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();
		assert!(XsdGYearMonth::new(2024, 2, Some(offset)).is_none());
	}

	#[test]
	fn into_parts_roundtrip() {
		let offset = time::UtcOffset::from_hms(5, 0, 0).unwrap();
		let ym = XsdGYearMonth::new(2024, 2, Some(offset)).unwrap();

		assert_eq!(ym.into_parts(), (2024, 2, Some(offset)));
	}
}
