use crate::lexical::lexical_form;

use super::{Decimal, DecimalBuf, Integer, IntegerBuf, NonNegativeInteger, Overflow, Sign};
use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

mod negative_integer;

pub use negative_integer::*;

lexical_form! {
	/// Non positive integer number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#nonPositiveInteger>
	ty: NonPositiveInteger,

	/// Owned non positive integer number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#nonPositiveInteger>
	buffer: NonPositiveIntegerBuf,

	/// Creates a new non positive integer from a string.
	///
	/// If the input string is ot a [valid XSD non positive integer](https://www.w3.org/TR/xmlschema-2/#nonPositiveInteger),
	/// an [`InvalidNonPositiveInteger`] error is returned.
	new,

	/// Creates a new non positive integer from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD non positive integer](https://www.w3.org/TR/xmlschema-2/#nonPositiveInteger).
	new_unchecked,

	value: crate::NonPositiveInteger,
	error: InvalidNonPositiveInteger,
	as_ref: as_non_positive_integer,
	parent_forms: {
		as_integer: Integer, IntegerBuf,
		as_decimal: Decimal, DecimalBuf
	}
}

impl NonPositiveInteger {
	/// Returns `true` if `self` is negative
	/// and `false` is the number is zero.
	pub fn is_negative(&self) -> bool {
		for c in &self.0 {
			match c {
				b'-' | b'0' => (),
				_ => return true,
			}
		}

		false
	}

	/// Returns `true` if `self` is zero
	/// and `false` otherwise.
	pub fn is_zero(&self) -> bool {
		for c in &self.0 {
			if !matches!(c, b'-' | b'0') {
				return false;
			}
		}

		true
	}

	pub fn sign(&self) -> Sign {
		for c in &self.0 {
			match c {
				b'-' | b'0' => (),
				_ => return Sign::Negative,
			}
		}

		Sign::Zero
	}

	/// Returns the absolute value of `self`.
	///
	/// The returned integer is in canonical form (without leading zeros).
	pub fn abs(&self) -> &NonNegativeInteger {
		let mut last_zero = 0;
		for (i, c) in self.0.iter().enumerate() {
			match c {
				b'+' | b'-' => (),
				b'0' => last_zero = i,
				_ => return unsafe { NonNegativeInteger::new_unchecked(&self.0[i..]) },
			}
		}

		unsafe { NonNegativeInteger::new_unchecked(&self.0[last_zero..]) }
	}

	#[inline(always)]
	pub fn value(&self) -> crate::NonPositiveInteger {
		use num_bigint::BigInt;
		unsafe {
			crate::NonPositiveInteger::new_unchecked(BigInt::from_str(self.as_str()).unwrap())
		}
	}
}

impl PartialEq for NonPositiveInteger {
	fn eq(&self, other: &Self) -> bool {
		self.sign() == other.sign() && self.abs() == other.abs()
	}
}

impl Eq for NonPositiveInteger {}

impl Hash for NonPositiveInteger {
	fn hash<H: Hasher>(&self, h: &mut H) {
		match self.sign() {
			Sign::Zero => 0.hash(h),
			sign => {
				sign.hash(h);
				self.abs().hash(h)
			}
		}
	}
}

impl Ord for NonPositiveInteger {
	fn cmp(&self, other: &Self) -> Ordering {
		let sign = self.sign();
		let other_sign = other.sign();
		match sign.cmp(&other_sign) {
			Ordering::Equal => {
				let a = &self.abs();
				let b = &other.abs();

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

impl PartialOrd for NonPositiveInteger {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl NonPositiveIntegerBuf {
	pub fn zero() -> Self {
		unsafe { Self::new_unchecked("0".to_string()) }
	}

	pub fn minus_one() -> Self {
		unsafe { Self::new_unchecked("-1".to_string()) }
	}
}

impl Default for NonPositiveIntegerBuf {
	fn default() -> Self {
		Self::zero()
	}
}

impl PartialOrd for NonPositiveIntegerBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for NonPositiveIntegerBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for NonPositiveIntegerBuf {
				fn from(i: $ty) -> Self {
					unsafe { NonPositiveIntegerBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a NonPositiveInteger> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a NonPositiveInteger) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<NonPositiveIntegerBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: NonPositiveIntegerBuf) -> Result<Self, Overflow> {
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
				Some(b'-') => State::NonEmptyInteger,
				Some(b'+' | b'0') => State::Zero,
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
