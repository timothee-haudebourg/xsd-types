use super::lexical_form;
use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

lexical_form! {
	/// Boolean.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#boolean>
	ty: Boolean,

	/// Owned boolean.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#boolean>
	buffer: BooleanBuf,

	/// Creates a new boolean from a string.
	///
	/// If the input string is ot a [valid XSD boolean](https://www.w3.org/TR/xmlschema-2/#boolean),
	/// an [`InvalidBoolean`] error is returned.
	new,

	/// Creates a new boolean from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD boolean](https://www.w3.org/TR/xmlschema-2/#boolean).
	new_unchecked,

	value: crate::Boolean,
	error: InvalidBoolean,
	as_ref: as_boolean,
	parent_forms: {}
}

impl Boolean {
	pub fn value(&self) -> bool {
		matches!(&self.0, b"true" | b"1")
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
			unsafe { BooleanBuf::new_unchecked(vec![b't', b'r', b'u', b'e']) }
		} else {
			unsafe { BooleanBuf::new_unchecked(vec![b'f', b'a', b'l', b's', b'e']) }
		}
	}
}

impl<'a> From<&'a Boolean> for bool {
	fn from(b: &'a Boolean) -> bool {
		b.value()
	}
}

impl From<BooleanBuf> for bool {
	fn from(b: BooleanBuf) -> bool {
		b.value()
	}
}

fn check_bytes(s: &[u8]) -> bool {
	matches!(s, b"true" | b"false" | b"0" | b"1")
}
