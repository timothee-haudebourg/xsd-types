use super::{Decimal, DecimalBuf, Integer, IntegerBuf, Overflow};
use std::borrow::{Borrow, ToOwned};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct InvalidDouble;

pub const NAN: &Double = unsafe { Double::new_unchecked("NaN") };
pub const POSITIVE_INFINITY: &Double = unsafe { Double::new_unchecked("INF") };
pub const NEGATIVE_INFINITY: &Double = unsafe { Double::new_unchecked("-INF") };

/// Double number.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#double>
#[derive(PartialEq, Eq, Hash)]
pub struct Double(str);

impl Double {
	/// Creates a new `Double` from a string.
	///
	/// If the input string is ot a [valid XSD double](https://www.w3.org/TR/xmlschema-2/#double),
	/// an [`InvalidDouble`] error is returned.
	#[inline(always)]
	pub fn new(s: &str) -> Result<&Self, InvalidDouble> {
		if check(s) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err(InvalidDouble)
		}
	}

	/// Creates a new `Double` from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD double](https://www.w3.org/TR/xmlschema-2/#double).
	#[inline(always)]
	pub const unsafe fn new_unchecked(s: &str) -> &Self {
		std::mem::transmute(s)
	}

	#[inline(always)]
	pub fn as_str(&self) -> &str {
		&self.0
	}

	pub fn is_infinite(&self) -> bool {
		matches!(self.as_str(), "INF" | "-INF")
	}

	pub fn is_finite(&self) -> bool {
		!matches!(self.as_str(), "INF" | "-INF" | "NaN")
	}

	pub fn is_nan(&self) -> bool {
		self.as_str() == "NaN"
	}

	fn exponent_separator_index(&self) -> Option<usize> {
		for (i, c) in self.as_str().char_indices() {
			if matches!(c, 'e' | 'E') {
				return Some(i);
			}
		}

		None
	}

	pub fn mantissa(&self) -> Option<&Decimal> {
		if self.is_finite() {
			Some(match self.exponent_separator_index() {
				Some(e) => unsafe { Decimal::new_unchecked(&self[..e]) },
				None => unsafe { Decimal::new_unchecked(self) },
			})
		} else {
			None
		}
	}

	pub fn exponent(&self) -> Option<&Integer> {
		if self.is_finite() {
			self.exponent_separator_index()
				.map(|e| unsafe { Integer::new_unchecked(&self[(e + 1)..]) })
		} else {
			None
		}
	}
}

