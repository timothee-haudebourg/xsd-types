use crate::{
	format_timezone, is_valid_offset, lexical::InvalidXsdDateStr, seven_property_model_eq,
	seven_property_model_partial_cmp, Datatype, DisplayYear, XsdFromStr, XsdTyped, XsdValue,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

/// Error raised when the components of a [`XsdDate`] do not form a valid date
/// (e.g. an out-of-range day for the given month, or a timezone offset
/// outside the range permitted by XSD).
#[derive(Debug, thiserror::Error)]
#[error("invalid date value")]
pub struct InvalidDateValue;

/// A date, optionally with a timezone offset.
///
/// This is the value space of the XSD `date` datatype: a decoded `time::Date`
/// paired with an optional XSD timezone offset, as opposed to its lexical
/// (string) representation, [`lexical::XsdDateStr`](crate::lexical::XsdDateStr).
/// Equality and ordering follow XSD's partial order over timezone offsets: an
/// absent offset stands for the full `-14:00..=+14:00` range, so two dates
/// can compare as neither equal nor ordered.
/// See: <https://www.w3.org/TR/xmlschema11-2/#date>.
#[derive(Debug, Clone, Copy)]
pub struct XsdDate {
	date: time::Date,
	offset: Option<time::UtcOffset>,
}

impl XsdDate {
	/// Creates a new `XsdDate`, or returns `None` if `offset` is outside the
	/// `-14:00..=+14:00` range permitted by XSD.
	pub fn new(date: time::Date, offset: Option<time::UtcOffset>) -> Option<Self> {
		if offset.is_none_or(is_valid_offset) {
			Some(Self { date, offset })
		} else {
			None
		}
	}

	/// Returns the date, without its timezone offset.
	pub fn date(&self) -> time::Date {
		self.date
	}

	/// Returns the timezone offset, if any.
	pub fn offset(&self) -> Option<time::UtcOffset> {
		self.offset
	}

	/// Deconstructs this `XsdDate` into its date and timezone offset.
	pub fn into_parts(self) -> (time::Date, Option<time::UtcOffset>) {
		(self.date, self.offset)
	}

	fn effective_date_time(&self) -> time::PrimitiveDateTime {
		time::PrimitiveDateTime::new(self.date, time::Time::MIDNIGHT)
	}
}

impl PartialEq for XsdDate {
	fn eq(&self, other: &Self) -> bool {
		seven_property_model_eq(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

impl Eq for XsdDate {}

impl Hash for XsdDate {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.date.hash(state);
		self.offset.hash(state)
	}
}

impl PartialOrd for XsdDate {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		seven_property_model_partial_cmp(
			self.effective_date_time(),
			self.offset,
			other.effective_date_time(),
			other.offset,
		)
	}
}

/// Error raised when parsing a [`XsdDate`] from a string, either because the
/// input is not a syntactically valid `date` lexical representation, or
/// because it does not denote a valid date value.
#[derive(Debug, thiserror::Error)]
pub enum DateFromStrError {
	#[error("invalid date syntax")]
	Syntax(#[from] InvalidXsdDateStr<String>),

	#[error(transparent)]
	Value(#[from] InvalidDateValue),
}

impl FromStr for XsdDate {
	type Err = DateFromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value =
			crate::lexical::XsdDateStr::new(s).map_err(|_| InvalidXsdDateStr(s.to_owned()))?;
		Self::try_from(lexical_value).map_err(Into::into)
	}
}

impl XsdTyped for XsdDate {
	fn datatype(&self) -> Datatype {
		Datatype::Date
	}
}

impl XsdValue for XsdDate {
	type LexicalForm = crate::lexical::XsdDateStr;

	fn to_lexical_form(&self) -> crate::lexical::XsdDateString {
		// `Display` emits the canonical lexical form by contract, so it is
		// always valid and this cannot panic.
		crate::lexical::XsdDateString::new(self.to_string())
			.expect("Display emits a valid date lexical form")
	}
}

impl<'a> XsdFromStr<'a> for XsdDate {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<crate::lexical::XsdDateStr, Self>(lexical_value)
	}
}

impl fmt::Display for XsdDate {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}-{:02}-{:02}",
			DisplayYear(self.date.year()),
			u8::from(self.date.month()),
			self.date.day()
		)?;

		format_timezone(self.offset, f)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdDate {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdDate {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdDate;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#date")
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
	use super::XsdDate;
	use crate::XsdValue;

	#[test]
	fn to_lexical_form_roundtrip() {
		for s in [
			"2024-02-29",
			"-0044-03-15Z",
			"2000-01-01+05:30",
			"0001-12-31-14:00",
		] {
			let value: XsdDate = s.parse().unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), s);
			assert_eq!(lexical.as_str().parse::<XsdDate>().unwrap(), value);
		}
	}

	#[test]
	fn new_rejects_out_of_range_offset() {
		let date = time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap();
		let offset = time::UtcOffset::from_hms(15, 0, 0).unwrap();

		assert!(XsdDate::new(date, Some(offset)).is_none());
	}

	#[test]
	fn into_parts_roundtrip() {
		let date = time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap();
		let offset = time::UtcOffset::from_hms(2, 0, 0).unwrap();
		let d = XsdDate::new(date, Some(offset)).unwrap();

		assert_eq!(d.into_parts(), (date, Some(offset)));
	}

	#[test]
	fn ord_without_offset_is_direct() {
		let a: XsdDate = "2024-01-01".parse().unwrap();
		let b: XsdDate = "2024-01-02".parse().unwrap();

		assert!(a < b);
	}

	#[test]
	fn ord_with_one_offset_can_be_incomparable() {
		// The imputed `+14:00`/`-14:00` bounds for `2024-01-01` (no offset)
		// span `[2023-12-31T10:00Z, 2024-01-01T14:00Z]`, which overlaps the
		// single instant `2024-01-02T00:00:00+14:00` resolves to
		// (`2024-01-01T10:00Z`).
		let a: XsdDate = "2024-01-01".parse().unwrap();
		let b: XsdDate = "2024-01-02+14:00".parse().unwrap();

		assert_eq!(a.partial_cmp(&b), None);
	}

	#[test]
	fn eq_ignores_representation() {
		let a: XsdDate = "2024-01-01+02:00".parse().unwrap();
		let b: XsdDate = "2024-01-01+02:00".parse().unwrap();
		let c: XsdDate = "2024-01-01+03:00".parse().unwrap();

		assert_eq!(a, b);
		assert_ne!(a, c);
	}
}
