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
mod notation;
mod q_name;
mod string;
mod time;

use std::fmt;

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
pub use notation::*;
pub use q_name::*;
pub use string::*;
pub use time::*;

use crate::{
	Datatype, DecimalDatatype, IntDatatype, IntegerDatatype, LongDatatype,
	NonNegativeIntegerDatatype, NonPositiveIntegerDatatype, ShortDatatype, UnsignedIntDatatype,
	UnsignedLongDatatype, UnsignedShortDatatype,
};

pub trait XsdDatatype {
	/// Returns the XSD datatype that best describes the value.
	fn type_(&self) -> Datatype;
}

/// XSD datatype value.
#[derive(Debug, Clone)]
pub enum Value {
	String(String),
	Boolean(Boolean),
	Decimal(Decimal),
	Integer(Integer),
	NonPositiveInteger(NonPositiveInteger),
	NegativeInteger(NegativeInteger),
	Long(Long),
	Int(Int),
	Short(Short),
	Byte(Byte),
	NonNegativeInteger(NonNegativeInteger),
	UnsignedLong(UnsignedLong),
	UnsignedInt(UnsignedInt),
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
	PositiveInteger(PositiveInteger),
	Float(Float),
	Double(Double),
	Duration(Duration),
	DateTime(DateTime),
	Time(Time),
	Date(Date),
	GYearMonth(GYearMonth),
	GYear(GYear),
	GMonthDay(GMonthDay),
	GDay(GDay),
	GMonth(GMonth),
	HexBinary(HexBinaryBuf),
	Base64Binary(Base64BinaryBuf),
	AnyUri(AnyUriBuf),
	QName(QName),
	Notation(Notation),
}

impl Value {
	pub fn as_value_ref(&self) -> ValueRef {
		match self {
			Self::String(v) => ValueRef::String(v),
			Self::Boolean(v) => ValueRef::Boolean(*v),
			Self::Decimal(v) => ValueRef::Decimal(v),
			Self::Integer(v) => ValueRef::Integer(v),
			Self::NonPositiveInteger(v) => ValueRef::NonPositiveInteger(v),
			Self::NegativeInteger(v) => ValueRef::NegativeInteger(v),
			Self::Long(v) => ValueRef::Long(*v),
			Self::Int(v) => ValueRef::Int(*v),
			Self::Short(v) => ValueRef::Short(*v),
			Self::Byte(v) => ValueRef::Byte(*v),
			Self::NonNegativeInteger(v) => ValueRef::NonNegativeInteger(v),
			Self::UnsignedLong(v) => ValueRef::UnsignedLong(*v),
			Self::UnsignedInt(v) => ValueRef::UnsignedInt(*v),
			Self::UnsignedShort(v) => ValueRef::UnsignedShort(*v),
			Self::UnsignedByte(v) => ValueRef::UnsignedByte(*v),
			Self::PositiveInteger(v) => ValueRef::PositiveInteger(v),
			Self::Float(v) => ValueRef::Float(*v),
			Self::Double(v) => ValueRef::Double(*v),
			Self::Duration(v) => ValueRef::Duration(*v),
			Self::DateTime(v) => ValueRef::DateTime(*v),
			Self::Time(v) => ValueRef::Time(*v),
			Self::Date(v) => ValueRef::Date(*v),
			Self::GYearMonth(v) => ValueRef::GYearMonth(*v),
			Self::GYear(v) => ValueRef::GYear(*v),
			Self::GMonthDay(v) => ValueRef::GMonthDay(*v),
			Self::GDay(v) => ValueRef::GDay(*v),
			Self::GMonth(v) => ValueRef::GMonth(*v),
			Self::HexBinary(v) => ValueRef::HexBinary(v),
			Self::Base64Binary(v) => ValueRef::Base64Binary(v),
			Self::AnyUri(v) => ValueRef::AnyUri(v),
			Self::QName(v) => ValueRef::QName(v),
			Self::Notation(v) => ValueRef::Notation(v),
		}
	}
}

