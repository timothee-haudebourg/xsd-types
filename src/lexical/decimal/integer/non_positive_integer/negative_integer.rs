use crate::lexical::{
	lexical_form, Decimal, DecimalBuf, Integer, IntegerBuf, NonNegativeInteger,
	NonNegativeIntegerBuf,
};

use super::Overflow;
use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

lexical_form! {
	/// Negative integer number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#negativeInteger>
	ty: NegativeInteger,

	/// Owned negative integer number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#negativeInteger>
	buffer: NegativeIntegerBuf,

	/// Creates a new negative integer from a string.
	///
	/// If the input string is ot a [valid XSD negative integer](https://www.w3.org/TR/xmlschema-2/#negativeInteger),
	/// an [`InvalidNegativeInteger`] error is returned.
	new,

	/// Creates a new positive integer from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD negative integer](https://www.w3.org/TR/xmlschema-2/#negativeInteger).
	new_unchecked,

	value: crate::NegativeInteger,
	error: InvalidNegativeInteger,
	as_ref: as_negative_integer,
	parent_forms: {
		as_non_positive_integer: NonNegativeInteger, NonNegativeIntegerBuf,
		as_integer: Integer, IntegerBuf,
		as_decimal: Decimal, DecimalBuf
	}
}

impl NegativeInteger {
	/// Returns the canonical form of the absolute value of `self` (without leading zeros).
	pub fn abs(&self) -> &NonNegativeInteger {
		let mut last_zero = 0;
		for (i, c) in self.0.iter().enumerate() {
			match c {
				b'-' => (),
				b'0' => last_zero = i,
				_ => return unsafe { NonNegativeInteger::new_unchecked(&self.0[i..]) },
			}
		}

		unsafe { NonNegativeInteger::new_unchecked(&self.0[last_zero..]) }
	}

	fn as_canonical_str(&self) -> &str {
		self.abs().as_str()
	}

	#[inline(always)]
	pub fn value(&self) -> crate::NegativeInteger {
		use num_bigint::BigInt;
		unsafe { crate::NegativeInteger::new_unchecked(BigInt::from_str(self.as_str()).unwrap()) }
	}
}

impl PartialEq for NegativeInteger {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for NegativeInteger {}

impl Hash for NegativeInteger {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for NegativeInteger {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_canonical_str()
			.cmp(other.as_canonical_str())
			.reverse()
	}
}

impl PartialOrd for NegativeInteger {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl NegativeIntegerBuf {
	pub fn minus_one() -> Self {
		unsafe { Self::new_unchecked("-1".to_string()) }
	}
}

impl PartialOrd for NegativeIntegerBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for NegativeIntegerBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for NegativeIntegerBuf {
				fn from(i: $ty) -> Self {
					unsafe { NegativeIntegerBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a NegativeInteger> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a NegativeInteger) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<NegativeIntegerBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: NegativeIntegerBuf) -> Result<Self, Overflow> {
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
				Some(b'-') => State::NonEmptyInteger,
				_ => break false,
			},
			State::NonEmptyInteger => match chars.next() {
				Some(b'0'..=b'9') => State::Integer,
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
