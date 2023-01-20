use super::{
	Decimal, DecimalBuf, Double, DoubleBuf, Float, FloatBuf, Integer, NonNegativeInteger, Overflow,
	Sign,
};
use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct InvalidNonPositiveInteger;

/// NonPositiveInteger number.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#integer>
pub struct NonPositiveInteger([u8]);

impl NonPositiveInteger {
	/// Creates a new `NonPositiveInteger` from a string.
	///
	/// If the input string is ot a [valid XSD non positive integer](https://www.w3.org/TR/xmlschema-2/#nonPositiveInteger),
	/// an [`InvalidNonPositiveInteger`] error is returned.
	#[inline(always)]
	pub fn new<S: ?Sized + AsRef<[u8]>>(s: &S) -> Result<&Self, InvalidNonPositiveInteger> {
		if check(s.as_ref().iter().cloned()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err(InvalidNonPositiveInteger)
		}
	}

	/// Creates a new `NonPositiveInteger` from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD non positive integer](https://www.w3.org/TR/xmlschema-2/#nonPositiveInteger).
	#[inline(always)]
	pub unsafe fn new_unchecked<S: ?Sized + AsRef<[u8]>>(s: &S) -> &Self {
		std::mem::transmute(s.as_ref())
	}

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

impl Deref for NonPositiveInteger {
	type Target = str;

	#[inline(always)]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

impl AsRef<[u8]> for NonPositiveInteger {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl AsRef<str> for NonPositiveInteger {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl ToOwned for NonPositiveInteger {
	type Owned = NonPositiveIntegerBuf;

	#[inline(always)]
	fn to_owned(&self) -> NonPositiveIntegerBuf {
		unsafe { NonPositiveIntegerBuf::new_unchecked(self.as_str().to_owned()) }
	}
}

impl fmt::Display for NonPositiveInteger {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_str().fmt(f)
	}
}

impl fmt::Debug for NonPositiveInteger {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl AsRef<Decimal> for NonPositiveInteger {
	fn as_ref(&self) -> &Decimal {
		self.into()
	}
}

impl AsRef<Float> for NonPositiveInteger {
	fn as_ref(&self) -> &Float {
		self.into()
	}
}

impl AsRef<Double> for NonPositiveInteger {
	fn as_ref(&self) -> &Double {
		self.into()
	}
}

impl<'a> From<&'a NonPositiveIntegerBuf> for &'a NonPositiveInteger {
	#[inline(always)]
	fn from(b: &'a NonPositiveIntegerBuf) -> Self {
		b.as_ref()
	}
}

impl<'a> TryFrom<&'a Decimal> for &'a NonPositiveInteger {
	type Error = InvalidNonPositiveInteger;

