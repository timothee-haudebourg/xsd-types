use crate::lexical::{lexical_form, Decimal, DecimalBuf, Integer, IntegerBuf, NonNegativeInteger};

use super::{NonNegativeIntegerBuf, Overflow};
use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

lexical_form! {
	/// Positive integer number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#positiveInteger>
	ty: PositiveInteger,

	/// Owned positive integer number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#positiveInteger>
	buffer: PositiveIntegerBuf,

	/// Creates a new positive integer from a string.
	///
	/// If the input string is ot a [valid XSD positive integer](https://www.w3.org/TR/xmlschema-2/#positiveInteger),
	/// an [`InvalidPositiveInteger`] error is returned.
	new,

	/// Creates a new positive integer from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD positive integer](https://www.w3.org/TR/xmlschema-2/#positiveInteger).
	new_unchecked,

	value: crate::PositiveInteger,
	error: InvalidPositiveInteger,
	as_ref: as_positive_integer,
	parent_forms: {
		as_non_negative_integer: NonNegativeInteger, NonNegativeIntegerBuf,
		as_integer: Integer, IntegerBuf,
		as_decimal: Decimal, DecimalBuf
	}
}

impl PositiveInteger {
	/// Returns the canonical form of `self` (without leading zeros).
	pub fn canonical(&self) -> &Self {
		let mut last_zero = 0;
		for (i, c) in self.0.iter().enumerate() {
			match c {
				b'+' => (),
				b'0' => last_zero = i,
				_ => return unsafe { Self::new_unchecked(&self.0[i..]) },
			}
		}

		unsafe { Self::new_unchecked(&self.0[last_zero..]) }
	}

	fn as_canonical_str(&self) -> &str {
		self.canonical().as_str()
	}

	#[inline(always)]
	pub fn value(&self) -> crate::PositiveInteger {
		use num_bigint::BigInt;
		unsafe { crate::PositiveInteger::new_unchecked(BigInt::from_str(self.as_str()).unwrap()) }
	}
}

impl PartialEq for PositiveInteger {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for PositiveInteger {}

impl Hash for PositiveInteger {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for PositiveInteger {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_canonical_str().cmp(other.as_canonical_str())
	}
}

impl PartialOrd for PositiveInteger {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PositiveIntegerBuf {
	pub fn one() -> Self {
		unsafe { Self::new_unchecked("1".to_string()) }
	}
}

impl PartialOrd for PositiveIntegerBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PositiveIntegerBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for PositiveIntegerBuf {
				fn from(i: $ty) -> Self {
					unsafe { PositiveIntegerBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a PositiveInteger> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a PositiveInteger) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<PositiveIntegerBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: PositiveIntegerBuf) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}
		)*
	};
}

number_conversion! {
	u8,
	i8,
	u16,
	i16,
	u32,
	i32,
	u64,
	i64,
	u128,
	i128,
	usize,
	isize
}

fn check_bytes(s: &[u8]) -> bool {
	check(s.iter().copied())
}

fn check<C: Iterator<Item = u8>>(mut chars: C) -> bool {
	enum State {
		Initial,
		NonEmptyInteger,
		Integer,
	}

	let mut state = State::Initial;

	loop {
		state = match state {
			State::Initial => match chars.next() {
				Some(b'+' | b'0') => State::NonEmptyInteger,
				Some(b'1'..=b'9') => State::Integer,
				_ => break false,
			},
			State::NonEmptyInteger => match chars.next() {
				Some(b'0') => State::NonEmptyInteger,
				Some(b'1'..=b'9') => State::Integer,
				_ => break false,
			},
			State::Integer => match chars.next() {
				Some(b'0'..=b'9') => State::Integer,
				Some(_) => break false,
				None => break true,
			},
		}
	}
}
