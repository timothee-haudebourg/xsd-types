use crate::lexical::lexical_form;

use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

lexical_form! {
	/// Hexadecimal string.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#hexBinary>
	ty: HexBinary,

	/// Owned hexadecimal string.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#hexBinary>
	buffer: HexBinaryBuf,

	/// Creates a new hexadecimal string from a string.
	///
	/// If the input string is ot a [valid XSD hexadecimal string](https://www.w3.org/TR/xmlschema-2/#hexBinary),
	/// an [`InvalidHexBinary`] error is returned.
	new,

	/// Creates a new hexadecimal string from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD hexadecimal string](https://www.w3.org/TR/xmlschema-2/#hexBinary).
	new_unchecked,

	value: crate::HexBinaryBuf,
	error: InvalidHexBinary,
	as_ref: as_hex_binary,
	parent_forms: {}
}

impl HexBinary {
	#[inline(always)]
	fn as_canonical_str(&self) -> &str {
		self.as_str()
	}

	#[inline(always)]
	pub fn value(&self) -> crate::HexBinaryBuf {
		crate::HexBinaryBuf::decode(self.as_bytes()).unwrap()
	}
}

impl PartialEq for HexBinary {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for HexBinary {}

impl Hash for HexBinary {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for HexBinary {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_canonical_str().cmp(other.as_canonical_str())
	}
}

impl PartialOrd for HexBinary {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Default for HexBinaryBuf {
	fn default() -> Self {
		unsafe { Self::new_unchecked(Vec::new()) }
	}
}

impl PartialOrd for HexBinaryBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for HexBinaryBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_hex_binary().cmp(other.as_hex_binary())
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
