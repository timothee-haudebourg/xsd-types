use super::{
	Decimal, DecimalBuf, Float, FloatBuf, Integer, NonNegativeInteger, NonNegativeIntegerBuf,
	NonPositiveInteger, NonPositiveIntegerBuf, Overflow,
};
use std::borrow::{Borrow, ToOwned};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct InvalidDouble;

pub const NAN: &Double = unsafe { Double::new_unchecked_from_slice(b"NaN") };
pub const POSITIVE_INFINITY: &Double = unsafe { Double::new_unchecked_from_slice(b"INF") };
pub const NEGATIVE_INFINITY: &Double = unsafe { Double::new_unchecked_from_slice(b"-INF") };

/// Double number.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#double>
#[derive(PartialEq, Eq, Hash)]
pub struct Double([u8]);

impl Double {
	/// Creates a new `Double` from a string.
	///
	/// If the input string is ot a [valid XSD double](https://www.w3.org/TR/xmlschema-2/#double),
	/// an [`InvalidDouble`] error is returned.
	#[inline(always)]
	pub fn new<S: ?Sized + AsRef<[u8]>>(s: &S) -> Result<&Self, InvalidDouble> {
		if check(s.as_ref()) {
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
	pub unsafe fn new_unchecked<S: ?Sized + AsRef<[u8]>>(s: &S) -> &Self {
		std::mem::transmute(s.as_ref())
	}

	/// Creates a new `Double` from a byte slice without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD double](https://www.w3.org/TR/xmlschema-2/#double).
	#[inline(always)]
	pub const unsafe fn new_unchecked_from_slice(s: &[u8]) -> &Self {
		std::mem::transmute(s)
	}

	#[inline(always)]
	pub fn as_str(&self) -> &str {
		unsafe { core::str::from_utf8_unchecked(&self.0) }
	}

	#[inline(always)]
	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}

	pub fn is_infinite(&self) -> bool {
		matches!(&self.0, b"INF" | b"-INF")
	}

	pub fn is_finite(&self) -> bool {
		!matches!(&self.0, b"INF" | b"-INF" | b"NaN")
	}

	pub fn is_nan(&self) -> bool {
		&self.0 == b"NaN"
	}

	fn exponent_separator_index(&self) -> Option<usize> {
		for (i, c) in self.0.iter().enumerate() {
			if matches!(c, b'e' | b'E') {
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

impl AsRef<[u8]> for Double {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl AsRef<str> for Double {
	fn as_ref(&self) -> &str {
		self.as_str()
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

impl<'a> From<&'a Double> for f64 {
	fn from(i: &'a Double) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl From<DoubleBuf> for f64 {
	fn from(i: DoubleBuf) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl<'a> From<&'a Float> for f64 {
	fn from(i: &'a Float) -> Self {
		i.as_str().parse().unwrap()
	}
}

impl From<FloatBuf> for f64 {
	fn from(i: FloatBuf) -> Self {
		i.as_str().parse().unwrap()
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
		self.as_str().fmt(f)
	}
}

impl fmt::Debug for Double {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_str().fmt(f)
	}
}

impl<'a> From<&'a Integer> for &'a Double {
	#[inline(always)]
	fn from(d: &'a Integer) -> Self {
		unsafe { Double::new_unchecked(d) }
	}
}

impl<'a> From<&'a NonNegativeInteger> for &'a Double {
	#[inline(always)]
	fn from(d: &'a NonNegativeInteger) -> Self {
		unsafe { Double::new_unchecked(d) }
	}
}

impl<'a> From<&'a NonPositiveInteger> for &'a Double {
	#[inline(always)]
	fn from(d: &'a NonPositiveInteger) -> Self {
		unsafe { Double::new_unchecked(d) }
	}
}

impl<'a> From<&'a Decimal> for &'a Double {
	#[inline(always)]
	fn from(d: &'a Decimal) -> Self {
		unsafe { Double::new_unchecked(d) }
	}
}

impl<'a> From<&'a NonPositiveIntegerBuf> for &'a Double {
	#[inline(always)]
	fn from(d: &'a NonPositiveIntegerBuf) -> Self {
		d.as_ref()
	}
}

impl<'a> From<&'a NonNegativeIntegerBuf> for &'a Double {
	#[inline(always)]
	fn from(d: &'a NonNegativeIntegerBuf) -> Self {
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
pub struct DoubleBuf(Vec<u8>);

impl DoubleBuf {
	/// Creates a new `DoubleBuf` from a `String`.
	///
	/// If the input string is ot a [valid XSD double](https://www.w3.org/TR/xmlschema-2/#double),
	/// an [`InvalidDouble`] error is returned along with the input string.
	#[inline(always)]
	pub fn new<S: AsRef<[u8]> + Into<Vec<u8>>>(s: S) -> Result<Self, (InvalidDouble, S)> {
		if check(s.as_ref()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err((InvalidDouble, s))
		}
	}

	/// Creates a new `DoubleBuf` from a `String` without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD double](https://www.w3.org/TR/xmlschema-2/#double).
	#[inline(always)]
	pub unsafe fn new_unchecked(s: impl Into<Vec<u8>>) -> Self {
		std::mem::transmute(s.into())
	}

	#[inline(always)]
	pub fn nan() -> Self {
		NAN.to_owned()
	}

	#[inline(always)]
	pub fn positive_infinity() -> Self {
		POSITIVE_INFINITY.to_owned()
	}

	#[inline(always)]
	pub fn negative_infinity() -> Self {
		NEGATIVE_INFINITY.to_owned()
	}

	#[inline(always)]
	pub fn into_string(mut self) -> String {
		let buf = self.0.as_mut_ptr();
		let len = self.0.len();
		let capacity = self.0.capacity();
		core::mem::forget(self);
		unsafe { String::from_raw_parts(buf, len, capacity) }
	}
}

impl FromStr for DoubleBuf {
	type Err = InvalidDouble;

	fn from_str(s: &str) -> Result<Self, InvalidDouble> {
		Self::new(s.to_owned()).map_err(|(e, _)| e)
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
		self.as_str().fmt(f)
	}
}

impl fmt::Debug for DoubleBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_str().fmt(f)
	}
}

fn check(s: &[u8]) -> bool {
	s == b"INF" || s == b"-INF" || s == b"NaN" || check_normal(s.iter().cloned())
}

fn check_normal<C: Iterator<Item = u8>>(mut chars: C) -> bool {
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
				Some(b'+') => State::NonEmptyInteger,
				Some(b'-') => State::NonEmptyInteger,
				Some(b'.') => State::NonEmptyDecimal,
				Some(b'0'..=b'9') => State::Integer,
				_ => break false,
			},
			State::NonEmptyInteger => match chars.next() {
				Some(b'0'..=b'9') => State::Integer,
				Some(b'.') => State::Decimal,
				_ => break false,
			},
			State::Integer => match chars.next() {
				Some(b'0'..=b'9') => State::Integer,
				Some(b'.') => State::Decimal,
				Some(b'e' | b'E') => State::ExponentSign,
				Some(_) => break false,
				None => break true,
			},
			State::NonEmptyDecimal => match chars.next() {
				Some(b'0'..=b'9') => State::Decimal,
				_ => break false,
			},
			State::Decimal => match chars.next() {
				Some(b'0'..=b'9') => State::Decimal,
				Some(b'e' | b'E') => State::ExponentSign,
				Some(_) => break false,
				None => break true,
			},
			State::ExponentSign => match chars.next() {
				Some(b'+' | b'-') => State::NonEmptyExponent,
				Some(b'0'..=b'9') => State::Exponent,
				_ => break false,
			},
			State::NonEmptyExponent => match chars.next() {
				Some(b'0'..=b'9') => State::Exponent,
				_ => break false,
			},
			State::Exponent => match chars.next() {
				Some(b'0'..=b'9') => State::Exponent,
				Some(_) => break false,
				None => break true,
			},
		}
	}
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
		assert_eq!(DoubleBuf::from(1.0e10f64).to_string(), "1.0e10")
	}
}
