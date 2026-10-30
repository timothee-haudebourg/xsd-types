use crate::{
	XsdDatatype, XsdDurationDatatype, XsdFromStr, XsdTyped, XsdValueDomain,
	lexical::InvalidXsdYearMonthDurationStr,
};
use core::fmt;
use std::{cmp::Ordering, hash::Hash, str::FromStr};

/// Year month duration value.
///
/// Value space representation of the `yearMonthDuration` datatype, a
/// restriction of `duration` where the number of seconds is always zero.
/// See: <https://www.w3.org/TR/xmlschema11-2/#yearMonthDuration>.
#[derive(Debug, Clone, Copy)]
pub struct XsdYearMonthDuration {
	is_negative: bool,
	months: u32,
}

impl XsdYearMonthDuration {
	/// Creates a new year month duration from its sign and number of months.
	pub fn new(is_negative: bool, months: u32) -> Self {
		Self {
			is_negative,
			months,
		}
	}

	/// Converts this duration into its canonical lexical string
	/// representation.
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
}

impl PartialEq for XsdYearMonthDuration {
	fn eq(&self, other: &Self) -> bool {
		self.signed_months() == other.signed_months()
	}
}

impl Eq for XsdYearMonthDuration {}

impl Hash for XsdYearMonthDuration {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.signed_months().hash(state);
	}
}

/// Since `seconds` is always zero, `yearMonthDuration` is totally ordered by
/// its number of months: <https://www.w3.org/TR/xmlschema11-2/#yearMonthDuration>.
impl PartialOrd for XsdYearMonthDuration {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for XsdYearMonthDuration {
	fn cmp(&self, other: &Self) -> Ordering {
		self.signed_months().cmp(&other.signed_months())
	}
}

impl XsdTyped for XsdYearMonthDuration {
	fn datatype(&self) -> XsdDatatype {
		XsdDatatype::Duration(XsdDurationDatatype::YearMonthDuration)
	}
}

impl XsdValueDomain for XsdYearMonthDuration {
	type LexicalForm = crate::lexical::XsdYearMonthDurationStr;

	fn to_lexical_form(&self) -> crate::lexical::XsdYearMonthDurationString {
		// `Display` emits the canonical lexical form by contract, so it is
		// always valid and this cannot panic.
		crate::lexical::XsdYearMonthDurationString::new(self.to_string())
			.expect("Display emits a valid yearMonthDuration lexical form")
	}
}

impl<'a> XsdFromStr<'a> for XsdYearMonthDuration {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<crate::lexical::XsdYearMonthDurationStr, Self>(lexical_value)
	}
}

impl FromStr for XsdYearMonthDuration {
	type Err = InvalidXsdYearMonthDurationStr<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = crate::lexical::XsdYearMonthDurationStr::new(s)
			.map_err(|_| InvalidXsdYearMonthDurationStr(s.to_owned()))?;
		Ok(lexical_value.into())
	}
}

impl fmt::Display for XsdYearMonthDuration {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.months == 0 {
			// The canonical form of the zero duration has no sign.
			return f.write_str("P0M");
		}

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

#[cfg(feature = "serde")]
impl serde::Serialize for XsdYearMonthDuration {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdYearMonthDuration {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdYearMonthDuration;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#yearMonthDuration")
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

	fn assert_lexical_form_round_trip(input: &str) {
		let value = <XsdYearMonthDuration as XsdFromStr>::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert_eq!(
			XsdYearMonthDuration::from_str(lexical.as_str()).unwrap(),
			value
		);
	}

	#[test]
	fn to_lexical_form_round_trips() {
		for input in ["P1Y2M", "-P1Y2M", "P1Y", "P11M", "-P5Y"] {
			assert_lexical_form_round_trip(input);
		}
	}

	#[test]
	fn to_lexical_form_of_zero_is_p0m() {
		for input in ["P0M", "-P0M", "P0Y"] {
			let value = XsdYearMonthDuration::from_str(input).unwrap();
			assert_eq!(value.to_lexical_form().as_str(), "P0M");
			assert_eq!(value.to_string(), "P0M");
		}
	}
}
