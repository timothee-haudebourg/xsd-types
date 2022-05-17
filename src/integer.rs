use super::{Decimal, DecimalBuf, Float, FloatBuf, Overflow};
use std::borrow::{Borrow, ToOwned};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::hash::{
	Hash,
	Hasher
};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct InvalidInteger;

/// Numeric sign.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Sign {
	Negative,
	Zero,
	Positive
}

impl Sign {
	pub fn is_positive(&self) -> bool {
		matches!(self, Self::Positive)
	}

	pub fn is_negative(&self) -> bool {
		matches!(self, Self::Negative)
	}

	pub fn is_zero(&self) -> bool {
		matches!(self, Self::Zero)
	}
}

/// Integer number.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#integer>
pub struct Integer([u8]);

impl Integer {
	/// Creates a new `Integer` from a string.
	///
	/// If the input string is ot a [valid XSD integer](https://www.w3.org/TR/xmlschema-2/#integer),
	/// an [`InvalidInteger`] error is returned.
	#[inline(always)]
	pub fn new<S: ?Sized + AsRef<[u8]>>(s: &S) -> Result<&Self, InvalidInteger> {
		if check(s.as_ref().iter().cloned()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err(InvalidInteger)
		}
	}

	/// Creates a new `Integer` from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD integer](https://www.w3.org/TR/xmlschema-2/#integer).
	#[inline(always)]
	pub unsafe fn new_unchecked<S: ?Sized + AsRef<[u8]>>(s: &S) -> &Self {
		std::mem::transmute(s.as_ref())
	}

	/// Returns `true` if `self` is positive
	/// and `false` is the number is zero or negative.
	pub fn is_positive(&self) -> bool {
		let mut sign_positive = true;
		for c in &self.0 {
			match c {
				b'+' | b'0' => (),
				b'-' => sign_positive = false,
				_ => return sign_positive
			}
		}

		false
	}

	/// Returns `true` if `self` is negative
	/// and `false` is the number is zero or positive.
	pub fn is_negative(&self) -> bool {
		let mut sign_negative = true;
		for c in &self.0 {
			match c {
				b'-' | b'0' => (),
				b'+' => sign_negative = false,
				_ => return sign_negative
			}
		}

		false
	}

	/// Returns `true` if `self` is zero
	/// and `false` otherwise.
	pub fn is_zero(&self) -> bool {
		for c in &self.0 {
			if !matches!(c, b'+' | b'-' | b'0') {
				return false
			}
		}

		true
	}

	/// Returns `true` if `self` is positive or zero
	/// and `false` is negative.
	pub fn is_non_negative(&self) -> bool {
		self.0[0] != b'-'
	}

	pub fn sign(&self) -> Sign {
		let mut sign_positive = true;
		for c in &self.0 {
			match c {
				b'+' | b'0' => (),
				b'-' => sign_positive = false,
				_ => if sign_positive {
					return Sign::Positive
				} else {
					return Sign::Negative
				}
			}
		}

		Sign::Zero
	}

	/// Returns the absolute value of `self`.
	/// 
	/// The returned integer is in canonical form (without leading zeros).
	pub fn abs(&self) -> &Self {
		let mut last_zero = 0;
		for (i, c) in self.0.iter().enumerate() {
			match c {
				b'+' | b'-' => (),
				b'0' => last_zero = i,
				_ => return unsafe {
					Self::new_unchecked(&self.0[i..])
				}
			}
		}

		unsafe {
			Self::new_unchecked(&self.0[last_zero..])
		}
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
}

impl PartialEq for Integer {
	fn eq(&self, other: &Self) -> bool {
		self.sign() == other.sign() && self.abs().0 == other.abs().0
	}
}

impl Eq for Integer {}

impl Hash for Integer {
	fn hash<H: Hasher>(&self, h: &mut H) {
		match self.sign() {
			Sign::Zero => {
				0.hash(h)
			},
			sign => {
				sign.hash(h);
				self.abs().hash(h)
			}
		}
	}
}

impl Ord for Integer {
	fn cmp(&self, other: &Self) -> Ordering {
		let sign = self.sign();
		let other_sign = other.sign();
		match sign.cmp(&other_sign) {
			Ordering::Equal => {
				let a = &self.abs().0;
				let b = &other.abs().0;

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
			other => other
		}
	}
}

impl PartialOrd for Integer {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Deref for Integer {
	type Target = str;

	#[inline(always)]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

impl AsRef<[u8]> for Integer {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl AsRef<str> for Integer {
	fn as_ref(&self) -> &str {
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
		self.as_str().fmt(f)
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

impl AsRef<Float> for Integer {
	fn as_ref(&self) -> &Float {
		self.into()
	}
}

impl<'a> From<&'a IntegerBuf> for &'a Integer {
	#[inline(always)]
	fn from(b: &'a IntegerBuf) -> Self {
		b.as_ref()
	}
}

impl<'a> TryFrom<&'a Decimal> for &'a Integer {
	type Error = InvalidInteger;