	#[inline(always)]
	fn try_from(i: &'a Decimal) -> Result<Self, Self::Error> {
		NonPositiveInteger::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a DecimalBuf> for &'a NonPositiveInteger {
	type Error = InvalidNonPositiveInteger;

	#[inline(always)]
	fn try_from(i: &'a DecimalBuf) -> Result<Self, Self::Error> {
		NonPositiveInteger::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a Float> for &'a NonPositiveInteger {
	type Error = InvalidNonPositiveInteger;

	#[inline(always)]
	fn try_from(i: &'a Float) -> Result<Self, Self::Error> {
		NonPositiveInteger::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a FloatBuf> for &'a NonPositiveInteger {
	type Error = InvalidNonPositiveInteger;

	#[inline(always)]
	fn try_from(i: &'a FloatBuf) -> Result<Self, Self::Error> {
		NonPositiveInteger::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a Double> for &'a NonPositiveInteger {
	type Error = InvalidNonPositiveInteger;

	#[inline(always)]
	fn try_from(i: &'a Double) -> Result<Self, Self::Error> {
		NonPositiveInteger::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a DoubleBuf> for &'a NonPositiveInteger {
	type Error = InvalidNonPositiveInteger;

	#[inline(always)]
	fn try_from(i: &'a DoubleBuf) -> Result<Self, Self::Error> {
		NonPositiveInteger::new(i.as_str())
	}
}

/// Owned integer number.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#integer>
#[derive(Clone)]
pub struct NonPositiveIntegerBuf(Vec<u8>);

impl NonPositiveIntegerBuf {
	/// Creates a new `IntegerBuf` from a `String`.
	///
	/// If the input string is ot a [valid XSD non positive integer](https://www.w3.org/TR/xmlschema-2/#nonPositiveInteger),
	/// an [`InvalidNonPositiveInteger`] error is returned along with the input string.
	#[inline(always)]
	pub fn new<S: AsRef<[u8]> + Into<Vec<u8>>>(
		s: S,
	) -> Result<Self, (InvalidNonPositiveInteger, S)> {
		if check(s.as_ref().iter().cloned()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err((InvalidNonPositiveInteger, s))
		}
	}

	/// Creates a new `IntegerBuf` from a `String` without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD non positive integer](https://www.w3.org/TR/xmlschema-2/#nonPositiveInteger).
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
	pub fn as_non_positive_integer(&self) -> &NonPositiveInteger {
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

	fn abs_incr(&mut self) {
		let bytes = self.0.as_mut_slice();
		let mut i = bytes.len();

		while i > 0 {
			i -= 1;
			match bytes[i] {
				b'+' => bytes[i] = b'1',
				b'-' => break,
				b'0'..=b'8' => {
					bytes[i] += 1;
					return;
				}
				_ => {
					bytes[i] = b'0';
				}
			}
		}
	}

	fn abs_decr(&mut self) {
		let bytes = self.0.as_mut_slice();
		let mut i = bytes.len();

		while i > 0 {
			i -= 1;
			match bytes[i] {
				b'1' => {
					bytes[i] -= 1;
					break; // might be zero
				}
				b'2'..=b'8' => {
					bytes[i] -= 1;
					return; // non zero
				}
				_ => {
					bytes[i] = b'9';
				}
			}
		}

		// If we are here, the result might be zero.
		while i > 0 {
			i -= 1;
			if matches!(bytes[i], b'1'..=b'9') {
				return; // non zero
			}
		}

		// If we are here, the result is zero.
		self.0.clear();
		self.0.push(b'0');
	}

	pub fn incr(&mut self) {
		match self.sign() {
			Sign::Negative => self.abs_decr(),
			Sign::Positive => self.abs_incr(),
			Sign::Zero => {
				self.0.clear();
				self.0.push(b'1')
			}
		}
	}

	pub fn decr(&mut self) {
		match self.sign() {
			Sign::Negative => self.abs_incr(),
			Sign::Positive => self.abs_decr(),
			Sign::Zero => {
				self.0.clear();
				self.0.push(b'-');
				self.0.push(b'1')
			}
		}
	}

	// fn insert_zeros(&mut self, i: usize, len: usize) {
	// 	let new_len = self.0.len() + len;
	// 	self.0.resize(new_len, 0);
	// 	unsafe {
	// 		core::ptr::copy_nonoverlapping(self.0.as_mut_ptr().add(i), self.0.as_mut_ptr().add(i+len), len)
	// 	}
	// 	self.0[i..(i+len)].fill(0)
	// }

	// pub fn add(&mut self, other: &NonPositiveInteger) {
	// 	todo!()
	// }

	// fn abs_add(&mut self, other: &NonPositiveInteger) {
	// 	let bytes = self.0.as_mut_slice();
	// 	let other_bytes = other.as_bytes();

	// 	let mut i = bytes.len();
	// 	let mut j = other_bytes.len();

	// 	let offset = if bytes[0] == b'0' { 0 } else { 1 };
	// 	let other_offset = if other_bytes[0] == b'0' { 0 } else { 1 };

	// 	let mut acc = 0;
	// 	while i > offset && j > other_offset {
	// 		i -= 1;
	// 		j -= 1;

	// 		let r = bytes[i] - b'0' + other_bytes[j] - b'0' + acc;
	// 		acc = r / 10;
	// 		bytes[i] = b'0' + (r % 10);
	// 	}

	// 	while i > offset {
	// 		i -= 1;

	// 		let r = bytes[i] - b'0' + acc;
	// 		acc = r / 10;
	// 		bytes[i] = b'0' + (r % 10);
	// 	}

	// 	if j > other_offset {
	// 		let padding = j-other_offset;
	// 		self.insert_zeros(offset, padding);
	// 		i += padding;

	// 		while j > other_offset {
	// 			i -= 1;
	// 			j -= 1;

	// 			let r = other_bytes[i] - b'0' + acc;
	// 			acc = r / 10;
	// 			bytes[i] = b'0' + (r % 10);
	// 		}
	// 	}

	// 	if acc > 0 {
	// 		self.0.insert(offset, b'0' + acc)
	// 	}
	// }

	// pub fn sub(&mut self, other: &NonPositiveInteger) {
	// 	todo!()
	// }
}

impl Default for NonPositiveIntegerBuf {
	fn default() -> Self {
		Self::zero()
	}
}

impl PartialEq for NonPositiveIntegerBuf {
	fn eq(&self, other: &Self) -> bool {
		self.as_integer().eq(other.as_integer())
	}
}

impl Eq for NonPositiveIntegerBuf {}

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

// impl std::ops::Add for IntegerBuf {
// 	type Output = Self;

// 	fn add(mut self, rhs: Self) -> Self {
// 		IntegerBuf::add(&mut self, rhs.as_integer());
// 		self
// 	}
// }

// impl<'a> std::ops::Add<&'a IntegerBuf> for IntegerBuf {
// 	type Output = Self;

// 	fn add(mut self, rhs: &'a IntegerBuf) -> Self {
// 		IntegerBuf::add(&mut self, rhs.as_integer());
// 		self
// 	}
// }

// impl<'a> std::ops::Add<&'a NonPositiveInteger> for IntegerBuf {
// 	type Output = Self;

// 	fn add(mut self, rhs: &'a NonPositiveInteger) -> Self {
// 		IntegerBuf::add(&mut self, rhs);
// 		self
// 	}
// }

// impl std::ops::AddAssign for IntegerBuf {
// 	fn add_assign(&mut self, rhs: Self) {
// 		self.add(rhs.as_integer())
// 	}
// }

// impl<'a> std::ops::AddAssign<&'a IntegerBuf> for IntegerBuf {
// 	fn add_assign(&mut self, rhs: &'a IntegerBuf) {
// 		self.add(rhs.as_integer())
// 	}
// }

// impl<'a> std::ops::AddAssign<&'a NonPositiveInteger> for IntegerBuf {
// 	fn add_assign(&mut self, rhs: &'a NonPositiveInteger) {
// 		self.add(rhs)
// 	}
// }

// impl std::ops::Sub for IntegerBuf {
// 	type Output = Self;

// 	fn sub(mut self, rhs: Self) -> Self {
// 		IntegerBuf::sub(&mut self, rhs.as_integer());
// 		self
// 	}
// }

// impl<'a> std::ops::Sub<&'a IntegerBuf> for IntegerBuf {
// 	type Output = Self;

// 	fn sub(mut self, rhs: &'a IntegerBuf) -> Self {
// 		IntegerBuf::sub(&mut self, rhs.as_integer());
// 		self
// 	}
// }

// impl<'a> std::ops::Sub<&'a NonPositiveInteger> for IntegerBuf {
// 	type Output = Self;

// 	fn sub(mut self, rhs: &'a NonPositiveInteger) -> Self {
// 		IntegerBuf::sub(&mut self, rhs);
// 		self
// 	}
// }

// impl std::ops::SubAssign for IntegerBuf {
// 	fn sub_assign(&mut self, rhs: Self) {
// 		self.sub(rhs.as_integer())
// 	}
// }

// impl<'a> std::ops::SubAssign<&'a IntegerBuf> for IntegerBuf {
// 	fn sub_assign(&mut self, rhs: &'a IntegerBuf) {
// 		self.sub(rhs.as_integer())
// 	}
// }

// impl<'a> std::ops::SubAssign<&'a NonPositiveInteger> for IntegerBuf {
// 	fn sub_assign(&mut self, rhs: &'a NonPositiveInteger) {
// 		self.sub(rhs)
// 	}
// }

impl FromStr for NonPositiveIntegerBuf {
	type Err = InvalidNonPositiveInteger;

	fn from_str(s: &str) -> Result<Self, InvalidNonPositiveInteger> {
		Self::new(s.to_owned()).map_err(|(e, _)| e)
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

impl TryFrom<DecimalBuf> for NonPositiveIntegerBuf {
	type Error = (InvalidNonPositiveInteger, DecimalBuf);

	#[inline(always)]
	fn try_from(i: DecimalBuf) -> Result<Self, Self::Error> {
		match Self::new(i.into_string()) {
			Ok(d) => Ok(d),
			Err((e, s)) => Err((e, unsafe { DecimalBuf::new_unchecked(s) })),
		}
	}
}

impl TryFrom<FloatBuf> for NonPositiveIntegerBuf {
	type Error = (InvalidNonPositiveInteger, FloatBuf);

	#[inline(always)]
	fn try_from(i: FloatBuf) -> Result<Self, Self::Error> {
		match Self::new(i.into_string()) {
			Ok(d) => Ok(d),
			Err((e, s)) => Err((e, unsafe { FloatBuf::new_unchecked(s) })),
		}
	}
}

impl TryFrom<DoubleBuf> for NonPositiveIntegerBuf {
	type Error = (InvalidNonPositiveInteger, DoubleBuf);

	#[inline(always)]
	fn try_from(i: DoubleBuf) -> Result<Self, Self::Error> {
		match Self::new(i.into_string()) {
			Ok(d) => Ok(d),
			Err((e, s)) => Err((e, unsafe { DoubleBuf::new_unchecked(s) })),
		}
	}
}

impl Deref for NonPositiveIntegerBuf {
	type Target = NonPositiveInteger;

	#[inline(always)]
	fn deref(&self) -> &NonPositiveInteger {
		unsafe { NonPositiveInteger::new_unchecked(&self.0) }
	}
}

impl AsRef<NonPositiveInteger> for NonPositiveIntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &NonPositiveInteger {
		unsafe { NonPositiveInteger::new_unchecked(&self.0) }
	}
}

impl AsRef<Decimal> for NonPositiveIntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Decimal {
		NonPositiveInteger::as_ref(self)
	}
}

impl AsRef<Float> for NonPositiveIntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Float {
		NonPositiveInteger::as_ref(self)
	}
}

impl AsRef<Double> for NonPositiveIntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Double {
		NonPositiveInteger::as_ref(self)
	}
}

impl AsRef<[u8]> for NonPositiveIntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl AsRef<str> for NonPositiveIntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl Borrow<NonPositiveInteger> for NonPositiveIntegerBuf {
	#[inline(always)]
	fn borrow(&self) -> &NonPositiveInteger {
		unsafe { NonPositiveInteger::new_unchecked(&self.0) }
	}
}

impl fmt::Display for NonPositiveIntegerBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_str().fmt(f)
	}
}

impl fmt::Debug for NonPositiveIntegerBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl PartialEq<NonPositiveInteger> for NonPositiveIntegerBuf {
	#[inline(always)]
	fn eq(&self, other: &NonPositiveInteger) -> bool {
		self == other
	}
}

impl PartialEq<NonPositiveIntegerBuf> for NonPositiveInteger {
	#[inline(always)]
	fn eq(&self, other: &NonPositiveIntegerBuf) -> bool {
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