impl XsdDatatype for Value {
	fn type_(&self) -> Datatype {
		match self {
			Self::String(_) => Datatype::String(None),
			Self::Boolean(_) => Datatype::Boolean,
			Self::Decimal(_) => Datatype::Decimal(None),
			Self::Integer(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(None))),
			Self::NonPositiveInteger(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonPositiveInteger(None),
			)))),
			Self::NegativeInteger(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonPositiveInteger(Some(
					NonPositiveIntegerDatatype::NegativeInteger,
				)),
			)))),
			Self::Long(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::Long(None),
			)))),
			Self::Int(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::Long(Some(LongDatatype::Int(None))),
			)))),
			Self::Short(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::Long(Some(LongDatatype::Int(Some(IntDatatype::Short(None))))),
			)))),
			Self::Byte(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::Long(Some(LongDatatype::Int(Some(IntDatatype::Short(Some(
					ShortDatatype::Byte,
				)))))),
			)))),
			Self::NonNegativeInteger(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(None),
			)))),
			Self::UnsignedLong(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::UnsignedLong(None),
				)),
			)))),
			Self::UnsignedInt(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::UnsignedLong(Some(
						UnsignedLongDatatype::UnsignedInt(None),
					)),
				)),
			)))),
			Self::UnsignedShort(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::UnsignedLong(Some(
						UnsignedLongDatatype::UnsignedInt(Some(
							UnsignedIntDatatype::UnsignedShort(None),
						)),
					)),
				)),
			)))),
			Self::UnsignedByte(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::UnsignedLong(Some(
						UnsignedLongDatatype::UnsignedInt(Some(
							UnsignedIntDatatype::UnsignedShort(Some(
								UnsignedShortDatatype::UnsignedByte,
							)),
						)),
					)),
				)),
			)))),
			Self::PositiveInteger(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::PositiveInteger,
				)),
			)))),
			Self::Float(_) => Datatype::Float,
			Self::Double(_) => Datatype::Double,
			Self::Duration(_) => Datatype::Duration,
			Self::DateTime(_) => Datatype::DateTime,
			Self::Time(_) => Datatype::Time,
			Self::Date(_) => Datatype::Date,
			Self::GYearMonth(_) => Datatype::GYearMonth,
			Self::GYear(_) => Datatype::GYear,
			Self::GMonthDay(_) => Datatype::GMonthDay,
			Self::GDay(_) => Datatype::GDay,
			Self::GMonth(_) => Datatype::GMonth,
			Self::HexBinary(_) => Datatype::HexBinary,
			Self::Base64Binary(_) => Datatype::Base64Binary,
			Self::AnyUri(_) => Datatype::AnyUri,
			Self::QName(_) => Datatype::QName,
			Self::Notation(_) => Datatype::Notation,
		}
	}
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::String(v) => v.fmt(f),
			Self::Boolean(v) => v.fmt(f),
			Self::Decimal(v) => v.fmt(f),
			Self::Integer(v) => v.fmt(f),
			Self::NonPositiveInteger(v) => v.fmt(f),
			Self::NegativeInteger(v) => v.fmt(f),
			Self::Long(v) => v.fmt(f),
			Self::Int(v) => v.fmt(f),
			Self::Short(v) => v.fmt(f),
			Self::Byte(v) => v.fmt(f),
			Self::NonNegativeInteger(v) => v.fmt(f),
			Self::UnsignedLong(v) => v.fmt(f),
			Self::UnsignedInt(v) => v.fmt(f),
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
			Self::PositiveInteger(v) => v.fmt(f),
			Self::Float(v) => v.fmt(f),
			Self::Double(v) => v.fmt(f),
			Self::Duration(v) => v.fmt(f),
			Self::DateTime(v) => v.fmt(f),
			Self::Time(v) => v.fmt(f),
			Self::Date(v) => v.fmt(f),
			Self::GYearMonth(v) => v.fmt(f),
			Self::GYear(v) => v.fmt(f),
			Self::GMonthDay(v) => v.fmt(f),
			Self::GDay(v) => v.fmt(f),
			Self::GMonth(v) => v.fmt(f),
			Self::HexBinary(v) => v.fmt(f),
			Self::Base64Binary(v) => v.fmt(f),
			Self::AnyUri(v) => v.fmt(f),
			Self::QName(v) => v.fmt(f),
			Self::Notation(v) => v.fmt(f),
		}
	}
}

