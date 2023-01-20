use super::{Decimal, DecimalBuf, Double, DoubleBuf, Float, FloatBuf, Integer, Overflow, Sign};
use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct InvalidNonNegativeInteger;

/// Non negative integer number.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger>
pub struct NonNegativeInteger([u8]);

impl NonNegativeInteger {
	/// Creates a new `NonNegativeInteger` from a string.
	///
	/// If the input string is ot a [valid XSD non negative integer](https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger),
	/// an [`InvalidInteger`] error is returned.
	#[inline(always)]
	pub fn new<S: ?Sized + AsRef<[u8]>>(s: &S) -> Result<&Self, InvalidNonNegativeInteger> {
		if check(s.as_ref().iter().cloned()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err(InvalidNonNegativeInteger)
		}
	}

	/// Creates a new `NonNegativeInteger` from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD non negative integer](https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger).
	#[inline(always)]
	pub unsafe fn new_unchecked<S: ?Sized + AsRef<[u8]>>(s: &S) -> &Self {
		std::mem::transmute(s.as_ref())
	}

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
	pub fn as_str(&self) -> &str {
		unsafe { core::str::from_utf8_unchecked(&self.0) }
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}

	#[inline(always)]
	pub fn as_decimal(&self) -> &Decimal {
		self.into()
	}

	#[inline(always)]
	pub fn as_float(&self) -> &Float {
		self.into()
	}

	#[inline(always)]
	pub fn as_double(&self) -> &Double {
		self.into()
	}
}

impl PartialEq for NonNegativeInteger {
	fn eq(&self, other: &Self) -> bool {
		self.canonical().0 == other.canonical().0
	}
}

impl Eq for NonNegativeInteger {}

