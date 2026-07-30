use core::fmt;
use std::{borrow::Borrow, ops::Deref, str::FromStr};

use crate::{lexical::Lexical, ParseXsd};

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
pub struct Token(str);

impl Token {
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

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl ToOwned for Token {
	type Owned = TokenBuf;

	fn to_owned(&self) -> Self::Owned {
		TokenBuf(self.0.to_owned())
	}
}

/// Owned token value.
///
/// Owned variant of [`Token`].
/// See: <https://www.w3.org/TR/xmlschema11-2/#token>.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenBuf(String);

impl TokenBuf {
	/// Parses and validates the given string as a token.
	pub fn new(value: String) -> Result<Self, InvalidToken> {
		if Token::validate(&value) {
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

	/// Borrows this owned token as a [`Token`].
	pub fn as_token(&self) -> &Token {
		unsafe { Token::new_unchecked(self.0.as_str()) }
	}

	/// Converts this token into a plain [`String`].
	pub fn into_string(self) -> String {
		self.0
	}
}

impl fmt::Display for TokenBuf {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl Borrow<Token> for TokenBuf {
	fn borrow(&self) -> &Token {
		self.as_token()
	}
}

impl Deref for TokenBuf {
	type Target = Token;

	fn deref(&self) -> &Self::Target {
		self.as_token()
	}
}

impl FromStr for TokenBuf {
	type Err = InvalidToken;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// Not `try_as_value()`: that goes through `FromStr` too, which would
		// recurse back here.
		let lexical_value = Token::parse(s)?;
		Ok(lexical_value.to_owned())
	}
}

impl ParseXsd for TokenBuf {
	type LexicalForm = crate::lexical::Token;
}

#[cfg(feature = "serde")]
impl serde::Serialize for TokenBuf {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0.serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TokenBuf {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = TokenBuf;

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
