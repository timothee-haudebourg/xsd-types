use crate::lexical::{
	lexical_form, Decimal, DecimalBuf, Integer, IntegerBuf, Lexical, NonNegativeInteger,
};

use super::{NonNegativeIntegerBuf, Overflow};
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use str_newtype::StrNewType;

/// Positive integer number.
///
/// `positiveInteger` has no numbered grammar production of its own in
/// the XSD 1.1 Datatypes specification: it is defined by restricting
/// [`NonNegativeInteger`]'s lexical space with a `minInclusive` of 1.
/// See: <https://www.w3.org/TR/xmlschema11-2/#positiveInteger>.
///
/// ```abnf
/// positiveInteger = [ "+" ] *"0" NZDIGIT *DIGIT
/// ```
#[derive(Validate, StrNewType)]
#[automaton(crate::lexical::grammar::PositiveInteger)]
#[newtype(owned(PositiveIntegerBuf, derive(PartialEq, Eq)))]
pub struct PositiveInteger(str);

impl Lexical for PositiveInteger {
	type Error = InvalidPositiveInteger<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidPositiveInteger(value.to_owned()))
	}
}

lexical_form! {
	ty: PositiveInteger,
	buffer: PositiveIntegerBuf,
	value: crate::PositiveInteger,
	error: InvalidPositiveInteger,
	parent_forms: {
		as_non_negative_integer: NonNegativeInteger, NonNegativeIntegerBuf,
		as_integer: Integer, IntegerBuf,
		as_decimal: Decimal, DecimalBuf
	}
}

impl PositiveInteger {
	/// Returns the canonical form of `self` (without leading zeros).
	pub fn canonical(&self) -> &Self {
		let mut last_zero = 0;
		for (i, c) in self.0.as_bytes().iter().enumerate() {
			match c {
				b'+' => (),
				b'0' => last_zero = i,
				_ => return unsafe { Self::new_unchecked(&self.0[i..]) },
			}
		}

		unsafe { Self::new_unchecked(&self.0[last_zero..]) }
	}

	fn as_canonical_str(&self) -> &str {
		self.canonical().as_str()
	}

	#[inline(always)]
	pub fn value(&self) -> crate::PositiveInteger {
		use num_bigint::BigInt;
		unsafe { crate::PositiveInteger::new_unchecked(BigInt::from_str(self.as_str()).unwrap()) }
	}
}

impl PartialEq for PositiveInteger {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for PositiveInteger {}

impl Hash for PositiveInteger {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for PositiveInteger {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_canonical_str().cmp(other.as_canonical_str())
	}
}

impl PartialOrd for PositiveInteger {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PositiveIntegerBuf {
	pub fn one() -> Self {
		unsafe { Self::new_unchecked("1".to_string()) }
	}
}

impl PartialOrd for PositiveIntegerBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PositiveIntegerBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for PositiveIntegerBuf {
				fn from(i: $ty) -> Self {
					unsafe { PositiveIntegerBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a PositiveInteger> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a PositiveInteger) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<PositiveIntegerBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: PositiveIntegerBuf) -> Result<Self, Overflow> {
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
