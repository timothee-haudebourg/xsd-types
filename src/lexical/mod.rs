//! Lexical domain types.
use std::error::Error;

use static_automata::grammar;

#[grammar(
	file = "grammar.abnf",
	export(
		"dateLexicalRep" as Date,
		"dateTimeLexicalRep" as DateTime,
		"dateTimeStampLexicalRep" as DateTimeStamp,
		"timeLexicalRep" as Time,
		"gDayLexicalRep" as GDay,
		"gMonthLexicalRep" as GMonth,
		"gMonthDayLexicalRep" as GMonthDay,
		"gYearLexicalRep" as GYear,
		"gYearMonthLexicalRep" as GYearMonth,
		"durationLexicalRep" as Duration,
		"dayTimeDurationLexicalRep" as DayTimeDuration,
		"yearMonthDurationLexicalRep" as YearMonthDuration,
		"QName" as QName,
		"booleanRep" as Boolean,
		"hexBinary" as HexBinary,
		"Base64Binary" as Base64Binary,
		"decimalLexicalRep" as Decimal,
		"integer" as Integer,
		"nonNegativeInteger" as NonNegativeInteger,
		"positiveInteger" as PositiveInteger,
		"nonPositiveInteger" as NonPositiveInteger,
		"negativeInteger" as NegativeInteger,
		"floatRep" as Float,
		"floatRep" as Double
	)
)]
mod grammar {}

mod any_uri;
mod base64_binary;
mod boolean;
pub mod date;
pub mod date_time;
mod decimal;
pub mod double;
pub mod duration;
pub mod float;
pub mod g_day;
pub mod g_month;
pub mod g_month_day;
pub mod g_year;
pub mod g_year_month;
mod hex_binary;
mod q_name;
mod string;
pub mod time;

pub use base64_binary::*;
pub use boolean::*;
pub use date::{Date, DateBuf, InvalidDate};
pub(crate) use date_time::parse_timezone;
pub use date_time::{DateTime, DateTimeBuf, DateTimeStamp, DateTimeStampBuf, InvalidDateTime};
pub use decimal::*;
pub use double::{Double, DoubleBuf, InvalidDouble};
pub use duration::{
	DayTimeDuration, DayTimeDurationBuf, Duration, DurationBuf, InvalidDuration, YearMonthDuration,
	YearMonthDurationBuf,
};
pub use float::{Float, FloatBuf, InvalidFloat};
pub use g_day::{GDay, GDayBuf, InvalidGDay};
pub use g_month::{GMonth, GMonthBuf, InvalidGMonth};
pub use g_month_day::{GMonthDay, GMonthDayBuf, InvalidGMonthDay};
pub use g_year::{GYear, GYearBuf, InvalidGYear};
pub use g_year_month::{GYearMonth, GYearMonthBuf, InvalidGYearMonth};
pub use hex_binary::*;
pub use q_name::*;
pub use string::*;
pub use time::{InvalidTime, Time, TimeBuf};

/// Lexical type.
pub trait Lexical {
	type Error: Error;

	fn parse(value: &str) -> Result<&Self, Self::Error>;
}

impl Lexical for str {
	type Error = std::convert::Infallible;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Ok(value)
	}
}

pub trait LexicalFormOf<V>: Lexical {
	type ValueError: Error;

	fn try_as_value(&self) -> Result<V, Self::ValueError>;

	fn as_value(&self) -> V
	where
		Self: LexicalFormOf<V, ValueError = std::convert::Infallible>,
	{
		unsafe {
			// SAFETY: the error type is not constructible.
			self.try_as_value().unwrap_unchecked()
		}
	}
}

macro_rules! lexical_form {
	{
		ty: $ty:ident,
		buffer: $buffer_ty:ident,
		value: $value_ty:ty,
		error: $error_ty:ident,
		parent_forms: { $( $as_parent_form:ident: $parent_form:ty, $parent_buf_form:ty ),* }
	} => {
		impl $buffer_ty {
			#[inline(always)]
			pub fn into_value(self) -> $value_ty {
				self.value()
			}
		}

		$(
			impl $ty {
				#[inline(always)]
				pub fn $as_parent_form(&self) -> &$parent_form {
					self.into()
				}
			}

			impl<'a> From<&'a $ty> for &'a $parent_form {
				#[inline(always)]
				fn from(value: &'a $ty) -> Self {
					unsafe { <$parent_form>::new_unchecked(value.as_str()) }
				}
			}

			impl<'a> TryFrom<&'a $parent_form> for &'a $ty {
				type Error = $error_ty<&'a $parent_form>;

				#[inline(always)]
				fn try_from(i: &'a $parent_form) -> Result<Self, Self::Error> {
					<$ty>::new(i)
				}
			}

			impl TryFrom<$parent_buf_form> for $buffer_ty {
				type Error = $error_ty<$parent_buf_form>;

				#[inline(always)]
				fn try_from(i: $parent_buf_form) -> Result<Self, Self::Error> {
					<$buffer_ty>::new(i.into_string())
						.map_err(|e| $error_ty(unsafe { <$parent_buf_form>::new_unchecked(e.0) }))
				}
			}

			impl AsRef<$parent_form> for $ty {
				#[inline(always)]
				fn as_ref(&self) -> &$parent_form {
					self.into()
				}
			}

			impl AsRef<$parent_form> for $buffer_ty {
				#[inline(always)]
				fn as_ref(&self) -> &$parent_form {
					(**self).as_ref()
				}
			}

			impl PartialEq<$parent_form> for $ty {
				#[inline(always)]
				fn eq(&self, other: &$parent_form) -> bool {
					self.as_str() == other.as_str()
				}
			}
		)*
	};
}

pub(crate) use lexical_form;
