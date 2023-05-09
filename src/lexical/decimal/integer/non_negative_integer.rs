use crate::lexical::lexical_form;

use super::{Decimal, DecimalBuf, Integer, IntegerBuf, Overflow, Sign};
use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

mod positive_integer;

pub use positive_integer::*;

lexical_form! {
	/// Non negative integer number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger>
	ty: NonNegativeInteger,

	/// Owned non negative integer number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger>
	buffer: NonNegativeIntegerBuf,

	/// Creates a new non negative integer from a string.
	///
	/// If the input string is ot a [valid XSD non negative integer](https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger),
	/// an [`InvalidNonNegativeInteger`] error is returned.
	new,

	/// Creates a new non negative integer from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD non negative integer](https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger).
	new_unchecked,

	value: crate::NonNegativeInteger,
	error: InvalidNonNegativeInteger,
	as_ref: as_non_negative_integer,
	parent_forms: {
		as_integer: Integer, IntegerBuf,
		as_decimal: Decimal, DecimalBuf
	}
}

impl NonNegativeInteger {
	/// Returns `true` if `self` is positive
	/// and `false` is the number is zero.
	pub fn is_positive(&self) -> bool {
		for c in &self.0 {
			match c {
				b'+' | b'0' => (),
				_ => return true,
			}
		}

		false
	}

	/// Returns `true` if `self` is zero
	/// and `false` otherwise.
	pub fn is_zero(&self) -> bool {
		for c in &self.0 {
			if !matches!(c, b'+' | b'0') {
				return false;
			}
		}

		true
	}

	pub fn sign(&self) -> Sign {
		for c in &self.0 {
			match c {
				b'+' | b'0' => (),
				_ => return Sign::Positive,
			}
		}

		Sign::Zero
	}

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

	#[inline(always)]
	fn as_canonical_str(&self) -> &str {
		self.canonical().as_str()
	}

	#[inline(always)]
	pub fn value(&self) -> crate::NonNegativeInteger {
		use num_bigint::BigInt;
		unsafe {
			crate::NonNegativeInteger::new_unchecked(BigInt::from_str(self.as_str()).unwrap())
		}
	}
}

impl PartialEq for NonNegativeInteger {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for NonNegativeInteger {}

impl Hash for NonNegativeInteger {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for NonNegativeInteger {
	fn cmp(&self, other: &Self) -> Ordering {
		let sign = self.sign();
		let other_sign = other.sign();
		match sign.cmp(&other_sign) {
			Ordering::Equal => {
				let a = &self.canonical().0;
				let b = &other.canonical().0;

				match a.len().cmp(&b.len()) {
					Ordering::Equal => {
						if sign.is_negative() {
							a.cmp(b).reverse()
						} else {
							a.cmp(b)
						}
					}
					other => {
						if sign.is_negative() {
							other.reverse()
						} else {
							other
						}
					}
				}
			}
			other => other,
		}
	}
}

impl PartialOrd for NonNegativeInteger {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl NonNegativeIntegerBuf {
	pub fn zero() -> Self {
		unsafe { Self::new_unchecked("0".to_string()) }
	}

	pub fn one() -> Self {
		unsafe { Self::new_unchecked("1".to_string()) }
	}
}

impl Default for NonNegativeIntegerBuf {
	fn default() -> Self {
		Self::zero()
	}
}

impl PartialOrd for NonNegativeIntegerBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for NonNegativeIntegerBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for NonNegativeIntegerBuf {
				fn from(i: $ty) -> Self {
					unsafe { NonNegativeIntegerBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a NonNegativeInteger> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a NonNegativeInteger) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<NonNegativeIntegerBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: NonNegativeIntegerBuf) -> Result<Self, Overflow> {
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
		Zero,
	}

	let mut state = State::Initial;

	loop {
		state = match state {
			State::Initial => match chars.next() {
				Some(b'+') => State::NonEmptyInteger,
				Some(b'-') => State::Zero,
				Some(b'0'..=b'9') => State::Integer,
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
			State::Zero => match chars.next() {
				Some(b'0') => State::Zero,
				_ => break false,
			},
		}
	}
}
