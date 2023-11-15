use core::fmt;
use std::{borrow::Borrow, ops::Deref, str::FromStr};

use crate::ParseRdf;

#[derive(Debug, thiserror::Error)]
#[error("invalid token `{0}`")]
pub struct InvalidToken<T = String>(pub T);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Token(str);

impl Token {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenBuf(String);

impl TokenBuf {
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

	pub fn as_token(&self) -> &Token {
		unsafe { Token::new_unchecked(self.0.as_str()) }
	}

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
		Self::new(s.to_owned())
	}
}

impl ParseRdf for TokenBuf {
	type LexicalForm = crate::lexical::Token;
}
