use super::{Decimal, Double, Overflow};
use std::borrow::{Borrow, ToOwned};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct InvalidInteger;

/// XSD decimal.
#[derive(PartialEq, Eq, Hash)]
pub struct Integer(str);

impl Integer {
	#[inline(always)]
	pub fn new(s: &str) -> Result<&Self, InvalidInteger> {
		if check(s.chars()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err(InvalidInteger)
		}
	}

	#[inline(always)]
	pub unsafe fn new_unchecked(s: &str) -> &Self {
		std::mem::transmute(s)
	}

	#[inline(always)]
	pub fn as_str(&self) -> &str {
		&self.0
	}

	#[inline(always)]
	pub fn as_decimal(&self) -> &Decimal {
		self.into()
	}

	#[inline(always)]
	pub fn as_double(&self) -> &Double {
		self.into()
	}
}

impl Deref for Integer {
	type Target = str;

	#[inline(always)]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

impl ToOwned for Integer {
	type Owned = IntegerBuf;

	#[inline(always)]
	fn to_owned(&self) -> IntegerBuf {
		unsafe { IntegerBuf::new_unchecked(self.as_str().to_owned()) }
	}
}

impl fmt::Display for Integer {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl fmt::Debug for Integer {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl AsRef<Decimal> for Integer {
	fn as_ref(&self) -> &Decimal {
		self.into()
	}
}

impl AsRef<Double> for Integer {
	fn as_ref(&self) -> &Double {
		self.into()
	}
}

impl<'a> From<&'a IntegerBuf> for &'a Integer {
	#[inline(always)]
	fn from(b: &'a IntegerBuf) -> Self {
		b.as_ref()
	}
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct IntegerBuf(String);

impl IntegerBuf {
	#[inline(always)]
	pub fn new(s: String) -> Result<Self, InvalidInteger> {
		if check(s.chars()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err(InvalidInteger)
		}
	}

	#[inline(always)]
	pub unsafe fn new_unchecked(s: String) -> Self {
		std::mem::transmute(s)
	}

	#[inline(always)]
	pub fn as_integer(&self) -> &Integer {
		self.into()
	}
}

impl FromStr for IntegerBuf {
	type Err = InvalidInteger;

	fn from_str(s: &str) -> Result<Self, InvalidInteger> {
		Self::new(s.to_owned())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for IntegerBuf {
				fn from(i: $ty) -> Self {
					unsafe { IntegerBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a Integer> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a Integer) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<IntegerBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: IntegerBuf) -> Result<Self, Overflow> {
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
	usize,
	isize
}

impl Deref for IntegerBuf {
	type Target = Integer;

	#[inline(always)]
	fn deref(&self) -> &Integer {
		unsafe { Integer::new_unchecked(&self.0) }
	}
}

impl AsRef<Integer> for IntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Integer {
		unsafe { Integer::new_unchecked(&self.0) }
	}
}

impl AsRef<Decimal> for IntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Decimal {
		Integer::as_ref(self)
	}
}

impl AsRef<Double> for IntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Double {
		Integer::as_ref(self)
	}
}

impl Borrow<Integer> for IntegerBuf {
	#[inline(always)]
	fn borrow(&self) -> &Integer {
		unsafe { Integer::new_unchecked(&self.0) }
	}
}

impl fmt::Display for IntegerBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl fmt::Debug for IntegerBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

macro_rules! partial_eq {
	{ $($ty:ty),* } => {
		$(
			impl PartialEq<$ty> for Integer {
				#[inline(always)]
				fn eq(&self, other: &$ty) -> bool {
					self.as_str() == other
				}
			}

			impl PartialEq<$ty> for IntegerBuf {
				#[inline(always)]
				fn eq(&self, other: &$ty) -> bool {
					self.as_str() == other
				}
			}

			impl PartialEq<Integer> for $ty {
				#[inline(always)]
				fn eq(&self, other: &Integer) -> bool {
					self == other.as_str()
				}
			}

			impl PartialEq<IntegerBuf> for $ty {
				#[inline(always)]
				fn eq(&self, other: &IntegerBuf) -> bool {
					self == other.as_str()
				}
			}
		)*
	};
}

partial_eq! {
	str,
	String
}

impl PartialEq<Integer> for IntegerBuf {
	#[inline(always)]
	fn eq(&self, other: &Integer) -> bool {
		self == other
	}
}

impl PartialEq<IntegerBuf> for Integer {
	#[inline(always)]
	fn eq(&self, other: &IntegerBuf) -> bool {
		self == other
	}
}

fn check<C: Iterator<Item = char>>(mut chars: C) -> bool {
	enum State {
		Initial,
		NonEmptyInteger,
		Integer,
	}

	let mut state = State::Initial;

	loop {
		state = match state {
			State::Initial => match chars.next() {
				Some('+') => State::NonEmptyInteger,
				Some('-') => State::NonEmptyInteger,
				Some('0'..='9') => State::Integer,
				_ => break false,
			},
			State::NonEmptyInteger => match chars.next() {
				Some('0'..='9') => State::Integer,
				_ => break false,
			},
			State::Integer => match chars.next() {
				Some('0'..='9') => State::Integer,
				Some(_) => break false,
				None => break true,
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_01() {
		Integer::new("0").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_02() {
		Integer::new("+").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_03() {
		Integer::new("-").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_04() {
		Integer::new("012+").unwrap();
	}

	#[test]
	fn parse_05() {
		Integer::new("+42").unwrap();
	}

	#[test]
	fn parse_06() {
		Integer::new("-42").unwrap();
	}
}
