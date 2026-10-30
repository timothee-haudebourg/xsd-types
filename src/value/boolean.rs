use core::{fmt, str::FromStr};

use crate::{lexical, Datatype, XsdFromStr, XsdTyped, XsdValue};

/// Boolean value.
///
/// This is the value-space representation of the XSD 1.1 `boolean` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#boolean>.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XsdBoolean(pub bool);

impl From<bool> for XsdBoolean {
	fn from(value: bool) -> Self {
		Self(value)
	}
}

impl From<XsdBoolean> for bool {
	fn from(value: XsdBoolean) -> Self {
		value.0
	}
}

impl XsdTyped for XsdBoolean {
	fn datatype(&self) -> Datatype {
		Datatype::Boolean
	}
}

impl XsdValue for XsdBoolean {
	type LexicalForm = lexical::XsdBooleanStr;

	fn to_lexical_form(&self) -> lexical::XsdBooleanString {
		lexical::XsdBooleanString::from(self.0)
	}
}

impl<'a> XsdFromStr<'a> for XsdBoolean {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<lexical::XsdBooleanStr, Self>(lexical_value)
	}
}

impl FromStr for XsdBoolean {
	type Err = lexical::InvalidXsdBooleanStr<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::XsdBooleanStr::new(s)
			.map_err(|_| lexical::InvalidXsdBooleanStr(s.to_owned()))?;
		Ok(l.into())
	}
}

impl fmt::Display for XsdBoolean {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.0 {
			write!(f, "true")
		} else {
			write!(f, "false")
		}
	}
}

#[cfg(test)]
mod tests {
	use super::XsdBoolean;
	use crate::XsdValue;

	#[test]
	fn to_lexical_form_roundtrip() {
		for s in ["true", "false"] {
			let value: XsdBoolean = s.parse().unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), s);
			assert_eq!(lexical.as_str().parse::<XsdBoolean>().unwrap(), value);
		}
	}

	#[test]
	fn to_lexical_form_is_canonical() {
		// `1` and `0` are valid lexical forms, but the canonical ones are
		// `true` and `false`.
		let value: XsdBoolean = "1".parse().unwrap();
		assert_eq!(value.to_lexical_form().as_str(), "true");

		let value: XsdBoolean = "0".parse().unwrap();
		assert_eq!(value.to_lexical_form().as_str(), "false");
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XsdBoolean {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XsdBoolean {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = XsdBoolean;

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