impl Hash for NonNegativeInteger {
	fn hash<H: Hasher>(&self, h: &mut H) {
		match self.sign() {
			Sign::Zero => 0.hash(h),
			sign => {
				sign.hash(h);
				self.canonical().hash(h)
			}
		}
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

impl Deref for NonNegativeInteger {
	type Target = str;

	#[inline(always)]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

impl AsRef<[u8]> for NonNegativeInteger {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl AsRef<str> for NonNegativeInteger {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl ToOwned for NonNegativeInteger {
	type Owned = NonNegativeIntegerBuf;

	#[inline(always)]
	fn to_owned(&self) -> NonNegativeIntegerBuf {
		unsafe { NonNegativeIntegerBuf::new_unchecked(self.as_str().to_owned()) }
	}
}

impl fmt::Display for NonNegativeInteger {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_str().fmt(f)
	}
}

impl fmt::Debug for NonNegativeInteger {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl AsRef<Decimal> for NonNegativeInteger {
	fn as_ref(&self) -> &Decimal {
		self.into()
	}
}

impl AsRef<Float> for NonNegativeInteger {
	fn as_ref(&self) -> &Float {
		self.into()
	}
}

impl AsRef<Double> for NonNegativeInteger {
	fn as_ref(&self) -> &Double {
		self.into()
	}
}

impl<'a> From<&'a NonNegativeIntegerBuf> for &'a NonNegativeInteger {
	#[inline(always)]
	fn from(b: &'a NonNegativeIntegerBuf) -> Self {
		b.as_ref()
	}
}

impl<'a> TryFrom<&'a Decimal> for &'a NonNegativeInteger {
	type Error = InvalidNonNegativeInteger;

	#[inline(always)]
	fn try_from(i: &'a Decimal) -> Result<Self, Self::Error> {
		NonNegativeInteger::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a DecimalBuf> for &'a NonNegativeInteger {
	type Error = InvalidNonNegativeInteger;

	#[inline(always)]
	fn try_from(i: &'a DecimalBuf) -> Result<Self, Self::Error> {
		NonNegativeInteger::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a Float> for &'a NonNegativeInteger {
	type Error = InvalidNonNegativeInteger;

	#[inline(always)]
	fn try_from(i: &'a Float) -> Result<Self, Self::Error> {
		NonNegativeInteger::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a FloatBuf> for &'a NonNegativeInteger {
	type Error = InvalidNonNegativeInteger;

	#[inline(always)]
	fn try_from(i: &'a FloatBuf) -> Result<Self, Self::Error> {
		NonNegativeInteger::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a Double> for &'a NonNegativeInteger {
	type Error = InvalidNonNegativeInteger;

	#[inline(always)]
	fn try_from(i: &'a Double) -> Result<Self, Self::Error> {
		NonNegativeInteger::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a DoubleBuf> for &'a NonNegativeInteger {
	type Error = InvalidNonNegativeInteger;

	#[inline(always)]
	fn try_from(i: &'a DoubleBuf) -> Result<Self, Self::Error> {
		NonNegativeInteger::new(i.as_str())
	}
}

/// Owned non negative integer number.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger>
#[derive(Clone)]
pub struct NonNegativeIntegerBuf(Vec<u8>);

impl NonNegativeIntegerBuf {
	/// Creates a new `NonNegativeIntegerBuf` from a `String`.
	///
	/// If the input string is ot a [valid XSD non negative integer](https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger),
	/// an [`InvalidNonNegativeInteger`] error is returned along with the input string.
	#[inline(always)]
	pub fn new<S: AsRef<[u8]> + Into<Vec<u8>>>(
		s: S,
	) -> Result<Self, (InvalidNonNegativeInteger, S)> {
		if check(s.as_ref().iter().cloned()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err((InvalidNonNegativeInteger, s))
		}
	}

	/// Creates a new `NonNegativeIntegerBuf` from a `String` without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD non negative integer](https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger).
	#[inline(always)]
	pub unsafe fn new_unchecked(s: impl Into<Vec<u8>>) -> Self {
		std::mem::transmute(s.into())
	}

	pub fn zero() -> Self {
		unsafe { Self::new_unchecked("0".to_string()) }
	}

	pub fn one() -> Self {
		unsafe { Self::new_unchecked("1".to_string()) }
	}

	#[inline(always)]
	pub fn as_non_negative_integer(&self) -> &NonNegativeInteger {
		self.into()
	}

	#[inline(always)]
	pub fn as_integer(&self) -> &Integer {
		self.into()
	}

	#[inline(always)]
	pub fn into_bytes(self) -> Vec<u8> {
		self.0
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

impl Default for NonNegativeIntegerBuf {
	fn default() -> Self {
		Self::zero()
	}
}

impl PartialEq for NonNegativeIntegerBuf {
	fn eq(&self, other: &Self) -> bool {
		self.as_integer().eq(other.as_integer())
	}
}

impl Eq for NonNegativeIntegerBuf {}

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

impl FromStr for NonNegativeIntegerBuf {
	type Err = InvalidNonNegativeInteger;

	fn from_str(s: &str) -> Result<Self, InvalidNonNegativeInteger> {
		Self::new(s.to_owned()).map_err(|(e, _)| e)
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

impl TryFrom<DecimalBuf> for NonNegativeIntegerBuf {
	type Error = (InvalidNonNegativeInteger, DecimalBuf);

	#[inline(always)]
	fn try_from(i: DecimalBuf) -> Result<Self, Self::Error> {
		match Self::new(i.into_string()) {
			Ok(d) => Ok(d),
			Err((e, s)) => Err((e, unsafe { DecimalBuf::new_unchecked(s) })),
		}
	}
}

impl TryFrom<FloatBuf> for NonNegativeIntegerBuf {
	type Error = (InvalidNonNegativeInteger, FloatBuf);

	#[inline(always)]
	fn try_from(i: FloatBuf) -> Result<Self, Self::Error> {
		match Self::new(i.into_string()) {
			Ok(d) => Ok(d),
			Err((e, s)) => Err((e, unsafe { FloatBuf::new_unchecked(s) })),
		}
	}
}

impl TryFrom<DoubleBuf> for NonNegativeIntegerBuf {
	type Error = (InvalidNonNegativeInteger, DoubleBuf);

	#[inline(always)]
	fn try_from(i: DoubleBuf) -> Result<Self, Self::Error> {
		match Self::new(i.into_string()) {
			Ok(d) => Ok(d),
			Err((e, s)) => Err((e, unsafe { DoubleBuf::new_unchecked(s) })),
		}
	}
}

impl Deref for NonNegativeIntegerBuf {
	type Target = NonNegativeInteger;

	#[inline(always)]
	fn deref(&self) -> &NonNegativeInteger {
		unsafe { NonNegativeInteger::new_unchecked(&self.0) }
	}
}

impl AsRef<NonNegativeInteger> for NonNegativeIntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &NonNegativeInteger {
		unsafe { NonNegativeInteger::new_unchecked(&self.0) }
	}
}

impl AsRef<Decimal> for NonNegativeIntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Decimal {
		NonNegativeInteger::as_ref(self)
	}
}

impl AsRef<Float> for NonNegativeIntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Float {
		NonNegativeInteger::as_ref(self)
	}
}

impl AsRef<Double> for NonNegativeIntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Double {
		NonNegativeInteger::as_ref(self)
	}
}

impl AsRef<[u8]> for NonNegativeIntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl AsRef<str> for NonNegativeIntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl Borrow<NonNegativeInteger> for NonNegativeIntegerBuf {
	#[inline(always)]
	fn borrow(&self) -> &NonNegativeInteger {
		unsafe { NonNegativeInteger::new_unchecked(&self.0) }
	}
}

impl fmt::Display for NonNegativeIntegerBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_str().fmt(f)
	}
}

impl fmt::Debug for NonNegativeIntegerBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl PartialEq<NonNegativeInteger> for NonNegativeIntegerBuf {
	#[inline(always)]
	fn eq(&self, other: &NonNegativeInteger) -> bool {
		self == other
	}
}

impl PartialEq<NonNegativeIntegerBuf> for NonNegativeInteger {
	#[inline(always)]
	fn eq(&self, other: &NonNegativeIntegerBuf) -> bool {
		self == other
	}
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
				Some(b'+') => State::NonEmptyInteger,
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
		}
	}
}