	#[inline(always)]
	fn try_from(i: &'a Decimal) -> Result<Self, Self::Error> {
		Integer::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a DecimalBuf> for &'a Integer {
	type Error = InvalidInteger;

	#[inline(always)]
	fn try_from(i: &'a DecimalBuf) -> Result<Self, Self::Error> {
		Integer::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a Float> for &'a Integer {
	type Error = InvalidInteger;

	#[inline(always)]
	fn try_from(i: &'a Float) -> Result<Self, Self::Error> {
		Integer::new(i.as_str())
	}
}

impl<'a> TryFrom<&'a FloatBuf> for &'a Integer {
	type Error = InvalidInteger;

	#[inline(always)]
	fn try_from(i: &'a FloatBuf) -> Result<Self, Self::Error> {
		Integer::new(i.as_str())
	}
}

/// Owned integer number.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#integer>
#[derive(Clone)]
pub struct IntegerBuf(Vec<u8>);

impl IntegerBuf {
	/// Creates a new `IntegerBuf` from a `String`.
	///
	/// If the input string is ot a [valid XSD integer](https://www.w3.org/TR/xmlschema-2/#integer),
	/// an [`InvalidInteger`] error is returned along with the input string.
	#[inline(always)]
	pub fn new<S: AsRef<[u8]> + Into<Vec<u8>>>(s: S) -> Result<Self, (InvalidInteger, S)> {
		if check(s.as_ref().iter().cloned()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err((InvalidInteger, s))
		}
	}

	/// Creates a new `IntegerBuf` from a `String` without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD integer](https://www.w3.org/TR/xmlschema-2/#integer).
	#[inline(always)]
	pub unsafe fn new_unchecked(s: impl Into<Vec<u8>>) -> Self {
		std::mem::transmute(s.into())
	}

	pub fn zero() -> Self {
		unsafe {
			Self::new_unchecked("0".to_string())
		}
	}

	pub fn one() -> Self {
		unsafe {
			Self::new_unchecked("1".to_string())
		}
	}

	#[inline(always)]
	pub fn as_integer(&self) -> &Integer {
		self.into()
	}

	#[inline(always)]
	pub fn into_string(mut self) -> String {
		let buf = self.0.as_mut_ptr();
		let len = self.0.len();
		let capacity = self.0.capacity();
		core::mem::forget(self);
		unsafe {
			String::from_raw_parts(buf, len, capacity)
		}
	}