impl From<Value> for std::string::String {
	fn from(value: Value) -> Self {
		value.to_string()
	}
}

/// XSD datatype value.
#[derive(Debug, Clone, Copy)]
pub enum ValueRef<'a> {
	String(&'a str),
	Boolean(Boolean),
	Decimal(&'a Decimal),
	Integer(&'a Integer),
	NonPositiveInteger(&'a NonPositiveInteger),
	NegativeInteger(&'a NegativeInteger),
	Long(Long),
	Int(Int),
	Short(Short),
	Byte(Byte),
	NonNegativeInteger(&'a NonNegativeInteger),
	UnsignedLong(UnsignedLong),
	UnsignedInt(UnsignedInt),
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
	PositiveInteger(&'a PositiveInteger),
	Float(Float),
	Double(Double),
	Duration(Duration),
	DateTime(DateTime),
	Time(Time),
	Date(Date),
	GYearMonth(GYearMonth),
	GYear(GYear),
	GMonthDay(GMonthDay),
	GDay(GDay),
	GMonth(GMonth),
	HexBinary(&'a HexBinary),
	Base64Binary(&'a Base64Binary),
	AnyUri(&'a AnyUri),
	QName(&'a QName),
	Notation(&'a Notation),
}

impl<'a> ValueRef<'a> {
	pub fn into_owned(self) -> Value {
		match self {
			Self::String(v) => Value::String(v.to_owned()),
			Self::Boolean(v) => Value::Boolean(v),
			Self::Decimal(v) => Value::Decimal(v.to_owned()),
			Self::Integer(v) => Value::Integer(v.to_owned()),
			Self::NonPositiveInteger(v) => Value::NonPositiveInteger(v.to_owned()),
			Self::NegativeInteger(v) => Value::NegativeInteger(v.to_owned()),
			Self::Long(v) => Value::Long(v),
			Self::Int(v) => Value::Int(v),
			Self::Short(v) => Value::Short(v),
			Self::Byte(v) => Value::Byte(v),
			Self::NonNegativeInteger(v) => Value::NonNegativeInteger(v.to_owned()),
			Self::UnsignedLong(v) => Value::UnsignedLong(v),
			Self::UnsignedInt(v) => Value::UnsignedInt(v),
			Self::UnsignedShort(v) => Value::UnsignedShort(v),
			Self::UnsignedByte(v) => Value::UnsignedByte(v),
			Self::PositiveInteger(v) => Value::PositiveInteger(v.to_owned()),
			Self::Float(v) => Value::Float(v.to_owned()),
			Self::Double(v) => Value::Double(v.to_owned()),
			Self::Duration(v) => Value::Duration(v),
			Self::DateTime(v) => Value::DateTime(v),
			Self::Time(v) => Value::Time(v),
			Self::Date(v) => Value::Date(v),
			Self::GYearMonth(v) => Value::GYearMonth(v),
			Self::GYear(v) => Value::GYear(v),
			Self::GMonthDay(v) => Value::GMonthDay(v),
			Self::GDay(v) => Value::GDay(v),
			Self::GMonth(v) => Value::GMonth(v),
			Self::HexBinary(v) => Value::HexBinary(v.to_owned()),
			Self::Base64Binary(v) => Value::Base64Binary(v.to_owned()),
			Self::AnyUri(v) => Value::AnyUri(v.to_owned()),
			Self::QName(v) => Value::QName(v.to_owned()),
			Self::Notation(v) => Value::Notation(v.to_owned()),
		}
	}
}

impl<'a> XsdDatatype for ValueRef<'a> {
	fn type_(&self) -> Datatype {
		match self {
			Self::String(_) => Datatype::String(None),
			Self::Boolean(_) => Datatype::Boolean,
			Self::Decimal(_) => Datatype::Decimal(None),
			Self::Integer(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(None))),
			Self::NonPositiveInteger(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonPositiveInteger(None),
			)))),
			Self::NegativeInteger(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonPositiveInteger(Some(
					NonPositiveIntegerDatatype::NegativeInteger,
				)),
			)))),
			Self::Long(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::Long(None),
			)))),
			Self::Int(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::Long(Some(LongDatatype::Int(None))),
			)))),
			Self::Short(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::Long(Some(LongDatatype::Int(Some(IntDatatype::Short(None))))),
			)))),
			Self::Byte(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::Long(Some(LongDatatype::Int(Some(IntDatatype::Short(Some(
					ShortDatatype::Byte,
				)))))),
			)))),
			Self::NonNegativeInteger(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(None),
			)))),
			Self::UnsignedLong(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::UnsignedLong(None),
				)),
			)))),
			Self::UnsignedInt(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::UnsignedLong(Some(
						UnsignedLongDatatype::UnsignedInt(None),
					)),
				)),
			)))),
			Self::UnsignedShort(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::UnsignedLong(Some(
						UnsignedLongDatatype::UnsignedInt(Some(
							UnsignedIntDatatype::UnsignedShort(None),
						)),
					)),
				)),
			)))),
			Self::UnsignedByte(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::UnsignedLong(Some(
						UnsignedLongDatatype::UnsignedInt(Some(
							UnsignedIntDatatype::UnsignedShort(Some(
								UnsignedShortDatatype::UnsignedByte,
							)),
						)),
					)),
				)),
			)))),
			Self::PositiveInteger(_) => Datatype::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::PositiveInteger,
				)),
			)))),
			Self::Float(_) => Datatype::Float,
			Self::Double(_) => Datatype::Double,
			Self::Duration(_) => Datatype::Duration,
			Self::DateTime(_) => Datatype::DateTime,
			Self::Time(_) => Datatype::Time,
			Self::Date(_) => Datatype::Date,
			Self::GYearMonth(_) => Datatype::GYearMonth,
			Self::GYear(_) => Datatype::GYear,
			Self::GMonthDay(_) => Datatype::GMonthDay,
			Self::GDay(_) => Datatype::GDay,
			Self::GMonth(_) => Datatype::GMonth,
			Self::HexBinary(_) => Datatype::HexBinary,
			Self::Base64Binary(_) => Datatype::Base64Binary,
			Self::AnyUri(_) => Datatype::AnyUri,
			Self::QName(_) => Datatype::QName,
			Self::Notation(_) => Datatype::Notation,
		}
	}
}

