use super::{lexical_form, Decimal, Float, FloatBuf, Integer, Overflow};
use std::borrow::{Borrow, ToOwned};
use std::fmt;
use std::hash::Hash;

lexical_form! {
	/// Double number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#double>
	ty: Double,

	/// Owned double number.
	///
	/// See: <https://www.w3.org/TR/xmlschema-2/#double>
	buffer: DoubleBuf,

	/// Creates a new double from a string.
	///
	/// If the input string is ot a [valid XSD double](https://www.w3.org/TR/xmlschema-2/#double),
	/// an [`InvalidDouble`] error is returned.
	new,

	/// Creates a new double from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD double](https://www.w3.org/TR/xmlschema-2/#double).
	new_unchecked,

	value: crate::Double,
	error: InvalidDouble,
	as_ref: as_double,
	parent_forms: {}
}

pub const NAN: &Double = unsafe { Double::new_unchecked_from_slice(b"NaN") };
pub const POSITIVE_INFINITY: &Double = unsafe { Double::new_unchecked_from_slice(b"INF") };
pub const NEGATIVE_INFINITY: &Double = unsafe { Double::new_unchecked_from_slice(b"-INF") };

impl Double {
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

	pub fn value(&self) -> crate::Double {
		self.into()
	}
}

impl PartialEq for Double {
	fn eq(&self, other: &Self) -> bool {
		self.as_bytes() == other.as_bytes()
	}
}

impl Eq for Double {}

impl Hash for Double {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.as_bytes().hash(state)
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

impl<'a> From<&'a Decimal> for &'a Double {
	#[inline(always)]
	fn from(d: &'a Decimal) -> Self {
		unsafe { Double::new_unchecked(d) }
	}
}

impl DoubleBuf {
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
}

fn check_bytes(s: &[u8]) -> bool {
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
