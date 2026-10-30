use core::fmt;
use std::{borrow::Borrow, ops::Deref, str::FromStr};

/// Error raised when a string is not a valid `normalizedString` value.
#[derive(Debug, thiserror::Error)]
#[error("invalid normalized string `{0}`")]
pub struct InvalidNormalizedStr<T = String>(pub T);

/// Normalized string value.
///
/// Value space representation of the XSD `normalizedString` datatype: a
/// [`str`] guaranteed to contain no tab, line feed, or carriage return
/// character.
/// See: <https://www.w3.org/TR/xmlschema11-2/#normalizedString>.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct XsdNormalizedStr(str);

impl XsdNormalizedStr {
	/// Parses and validates the given string as a normalized string.
	pub fn new(value: &str) -> Result<&Self, InvalidNormalizedStr<&str>> {
		if Self::validate(value) {
			Ok(unsafe { Self::new_unchecked(value) })
		} else {
			Err(InvalidNormalizedStr(value))
		}
	}

	fn validate(value: &str) -> bool {
		value.chars().all(|c| !matches!(c, '\t' | '\n' | '\r'))
	}

	/// Creates a new normalized string from the input `value` without
	/// validation.
	///
	/// # Safety
	///
	/// The input `value` must be an XSD normalized string.
	pub unsafe fn new_unchecked(value: &str) -> &Self {
		std::mem::transmute(value)
	}

	/// Returns this normalized string as a plain [`str`].
	pub fn as_str(&self) -> &str {
		&self.0
	}
}

impl Borrow<str> for XsdNormalizedStr {
	fn borrow(&self) -> &str {
		&self.0
	}
}

impl fmt::Display for XsdNormalizedStr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl ToOwned for XsdNormalizedStr {
	type Owned = XsdNormalizedString;

	fn to_owned(&self) -> Self::Owned {
		XsdNormalizedString(self.0.to_owned())
	}
}

/// Owned normalized string value.
///
/// Owned variant of [`XsdNormalizedStr`].
/// See: <https://www.w3.org/TR/xmlschema11-2/#normalizedString>.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XsdNormalizedString(String);

impl XsdNormalizedString {
	/// Parses and validates the given string as a normalized string.
	pub fn new(value: String) -> Result<Self, InvalidNormalizedStr> {
		if XsdNormalizedStr::validate(&value) {
			Ok(Self(value))
		} else {
			Err(InvalidNormalizedStr(value))
		}
	}

	/// Creates a new normalized string from the input `value` without
	/// validation.
	///
	/// # Safety
	///
	/// The input `value` must be an XSD normalized string.
	pub unsafe fn new_unchecked(value: String) -> Self {
		Self(value)
	}

	/// Borrows this normalized string as a [`XsdNormalizedStr`].
	pub fn as_normalized_str(&self) -> &XsdNormalizedStr {
		unsafe { XsdNormalizedStr::new_unchecked(self.0.as_str()) }
	}

	/// Converts this normalized string into a plain [`String`].
	pub fn into_string(self) -> String {
		self.0
	}
}

impl fmt::Display for XsdNormalizedString {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl Borrow<XsdNormalizedStr> for XsdNormalizedString {
	fn borrow(&self) -> &XsdNormalizedStr {
		self.as_normalized_str()
	}
}

impl Deref for XsdNormalizedString {
	type Target = XsdNormalizedStr;

	fn deref(&self) -> &Self::Target {
		self.as_normalized_str()
	}
}

impl FromStr for XsdNormalizedString {
	type Err = InvalidNormalizedStr<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value =
			XsdNormalizedStr::new(s).map_err(|_| InvalidNormalizedStr(s.to_owned()))?;
		Ok(lexical_value.to_owned())
	}
}

impl TryFrom<String> for XsdNormalizedString {
	type Error = InvalidNormalizedStr<String>;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::new(value)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdNormalizedString {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0.serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdNormalizedString {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdNormalizedString;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#normalizedString")
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

impl<'a> crate::XsdFromStr<'a> for XsdNormalizedString {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<crate::lexical::XsdNormalizedStr, Self>(lexical_value)
	}
}

impl crate::XsdTyped for XsdNormalizedString {
	fn datatype(&self) -> crate::Datatype {
		crate::NormalizedStringDatatype::NormalizedString.into()
	}
}

impl crate::XsdValue for XsdNormalizedString {
	type LexicalForm = crate::lexical::XsdNormalizedStr;

	fn to_lexical_form(&self) -> XsdNormalizedString {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	use super::XsdNormalizedString;

	#[test]
	fn lexical_form_round_trip() {
		let input = "a normalized  string";
		let value = XsdNormalizedString::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert_eq!(
			XsdNormalizedString::xsd_from_str(lexical.as_str()).unwrap(),
			value
		);
	}
}