macro_rules! integer_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for DoubleBuf {
				fn from(i: $ty) -> Self {
					unsafe { DoubleBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a Double> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a Double) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<DoubleBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: DoubleBuf) -> Result<Self, Overflow> {
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

const DTOA_CONFIG: pretty_dtoa::FmtFloatConfig = pretty_dtoa::FmtFloatConfig::default();

impl From<f32> for DoubleBuf {
	fn from(i: f32) -> Self {
		if i.is_finite() {
			unsafe { DoubleBuf::new_unchecked(pretty_dtoa::ftoa(i, DTOA_CONFIG)) }
		} else if i.is_nan() {
			DoubleBuf::nan()
		} else if i.is_sign_positive() {
			DoubleBuf::positive_infinity()
		} else {
			DoubleBuf::negative_infinity()
		}
	}
}

impl<'a> TryFrom<&'a Double> for f32 {
	type Error = <f32 as std::str::FromStr>::Err;

	fn try_from(i: &'a Double) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl TryFrom<DoubleBuf> for f32 {
	type Error = <f32 as std::str::FromStr>::Err;

	fn try_from(i: DoubleBuf) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl From<f64> for DoubleBuf {
	fn from(i: f64) -> Self {
		if i.is_finite() {
			unsafe { DoubleBuf::new_unchecked(pretty_dtoa::dtoa(i, DTOA_CONFIG)) }
		} else if i.is_nan() {
			DoubleBuf::nan()
		} else if i.is_sign_positive() {
			DoubleBuf::positive_infinity()
		} else {
			DoubleBuf::negative_infinity()
		}
	}
}

impl<'a> TryFrom<&'a Double> for f64 {
	type Error = <f64 as std::str::FromStr>::Err;

	fn try_from(i: &'a Double) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl TryFrom<DoubleBuf> for f64 {
	type Error = <f64 as std::str::FromStr>::Err;

	fn try_from(i: DoubleBuf) -> Result<Self, Self::Error> {
		i.as_str().parse()
	}
}

impl Deref for Double {
	type Target = str;

	#[inline(always)]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

impl ToOwned for Double {
	type Owned = DoubleBuf;

	#[inline(always)]
	fn to_owned(&self) -> DoubleBuf {
		unsafe { DoubleBuf::new_unchecked(self.as_str().to_owned()) }
	}
}

impl fmt::Display for Double {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl fmt::Debug for Double {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl<'a> From<&'a Integer> for &'a Double {
	#[inline(always)]
	fn from(d: &'a Integer) -> Self {
		unsafe { Double::new_unchecked(d) }
	}
}

impl<'a> From<&'a Decimal> for &'a Double {
	#[inline(always)]
	fn from(d: &'a Decimal) -> Self {
		unsafe { Double::new_unchecked(d) }
	}
}

impl<'a> From<&'a IntegerBuf> for &'a Double {
	#[inline(always)]
	fn from(d: &'a IntegerBuf) -> Self {
		d.as_ref()
	}
}

impl<'a> From<&'a DecimalBuf> for &'a Double {
	#[inline(always)]
	fn from(d: &'a DecimalBuf) -> Self {
		d.as_ref()
	}
}

impl<'a> From<&'a DoubleBuf> for &'a Double {
	#[inline(always)]
	fn from(b: &'a DoubleBuf) -> Self {
		b.as_ref()
	}
}

/// Owned double number.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#double>
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DoubleBuf(String);

impl DoubleBuf {
	/// Creates a new `DoubleBuf` from a `String`.
	///
	/// If the input string is ot a [valid XSD double](https://www.w3.org/TR/xmlschema-2/#double),
	/// an [`InvalidDouble`] error is returned.
	#[inline(always)]
	pub fn new(s: String) -> Result<Self, InvalidDouble> {
		if check(&s) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err(InvalidDouble)
		}
	}

	/// Creates a new `DoubleBuf` from a `String` without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD double](https://www.w3.org/TR/xmlschema-2/#double).
	#[inline(always)]
	pub unsafe fn new_unchecked(s: String) -> Self {
		std::mem::transmute(s)
	}

	pub fn nan() -> Self {
		NAN.to_owned()
	}

	pub fn positive_infinity() -> Self {
		POSITIVE_INFINITY.to_owned()
	}

	pub fn negative_infinity() -> Self {
		NEGATIVE_INFINITY.to_owned()
	}

	#[inline(always)]
	pub fn from_suffix(suffix: &str) -> Result<Self, InvalidDouble> {
		Self::new(format!("_:{}", suffix))
	}
}

impl FromStr for DoubleBuf {
	type Err = InvalidDouble;

	fn from_str(s: &str) -> Result<Self, InvalidDouble> {
		Self::new(s.to_owned())
	}
}

impl Deref for DoubleBuf {
	type Target = Double;

	#[inline(always)]
	fn deref(&self) -> &Double {
		unsafe { Double::new_unchecked(&self.0) }
	}
}

impl AsRef<Double> for DoubleBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Double {
		unsafe { Double::new_unchecked(&self.0) }
	}
}

impl Borrow<Double> for DoubleBuf {
	#[inline(always)]
	fn borrow(&self) -> &Double {
		unsafe { Double::new_unchecked(&self.0) }
	}
}

impl fmt::Display for DoubleBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl fmt::Debug for DoubleBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

fn check(s: &str) -> bool {
	s == "INF" || s == "-INF" || s == "NaN" || check_normal(s.chars())
}

fn check_normal<C: Iterator<Item = char>>(mut chars: C) -> bool {
	enum State {
		Initial,
		NonEmptyInteger,
		Integer,
		NonEmptyDecimal,
		Decimal,
		ExponentSign,
		NonEmptyExponent,
		Exponent,
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
				Some('e' | 'E') => State::ExponentSign,
				Some(_) => break false,
				None => break true,
			},
			State::NonEmptyDecimal => match chars.next() {
				Some('0'..='9') => State::Decimal,
				_ => break false,
			},
			State::Decimal => match chars.next() {
				Some('0'..='9') => State::Decimal,
				Some('e' | 'E') => State::ExponentSign,
				Some(_) => break false,
				None => break true,
			},
			State::ExponentSign => match chars.next() {
				Some('+' | '-') => State::NonEmptyExponent,
				Some('0'..='9') => State::Exponent,
				_ => break false,
			},
			State::NonEmptyExponent => match chars.next() {
				Some('0'..='9') => State::Exponent,
				_ => break false,
			},
			State::Exponent => match chars.next() {
				Some('0'..='9') => State::Exponent,
				Some(_) => break false,
				None => break true,
			},
		}
	}
}

