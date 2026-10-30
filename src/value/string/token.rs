use core::fmt;
use std::{borrow::Borrow, ops::Deref, str::FromStr};

/// Error raised when a string is not a valid `token` value.
#[derive(Debug, thiserror::Error)]
#[error("invalid token `{0}`")]
pub struct InvalidToken<T = String>(pub T);

/// Token value.
///
/// Value space representation of the XSD `token` datatype: a [`str`]
/// guaranteed to contain no tab, line feed, or carriage return character,
/// no leading or trailing space, and no sequence of two or more
/// consecutive spaces.
/// See: <https://www.w3.org/TR/xmlschema11-2/#token>.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct XsdToken(str);

impl XsdToken {
	/// Parses and validates the given string as a token.
	pub fn new(value: &str) -> Result<&Self, InvalidToken<&str>> {
		if Self::validate(value) {
			Ok(unsafe { Self::new_unchecked(value) })
		} else {
			Err(InvalidToken(value))
		}
	}

	fn validate(value: &str) -> bool {
		let mut leading = true;
		let mut space = false;

		for c in value.chars() {
			if c == ' ' {
				if space {
					return false;
				}

				space = true
			} else {
				space = false
			}

			if matches!(c, '\t' | '\n' | '\r') || (space && leading) {
				return false;
			}

			leading = false;
		}

		!space
	}

	/// Creates a new token string from the input `value` without validation.
	///
	/// # Safety
	///
	/// The input `value` must be an XSD token string.
	pub unsafe fn new_unchecked(value: &str) -> &Self {
		std::mem::transmute(value)
	}

	/// Returns this token as a plain [`str`].
	pub fn as_str(&self) -> &str {
		&self.0
	}
}

impl Borrow<str> for XsdToken {
	fn borrow(&self) -> &str {
		&self.0
	}
}

impl fmt::Display for XsdToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl ToOwned for XsdToken {
	type Owned = XsdTokenBuf;

	fn to_owned(&self) -> Self::Owned {
		XsdTokenBuf(self.0.to_owned())
	}
}

/// Owned token value.
///
/// Owned variant of [`XsdToken`].
/// See: <https://www.w3.org/TR/xmlschema11-2/#token>.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XsdTokenBuf(String);

impl XsdTokenBuf {
	/// Parses and validates the given string as a token.
	pub fn new(value: String) -> Result<Self, InvalidToken> {
		if XsdToken::validate(&value) {
			Ok(Self(value))
		} else {
			Err(InvalidToken(value))
		}
	}

	/// Creates a new token string from the input `value` without validation.
	///
	/// # Safety
	///
	/// The input `value` must be an XSD token string.
	pub unsafe fn new_unchecked(value: String) -> Self {
		Self(value)
	}

	/// Borrows this owned token as a [`XsdToken`].
	pub fn as_token(&self) -> &XsdToken {
		unsafe { XsdToken::new_unchecked(self.0.as_str()) }
	}

	/// Converts this token into a plain [`String`].
	pub fn into_string(self) -> String {
		self.0
	}
}

impl fmt::Display for XsdTokenBuf {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl Borrow<XsdToken> for XsdTokenBuf {
	fn borrow(&self) -> &XsdToken {
		self.as_token()
	}
}

impl Deref for XsdTokenBuf {
	type Target = XsdToken;

	fn deref(&self) -> &Self::Target {
		self.as_token()
	}
}

impl FromStr for XsdTokenBuf {
	type Err = InvalidToken;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lexical_value = XsdToken::new(s).map_err(|_| InvalidToken(s.to_owned()))?;
		Ok(lexical_value.to_owned())
	}
}

impl TryFrom<String> for XsdTokenBuf {
	type Error = InvalidToken<String>;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::new(value)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdTokenBuf {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0.serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdTokenBuf {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdTokenBuf;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#token")
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

impl<'a> crate::XsdFromStr<'a> for XsdTokenBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<XsdToken, Self>(lexical_value)
	}
}

impl crate::XsdTyped for XsdTokenBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::TokenDatatype::Token.into()
	}
}

impl crate::XsdValue for XsdTokenBuf {
	type LexicalForm = XsdToken;

	fn to_lexical_form(&self) -> XsdTokenBuf {
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	use super::XsdTokenBuf;

	#[test]
	fn lexical_form_round_trip() {
		let input = "a token string";
		let value = XsdTokenBuf::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical.as_str(), input);
		assert_eq!(XsdTokenBuf::xsd_from_str(lexical.as_str()).unwrap(), value);
	}
}
