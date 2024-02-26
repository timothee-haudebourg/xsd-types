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
pub use base64_binary::{Base64Binary, Base64BinaryBuf, InvalidBase64};
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
pub use hex_binary::{HexBinary, HexBinaryBuf, InvalidHex};
pub use q_name::*;
pub use string::*;
pub use time::*;

use crate::{Datatype, Value, ValueRef};

pub trait XsdValue {
	/// Returns the XSD datatype that best describes the value.
	fn datatype(&self) -> Datatype;
}

impl From<Value> for std::string::String {
	fn from(value: Value) -> Self {
		value.to_string()
	}
}

pub enum CowValue<'a> {
	Borrowed(ValueRef<'a>),
	Owned(Value),
}

impl<'a> CowValue<'a> {
	pub fn as_value_ref(&self) -> ValueRef {
		match self {
			Self::Borrowed(v) => *v,
			Self::Owned(v) => v.as_ref(),
		}
	}

	pub fn into_owned(self) -> Value {
		match self {
			Self::Borrowed(v) => v.into_owned(),
			Self::Owned(v) => v,
		}
	}
}

impl<'a> XsdValue for CowValue<'a> {
	fn datatype(&self) -> Datatype {
		match self {
			Self::Borrowed(v) => v.datatype(),
			Self::Owned(v) => v.datatype(),
		}
	}
}