macro_rules! partial_eq {
	{ $($ty:ty),* } => {
		$(
			impl PartialEq<$ty> for Double {
				#[inline(always)]
				fn eq(&self, other: &$ty) -> bool {
					self.as_str() == other
				}
			}

			impl PartialEq<$ty> for DoubleBuf {
				#[inline(always)]
				fn eq(&self, other: &$ty) -> bool {
					self.as_str() == other
				}
			}

			impl PartialEq<Double> for $ty {
				#[inline(always)]
				fn eq(&self, other: &Double) -> bool {
					self == other.as_str()
				}
			}

			impl PartialEq<DoubleBuf> for $ty {
				#[inline(always)]
				fn eq(&self, other: &DoubleBuf) -> bool {
					self == other.as_str()
				}
			}
		)*
	};
}

partial_eq! {
	str,
	String,
	Integer,
	Decimal
}

impl PartialEq<Double> for DoubleBuf {
	#[inline(always)]
	fn eq(&self, other: &Double) -> bool {
		self.as_ref() == other
	}
}

impl<'a> PartialEq<&'a Double> for DoubleBuf {
	#[inline(always)]
	fn eq(&self, other: &&'a Double) -> bool {
		self.as_ref() == *other
	}
}

impl PartialEq<DoubleBuf> for Double {
	#[inline(always)]
	fn eq(&self, other: &DoubleBuf) -> bool {
		self == other.as_ref()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_01() {
		Double::new("0").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_02() {
		Double::new("+").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_03() {
		Double::new("-").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_04() {
		Double::new("012+").unwrap();
	}

	#[test]
	fn parse_05() {
		Double::new("+42").unwrap();
	}

	#[test]
	fn parse_06() {
		Double::new("-42").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_07() {
		Double::new(".").unwrap();
	}

	#[test]
	fn parse_08() {
		Double::new(".0").unwrap();
	}

	#[test]
	fn parse_09() {
		Double::new("0.").unwrap();
	}

	#[test]
	fn parse_10() {
		Double::new("42.0").unwrap();
	}

	#[test]
	fn parse_11() {
		Double::new("INF").unwrap();
	}

	#[test]
	fn parse_12() {
		Double::new("-INF").unwrap();
	}

	#[test]
	fn parse_13() {
		Double::new("NaN").unwrap();
	}

	#[test]
	fn parse_14() {
		Double::new(".0e1").unwrap();
	}

	#[test]
	fn parse_15() {
		Double::new("0.e1").unwrap();
	}

	#[test]
	fn parse_16() {
		Double::new("42E10").unwrap();
	}

	#[test]
	fn parse_17() {
		Double::new("-42E+10").unwrap();
	}

	#[test]
	fn parse_18() {
		Double::new("-42E-10").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_19() {
		Double::new("+42E-10e").unwrap();
	}

	#[test]
	fn parse_20() {
		Double::new("+42E-10").unwrap();
	}

	#[test]
	fn parse_21() {
		let d = Double::new("+01234E-56789").unwrap();
		assert_eq!(d.mantissa(), Some(Decimal::new("+01234").unwrap()));
		assert_eq!(d.exponent(), Some(Integer::new("-56789").unwrap()));
	}

	#[test]
	fn parse_22() {
		let a = DoubleBuf::new("+01234E-56789".to_string()).unwrap();
		let b = Double::new("+01234E-56789").unwrap();
		assert_eq!(a, b)
	}

	#[test]
	fn format_01() {
		assert_eq!(DoubleBuf::from(1.0e10f32).to_string(), "1.0e10")
	}
}
