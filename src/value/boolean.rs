use core::{fmt, str::FromStr};

use crate::{
	lexical::{self, Lexical, LexicalFormOf},
	Datatype, ParseXsd, XsdValue,
};

/// Boolean value.
///
/// This is the value-space representation of the XSD 1.1 `boolean` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#boolean>.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Boolean(pub bool);

impl From<bool> for Boolean {
	fn from(value: bool) -> Self {
		Self(value)
	}
}

impl From<Boolean> for bool {
	fn from(value: Boolean) -> Self {
		value.0
	}
}

impl XsdValue for Boolean {
	fn datatype(&self) -> Datatype {
		Datatype::Boolean
	}
}

impl LexicalFormOf<Boolean> for lexical::Boolean {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<Boolean, Self::ValueError> {
		Ok(self.value())
	}
}

impl ParseXsd for Boolean {
	type LexicalForm = lexical::Boolean;
}

impl FromStr for Boolean {
	type Err = lexical::InvalidBoolean<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::Boolean::parse(s)?;
		Ok(l.as_value())
	}
}

impl fmt::Display for Boolean {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.0 {
			write!(f, "true")
		} else {
			write!(f, "false")
		}
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for Boolean {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Boolean {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = Boolean;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a http://www.w3.org/2001/XMLSchema#boolean")
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