	fn abs_incr(&mut self) {
		let bytes = self.0.as_mut_slice();
		let mut i = bytes.len();

		while i > 0 {
			i -= 1;
			match bytes[i] {
				b'+' => {
					bytes[i] = b'1'
				}
				b'-' => {
					break
				}
				b'0'..=b'8' => {
					bytes[i] += 1;
					return
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
					break // might be zero
				}
				b'2'..=b'8' => {
					bytes[i] -= 1;
					return // non zero
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
				return // non zero
			}
		}

		// If we are here, the result is zero.
		self.0.clear();
		self.0.push(b'0');
	}

	pub fn incr(&mut self) {
		match self.sign() {
			Sign::Negative => {
				self.abs_decr()
			}
			Sign::Positive => {
				self.abs_incr()
			}
			Sign::Zero => {
				self.0.clear();
				self.0.push(b'1')
			}
		}
	}

	pub fn decr(&mut self) {
		match self.sign() {
			Sign::Negative => {
				self.abs_incr()
			}
			Sign::Positive => {
				self.abs_decr()
			}
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

	// pub fn add(&mut self, other: &Integer) {
	// 	todo!()
	// }

	// fn abs_add(&mut self, other: &Integer) {
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

	// pub fn sub(&mut self, other: &Integer) {
	// 	todo!()
	// }
}

impl Default for IntegerBuf {
	fn default() -> Self {
		Self::zero()
	}
}

impl PartialEq for IntegerBuf {
	fn eq(&self, other: &Self) -> bool {
		self.as_integer().eq(other.as_integer())
	} 
}

impl Eq for IntegerBuf {}

impl PartialOrd for IntegerBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for IntegerBuf {
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

// impl<'a> std::ops::Add<&'a Integer> for IntegerBuf {
// 	type Output = Self;

// 	fn add(mut self, rhs: &'a Integer) -> Self {
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

// impl<'a> std::ops::AddAssign<&'a Integer> for IntegerBuf {
// 	fn add_assign(&mut self, rhs: &'a Integer) {
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

// impl<'a> std::ops::Sub<&'a Integer> for IntegerBuf {
// 	type Output = Self;

// 	fn sub(mut self, rhs: &'a Integer) -> Self {
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

// impl<'a> std::ops::SubAssign<&'a Integer> for IntegerBuf {
// 	fn sub_assign(&mut self, rhs: &'a Integer) {
// 		self.sub(rhs)
// 	}
// }

impl FromStr for IntegerBuf {
	type Err = InvalidInteger;

	fn from_str(s: &str) -> Result<Self, InvalidInteger> {
		Self::new(s.to_owned()).map_err(|(e, _)| e)
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
	u128,
	i128,
	usize,
	isize
}

impl TryFrom<DecimalBuf> for IntegerBuf {
	type Error = (InvalidInteger, DecimalBuf);

	#[inline(always)]
	fn try_from(i: DecimalBuf) -> Result<Self, Self::Error> {
		match Self::new(i.into_string()) {
			Ok(d) => Ok(d),
			Err((e, s)) => Err((e, unsafe { DecimalBuf::new_unchecked(s) })),
		}
	}
}

impl TryFrom<FloatBuf> for IntegerBuf {
	type Error = (InvalidInteger, FloatBuf);

	#[inline(always)]
	fn try_from(i: FloatBuf) -> Result<Self, Self::Error> {
		match Self::new(i.into_string()) {
			Ok(d) => Ok(d),
			Err((e, s)) => Err((e, unsafe { FloatBuf::new_unchecked(s) })),
		}
	}
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

impl AsRef<Float> for IntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &Float {
		Integer::as_ref(self)
	}
}

impl AsRef<[u8]> for IntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl AsRef<str> for IntegerBuf {
	#[inline(always)]
	fn as_ref(&self) -> &str {
		self.as_str()
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
		self.as_str().fmt(f)
	}
}

impl fmt::Debug for IntegerBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
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
				Some(b'-') => State::NonEmptyInteger,
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

	#[test]
	fn abs_01() {
		assert_eq!(Integer::new("01").unwrap().abs().as_str(), "1")
	}

	#[test]
	fn abs_02() {
		assert_eq!(Integer::new("00").unwrap().abs().as_str(), "0")
	}

	#[test]
	fn abs_03() {
		assert_eq!(Integer::new("+00000").unwrap().abs().as_str(), "0")
	}

	#[test]
	fn abs_04() {
		assert_eq!(Integer::new("-00000").unwrap().abs().as_str(), "0")
	}

	#[test]
	fn abs_05() {
		assert_eq!(Integer::new("-00100").unwrap().abs().as_str(), "100")
	}

	#[test]
	fn eq_01() {
		assert_eq!(Integer::new("+001").unwrap(), Integer::new("1").unwrap())
	}

	#[test]
	fn eq_02() {
		assert_ne!(Integer::new("-001").unwrap(), Integer::new("1").unwrap())
	}

	#[test]
	fn eq_03() {
		assert_eq!(Integer::new("-000").unwrap(), Integer::new("+0").unwrap())
	}

	#[test]
	fn cmp_01() {
		assert!(Integer::new("123").unwrap() < Integer::new("456").unwrap())
	}

	#[test]
	fn cmp_02() {
		assert!(Integer::new("1230").unwrap() > Integer::new("456").unwrap())
	}

	#[test]
	fn cmp_03() {
		assert!(Integer::new("-1230").unwrap() < Integer::new("456").unwrap())
	}

	#[test]
	fn cmp_04() {
		assert!(Integer::new("-1230").unwrap() < Integer::new("-456").unwrap())
	}

	#[test]
	fn cmp_05() {
		assert!(Integer::new("-123").unwrap() > Integer::new("-456").unwrap())
	}

	#[test]
	fn cmp_06() {
		assert_eq!(Integer::new("+123456").unwrap().cmp(Integer::new("0000123456").unwrap()), Ordering::Equal)
	}

	#[test]
	fn incr_01() {
		let mut i = IntegerBuf::new("0".to_string()).unwrap();
		i.incr();
		assert_eq!(i.as_str(), "1")
	}
}

// #[cfg(feature="range-traits")]
// mod range_traits {
// 	use super::*;
// 	use ::range_traits::{
// 		Enum,
// 		MaybeBounded,
// 		Measure
// 	};

// 	impl MaybeBounded for IntegerBuf {
// 		fn min() -> Option<Self> {
// 			None
// 		}

// 		fn max() -> Option<Self> {
// 			None
// 		}
// 	}

// 	impl Enum for IntegerBuf {
// 		fn pred(&self) -> Option<Self> {
// 			let mut pred = self.clone();
// 			pred.decr();
// 			Some(pred)
// 		}

// 		fn succ(&self) -> Option<Self> {
// 			let mut succ = self.clone();
// 			succ.incr();
// 			Some(succ)
// 		}
// 	}

// 	impl Measure for IntegerBuf {
// 		type Len = Self;

// 		fn len(&self) -> Self {
// 			Self::one()
// 		}

// 		fn distance(&self, other: &Self) -> Self {
// 			if self >= other {
// 				self.clone() - other
// 			} else {
// 				other.clone() - self
// 			}
// 		}
// 	}
// }