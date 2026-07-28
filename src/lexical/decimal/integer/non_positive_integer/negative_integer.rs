use crate::lexical::{
	lexical_form, Decimal, DecimalBuf, Integer, IntegerBuf, Lexical, NonNegativeInteger,
};

use super::{NonPositiveInteger, NonPositiveIntegerBuf, Overflow};
use static_automata::Validate;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use str_newtype::StrNewType;

/// Negative integer number.
///
/// `negativeInteger` has no numbered grammar production of its own in
/// the XSD 1.1 Datatypes specification: it is defined by restricting
/// [`NonPositiveInteger`]'s lexical space with a `maxInclusive` of -1.
/// See: <https://www.w3.org/TR/xmlschema11-2/#negativeInteger>.
///
/// ```abnf
/// negativeInteger = "-" *"0" NZDIGIT *DIGIT
/// ```
#[derive(Validate, StrNewType)]
#[automaton(crate::lexical::grammar::NegativeInteger)]
#[newtype(owned(NegativeIntegerBuf, derive(PartialEq, Eq)))]
pub struct NegativeInteger(str);

impl Lexical for NegativeInteger {
	type Error = InvalidNegativeInteger<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidNegativeInteger(value.to_owned()))
	}
}

lexical_form! {
	ty: NegativeInteger,
	buffer: NegativeIntegerBuf,
	value: crate::NegativeInteger,
	error: InvalidNegativeInteger,
	parent_forms: {
		as_non_positive_integer: NonPositiveInteger, NonPositiveIntegerBuf,
		as_integer: Integer, IntegerBuf,
		as_decimal: Decimal, DecimalBuf
	}
}

impl NegativeInteger {
	/// Returns the canonical form of the absolute value of `self` (without leading zeros).
	pub fn abs(&self) -> &NonNegativeInteger {
		let mut last_zero = 0;
		for (i, c) in self.0.as_bytes().iter().enumerate() {
			match c {
				b'-' => (),
				b'0' => last_zero = i,
				_ => return unsafe { NonNegativeInteger::new_unchecked(&self.0[i..]) },
			}
		}

		unsafe { NonNegativeInteger::new_unchecked(&self.0[last_zero..]) }
	}

	fn as_canonical_str(&self) -> &str {
		self.abs().as_str()
	}

	#[inline(always)]
	pub fn value(&self) -> crate::NegativeInteger {
		use num_bigint::BigInt;
		unsafe { crate::NegativeInteger::new_unchecked(BigInt::from_str(self.as_str()).unwrap()) }
	}
}

impl PartialEq for NegativeInteger {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for NegativeInteger {}

impl Hash for NegativeInteger {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for NegativeInteger {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_canonical_str()
			.cmp(other.as_canonical_str())
			.reverse()
	}
}

impl PartialOrd for NegativeInteger {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl NegativeIntegerBuf {
	pub fn minus_one() -> Self {
		unsafe { Self::new_unchecked("-1".to_string()) }
	}
}

impl PartialOrd for NegativeIntegerBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for NegativeIntegerBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for NegativeIntegerBuf {
				fn from(i: $ty) -> Self {
					unsafe { NegativeIntegerBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a NegativeInteger> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a NegativeInteger) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<NegativeIntegerBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: NegativeIntegerBuf) -> Result<Self, Overflow> {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_negative() {
		NegativeInteger::new("-1").unwrap();
		NegativeInteger::new("-042").unwrap();
	}

	#[test]
	fn parse_zero_rejected() {
		// A negative integer's value must be strictly negative: `-0` and `0`
		// are not valid `negativeInteger` lexical representations.
		NegativeInteger::new("-0").unwrap_err();
		NegativeInteger::new("0").unwrap_err();
	}

	#[test]
	fn as_non_positive_integer_preserves_value() {
		let n = NegativeInteger::new("-5").unwrap();
		assert_eq!(n.as_non_positive_integer().as_str(), "-5");
	}
}
