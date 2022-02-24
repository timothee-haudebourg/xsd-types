use super::{Double, Integer, IntegerBuf, Overflow};
use std::borrow::{Borrow, ToOwned};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct InvalidDecimal;

/// XSD decimal.
#[derive(PartialEq, Eq, Hash)]
pub struct Decimal(str);

impl Decimal {
	#[inline(always)]
	pub fn new(s: &str) -> Result<&Self, InvalidDecimal> {
		if check(s.chars()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err(InvalidDecimal)
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
	pub fn as_double(&self) -> &Double {
		self.into()
	}

	#[inline(always)]
	pub fn integer_part(&self) -> &Integer {
		match self.split_once('.') {
			Some((integer_part, _)) => unsafe { Integer::new_unchecked(integer_part) },
			None => unsafe { Integer::new_unchecked(self) },
		}
	}

	#[inline(always)]
	pub fn fractional_part(&self) -> Option<&Integer> {
		self.split_once('.')
			.map(|(_, fractional_part)| unsafe { Integer::new_unchecked(fractional_part) })
	}
}

macro_rules! integer_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for DecimalBuf {
				fn from(i: $ty) -> Self {
					unsafe { DecimalBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a Decimal> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a Decimal) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<DecimalBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: DecimalBuf) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}
		)*
	};
}

