use crate::lexical::lexical_form;

use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

lexical_form! {
	/// Base 64 string.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#base64Binary>
	ty: Base64Binary,

	/// Owned base 64 string.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#base64Binary>
	buffer: Base64BinaryBuf,

	/// Creates a new base 64 string from a string.
	///
	/// If the input string is ot a [valid XSD base 64 string](https://www.w3.org/TR/xmlschema-2/#base64Binary),
	/// an [`InvalidBase64Binary`] error is returned.
	new,

	/// Creates a new base 64 string from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD base 64 string](https://www.w3.org/TR/xmlschema-2/#base64Binary).
	new_unchecked,

	value: crate::Base64BinaryBuf,
	error: InvalidBase64Binary,
	as_ref: as_base_64_binary,
	parent_forms: {}
}

impl Base64Binary {
	#[inline(always)]
	fn as_canonical_str(&self) -> &str {
		self.as_str()
	}

	#[inline(always)]
	pub fn value(&self) -> crate::Base64BinaryBuf {
		crate::Base64BinaryBuf::decode(self.as_bytes()).unwrap()
	}
}

impl PartialEq for Base64Binary {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for Base64Binary {}

impl Hash for Base64Binary {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for Base64Binary {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_canonical_str().cmp(other.as_canonical_str())
	}
}

impl PartialOrd for Base64Binary {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Default for Base64BinaryBuf {
	fn default() -> Self {
		unsafe { Self::new_unchecked(Vec::new()) }
	}
}

impl PartialOrd for Base64BinaryBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Base64BinaryBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_base_64_binary().cmp(other.as_base_64_binary())
	}
}

fn check_bytes(s: &[u8]) -> bool {
	check(s.iter().copied())
}

fn check<C: Iterator<Item = u8>>(mut chars: C) -> bool {
	enum State {
		Data,
		Padding,
	}

	let mut state = State::Data;

	loop {
		state = match state {
			State::Data => match chars.next() {
				Some(b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'/') => State::Data,
				Some(b'=') => State::Padding,
				None => break true,
				_ => break false,
			},
			State::Padding => match chars.next() {
				Some(b'=') => State::Padding,
				None => break true,
				_ => break false,
			},
		}
	}
}
