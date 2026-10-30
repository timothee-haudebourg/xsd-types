use crate::lexical::lexical_form;
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use str_newtype::StrNewType;

use crate::lexical::XsdLexicalForm;
use crate::XsdError;

/// XsdBooleanStr.
///
/// This is the `booleanRep` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-booleanRep>.
///
/// ```abnf
/// booleanRep = %s"true" / %s"false" / "0" / "1"
/// ```
#[derive(Validate, StrNewType)]
#[automaton(super::grammar::Boolean)]
#[newtype(owned(XsdBooleanString, derive(PartialEq, Eq)))]
pub struct XsdBooleanStr(str);

impl XsdLexicalForm for XsdBooleanStr {
	type Value = crate::XsdBoolean;
}

impl<'a> crate::XsdFromStr<'a> for &'a XsdBooleanStr {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		XsdBooleanStr::new(value).map_err(|_| XsdError::lexical_form(InvalidXsdBooleanStr(value)))
	}
}

lexical_form! {
	ty: XsdBooleanStr,
	buffer: XsdBooleanString,
	value: crate::XsdBoolean,
	error: InvalidXsdBooleanStr,
	parent_forms: {}
}

impl XsdBooleanStr {
	pub fn value(&self) -> crate::XsdBoolean {
		crate::XsdBoolean(matches!(self.as_str(), "true" | "1"))
	}
}

impl PartialEq for XsdBooleanStr {
	fn eq(&self, other: &Self) -> bool {
		self.value() == other.value()
	}
}

impl Eq for XsdBooleanStr {}

impl Hash for XsdBooleanStr {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value().hash(state)
	}
}

impl Ord for XsdBooleanStr {
	fn cmp(&self, other: &Self) -> Ordering {
		self.value().cmp(&other.value())
	}
}

impl PartialOrd for XsdBooleanStr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl From<bool> for XsdBooleanString {
	fn from(b: bool) -> Self {
		if b {
			unsafe { XsdBooleanString::new_unchecked("true".to_string()) }
		} else {
			unsafe { XsdBooleanString::new_unchecked("false".to_string()) }
		}
	}
}

impl<'a> From<&'a XsdBooleanStr> for crate::XsdBoolean {
	fn from(b: &'a XsdBooleanStr) -> crate::XsdBoolean {
		b.value()
	}
}

impl From<XsdBooleanString> for crate::XsdBoolean {
	fn from(b: XsdBooleanString) -> crate::XsdBoolean {
		b.value()
	}
}

impl<'a> From<&'a XsdBooleanStr> for bool {
	fn from(b: &'a XsdBooleanStr) -> bool {
		b.value().into()
	}
}

impl From<XsdBooleanString> for bool {
	fn from(b: XsdBooleanString) -> bool {
		b.value().into()
	}
}
