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
