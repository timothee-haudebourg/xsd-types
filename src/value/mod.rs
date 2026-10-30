//! Value domain types.
mod any_uri;
pub mod base64_binary;
mod boolean;
mod date;
mod date_time;
mod decimal;
mod double;
mod duration;
mod float;
mod g_day;
mod g_month;
mod g_month_day;
mod g_year;
mod g_year_month;
pub mod hex_binary;
mod q_name;
mod string;
mod time;

pub use any_uri::*;
pub use base64_binary::{InvalidBase64, XsdBase64Binary, XsdBase64BinaryBuf};
pub use boolean::*;
pub use date::*;
pub use date_time::*;
pub use decimal::*;
pub use double::*;
pub use duration::*;
pub use float::*;
pub use g_day::*;
pub use g_month::*;
pub use g_month_day::*;
pub use g_year::*;
pub use g_year_month::*;
pub use hex_binary::{InvalidHex, XsdHexBinary, XsdHexBinaryBuf};

pub use string::*;
pub use time::*;

use crate::{Datatype, Value, ValueRef, lexical::XsdLexicalForm};

/// XSD value.
///
/// # Canonical representation
///
/// If the type implements [`Display`](std::fmt::Display), it **must** write
/// the *canonical* lexical form of the value, that is exactly the string
/// returned by [`to_lexical_form`](Self::to_lexical_form). In particular, the
/// output must always be a valid lexical form of the datatype, including for
/// special values (e.g. `INF`, `-INF` and `NaN` for `float` and `double`), and
/// must be parsed back to the same value by the type's
/// [`FromStr`](std::str::FromStr) and [`XsdFromStr`](crate::XsdFromStr)
/// implementations.
///
/// Code serializing values (e.g. to RDF literals) is allowed to rely on this
/// guarantee and use `to_string()` instead of going through the lexical form.
pub trait XsdValueDomain: XsdTyped {
	/// Lexical form of the value.
	type LexicalForm: ?Sized + XsdLexicalForm;

	/// Returns the canonical lexical form of the value.
	fn to_lexical_form(&self) -> <Self::LexicalForm as ToOwned>::Owned;
}

pub trait XsdSizedValue:
	Sized + XsdValueDomain + for<'a> TryFrom<&'a Self::LexicalForm, Error: ToString>
{
}

impl<T> XsdSizedValue for T where
	T: Sized + XsdValueDomain + for<'a> TryFrom<&'a Self::LexicalForm, Error: ToString>
{
}

pub trait XsdTyped {
	/// Returns the XSD datatype that best describes the value.
	fn datatype(&self) -> Datatype;
}

impl From<Value> for std::string::String {
	fn from(value: Value) -> Self {
		value.to_string()
	}
}

/// A [`Value`] that is either borrowed as a [`ValueRef`] or owned.
///
/// This is the value-space equivalent of [`std::borrow::Cow`], used to avoid
/// unnecessary cloning when a value is already available by reference.
pub enum CowValue<'a> {
	/// A borrowed value.
	Borrowed(ValueRef<'a>),

	/// An owned value.
	Owned(Value),
}

impl<'a> CowValue<'a> {
	/// Returns this value as a [`ValueRef`], borrowing from the owned
	/// variant if necessary.
	pub fn as_value_ref(&self) -> ValueRef<'_> {
		match self {
			Self::Borrowed(v) => *v,
			Self::Owned(v) => v.as_ref(),
		}
	}

	/// Converts `self` into an owned [`Value`], cloning it if it was
	/// borrowed.
	pub fn into_owned(self) -> Value {
		match self {
			Self::Borrowed(v) => v.into_owned(),
			Self::Owned(v) => v,
		}
	}
}

impl<'a> XsdTyped for CowValue<'a> {
	fn datatype(&self) -> Datatype {
		match self {
			Self::Borrowed(v) => v.datatype(),
			Self::Owned(v) => v.datatype(),
		}
	}
}
