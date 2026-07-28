use crate::lexical::lexical_form;
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use str_newtype::StrNewType;

use crate::lexical::Lexical;

/// Boolean.
///
/// This is the `booleanRep` production of the XSD 1.1 Datatypes
/// specification: <https://www.w3.org/TR/xmlschema11-2/#nt-booleanRep>.
///
/// ```abnf
/// booleanRep = %s"true" / %s"false" / "0" / "1"
/// ```
#[derive(Validate, StrNewType)]
#[automaton(super::grammar::Boolean)]
#[newtype(owned(BooleanBuf, derive(PartialEq, Eq)))]
pub struct Boolean(str);

impl Lexical for Boolean {
	type Error = InvalidBoolean<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidBoolean(value.to_owned()))
	}
}

lexical_form! {
	ty: Boolean,
	buffer: BooleanBuf,
	value: crate::Boolean,
	error: InvalidBoolean,
	parent_forms: {}
}

impl Boolean {
	pub fn value(&self) -> crate::Boolean {
		crate::Boolean(matches!(self.as_str(), "true" | "1"))
	}
}

impl PartialEq for Boolean {
	fn eq(&self, other: &Self) -> bool {
		self.value() == other.value()
	}
}

impl Eq for Boolean {}

impl Hash for Boolean {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value().hash(state)
	}
}

impl Ord for Boolean {
	fn cmp(&self, other: &Self) -> Ordering {
		self.value().cmp(&other.value())
	}
}

impl PartialOrd for Boolean {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl From<bool> for BooleanBuf {
	fn from(b: bool) -> Self {
		if b {
			unsafe { BooleanBuf::new_unchecked("true".to_string()) }
		} else {
			unsafe { BooleanBuf::new_unchecked("false".to_string()) }
		}
	}
}

impl<'a> From<&'a Boolean> for crate::Boolean {
	fn from(b: &'a Boolean) -> crate::Boolean {
		b.value()
	}
}

impl From<BooleanBuf> for crate::Boolean {
	fn from(b: BooleanBuf) -> crate::Boolean {
		b.value()
	}
}

impl<'a> From<&'a Boolean> for bool {
	fn from(b: &'a Boolean) -> bool {
		b.value().into()
	}
}

impl From<BooleanBuf> for bool {
	fn from(b: BooleanBuf) -> bool {
		b.value().into()
	}
}