impl<'a> fmt::Display for ValueRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::String(v) => v.fmt(f),
			Self::Boolean(v) => v.fmt(f),
			Self::Decimal(v) => v.fmt(f),
			Self::Integer(v) => v.fmt(f),
			Self::NonPositiveInteger(v) => v.fmt(f),
			Self::NegativeInteger(v) => v.fmt(f),
			Self::Long(v) => v.fmt(f),
			Self::Int(v) => v.fmt(f),
			Self::Short(v) => v.fmt(f),
			Self::Byte(v) => v.fmt(f),
			Self::NonNegativeInteger(v) => v.fmt(f),
			Self::UnsignedLong(v) => v.fmt(f),
			Self::UnsignedInt(v) => v.fmt(f),
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
			Self::PositiveInteger(v) => v.fmt(f),
			Self::Float(v) => v.fmt(f),
			Self::Double(v) => v.fmt(f),
			Self::Duration(v) => v.fmt(f),
			Self::DateTime(v) => v.fmt(f),
			Self::Time(v) => v.fmt(f),
			Self::Date(v) => v.fmt(f),
			Self::GYearMonth(v) => v.fmt(f),
			Self::GYear(v) => v.fmt(f),
			Self::GMonthDay(v) => v.fmt(f),
			Self::GDay(v) => v.fmt(f),
			Self::GMonth(v) => v.fmt(f),
			Self::HexBinary(v) => v.fmt(f),
			Self::Base64Binary(v) => v.fmt(f),
			Self::AnyUri(v) => v.fmt(f),
			Self::QName(v) => v.fmt(f),
			Self::Notation(v) => v.fmt(f),
		}
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
			Self::Owned(v) => v.as_value_ref(),
		}
	}

	pub fn into_owned(self) -> Value {
		match self {
			Self::Borrowed(v) => v.into_owned(),
			Self::Owned(v) => v,
		}
	}
}

impl<'a> XsdDatatype for CowValue<'a> {
	fn type_(&self) -> Datatype {
		match self {
			Self::Borrowed(v) => v.type_(),
			Self::Owned(v) => v.type_(),
		}
	}
}