integer_conversion! {
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


const DTOA_CONFIG: pretty_dtoa::FmtFloatConfig =
	pretty_dtoa::FmtFloatConfig::default().force_no_e_notation();

impl From<f32> for DecimalBuf {
	fn from(i: f32) -> Self {
		unsafe { DecimalBuf::new_unchecked(pretty_dtoa::ftoa(i, DTOA_CONFIG)) }
	}
}

impl<'a> TryFrom<&'a Decimal> for f32 {
	type Error = <f32 as std::str::FromStr>::Err;

	fn try_from(i: &'a Decimal) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl TryFrom<DecimalBuf> for f32 {
	type Error = <f32 as std::str::FromStr>::Err;

	fn try_from(i: DecimalBuf) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl From<f64> for DecimalBuf {
	fn from(i: f64) -> Self {
		unsafe { DecimalBuf::new_unchecked(pretty_dtoa::dtoa(i, DTOA_CONFIG)) }
	}
}

impl<'a> TryFrom<&'a Decimal> for f64 {
	type Error = <f64 as std::str::FromStr>::Err;

	fn try_from(i: &'a Decimal) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl TryFrom<DecimalBuf> for f64 {
	type Error = <f64 as std::str::FromStr>::Err;

	fn try_from(i: DecimalBuf) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl Deref for Decimal {
	type Target = str;

	#[inline(always)]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

impl ToOwned for Decimal {
	type Owned = DecimalBuf;

	#[inline(always)]
	fn to_owned(&self) -> DecimalBuf {
		unsafe { DecimalBuf::new_unchecked(self.as_str().to_owned()) }
	}
}

impl fmt::Display for Decimal {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl fmt::Debug for Decimal {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl AsRef<Double> for Decimal {
	#[inline(always)]
	fn as_ref(&self) -> &Double {
		self.as_double()
	}
}

impl<'a> From<&'a Integer> for &'a Decimal {
	#[inline(always)]
	fn from(d: &'a Integer) -> Self {
		unsafe { Decimal::new_unchecked(d) }
	}
}

impl<'a> From<&'a IntegerBuf> for &'a Decimal {
	#[inline(always)]
	fn from(d: &'a IntegerBuf) -> Self {
		d.as_ref()
	}
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DecimalBuf(String);

impl DecimalBuf {
	#[inline(always)]
	pub fn new(s: String) -> Result<Self, InvalidDecimal> {
		if check(s.chars()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err(InvalidDecimal)
		}
	}

	#[inline(always)]
	pub unsafe fn new_unchecked(s: String) -> Self {
		std::mem::transmute(s)
	}

	#[inline(always)]
	pub fn as_decimal(&self) -> &Decimal {
		unsafe { Decimal::new_unchecked(&self.0) }
	}
}

impl FromStr for DecimalBuf {
	type Err = InvalidDecimal;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, InvalidDecimal> {
		Self::new(s.to_owned())
	}
}

impl Deref for DecimalBuf {
	type Target = Decimal;

	#[inline(always)]
	fn deref(&self) -> &Decimal {
		unsafe { Decimal::new_unchecked(&self.0) }
	}
}

impl AsRef<Decimal> for DecimalBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Decimal {
		self.as_decimal()
	}
}

impl AsRef<Double> for DecimalBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Double {
		Decimal::as_ref(self)
	}
}

impl Borrow<Decimal> for DecimalBuf {
	#[inline(always)]
	fn borrow(&self) -> &Decimal {
		unsafe { Decimal::new_unchecked(&self.0) }
	}
}

impl<'a> From<&'a DecimalBuf> for &'a Decimal {
	#[inline(always)]
	fn from(b: &'a DecimalBuf) -> Self {
		b.as_ref()
	}
}

impl fmt::Display for DecimalBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl fmt::Debug for DecimalBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

fn check<C: Iterator<Item = char>>(mut chars: C) -> bool {
	enum State {
		Initial,
		NonEmptyInteger,
		Integer,
		NonEmptyDecimal,
		Decimal,
	}

	let mut state = State::Initial;

	loop {
		state = match state {
			State::Initial => match chars.next() {
				Some('+') => State::NonEmptyInteger,
				Some('-') => State::NonEmptyInteger,
				Some('.') => State::NonEmptyDecimal,
				Some('0'..='9') => State::Integer,
				_ => break false,
			},
			State::NonEmptyInteger => match chars.next() {
				Some('0'..='9') => State::Integer,
				Some('.') => State::Decimal,
				_ => break false,
			},
			State::Integer => match chars.next() {
				Some('0'..='9') => State::Integer,
				Some('.') => State::Decimal,
				Some(_) => break false,
				None => break true,
			},
			State::NonEmptyDecimal => match chars.next() {
				Some('0'..='9') => State::Decimal,
				_ => break false,
			},
			State::Decimal => match chars.next() {
				Some('0'..='9') => State::Decimal,
				Some(_) => break false,
				None => break true,
			}
		}
	}
}

macro_rules! partial_eq {
	{ $($ty:ty),* } => {
		$(
			impl PartialEq<$ty> for Decimal {
				#[inline(always)]
				fn eq(&self, other: &$ty) -> bool {
					self.as_str() == other
				}
			}

			impl PartialEq<$ty> for DecimalBuf {
				#[inline(always)]
				fn eq(&self, other: &$ty) -> bool {
					self.as_str() == other
				}
			}

			impl PartialEq<Decimal> for $ty {
				#[inline(always)]
				fn eq(&self, other: &Decimal) -> bool {
					self == other.as_str()
				}
			}

			impl PartialEq<DecimalBuf> for $ty {
				#[inline(always)]
				fn eq(&self, other: &DecimalBuf) -> bool {
					self == other.as_str()
				}
			}
		)*
	};
}

partial_eq! {
	str,
	String,
	Integer
}

impl PartialEq<Decimal> for DecimalBuf {
	#[inline(always)]
	fn eq(&self, other: &Decimal) -> bool {
		self.as_decimal() == other
	}
}

impl<'a> PartialEq<&'a Decimal> for DecimalBuf {
	#[inline(always)]
	fn eq(&self, other: &&'a Decimal) -> bool {
		self.as_decimal() == *other
	}
}

impl PartialEq<DecimalBuf> for Decimal {
	#[inline(always)]
	fn eq(&self, other: &DecimalBuf) -> bool {
		self == other.as_decimal()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_01() {
		Decimal::new("0").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_02() {
		Decimal::new("+").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_03() {
		Decimal::new("-").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_04() {
		Decimal::new("012+").unwrap();
	}

	#[test]
	fn parse_05() {
		Decimal::new("+42").unwrap();
	}

	#[test]
	fn parse_06() {
		Decimal::new("-42").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_07() {
		Decimal::new(".").unwrap();
	}

	#[test]
	fn parse_08() {
		Decimal::new(".0").unwrap();
	}

	#[test]
	fn parse_09() {
		Decimal::new("0.").unwrap();
	}

	#[test]
	fn parse_10() {
		Decimal::new("42.0").unwrap();
	}

	#[test]
	fn format_01() {
		assert_eq!(DecimalBuf::from(1.0e10f32).to_string(), "10000000000.0")
	}
}