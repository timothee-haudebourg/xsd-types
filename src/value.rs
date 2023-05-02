mod decimal;
mod double;
mod float;

pub use decimal::*;
pub use double::*;
pub use float::*;

use crate::Datatype;

pub trait XsdDatatype {
	/// Returns the XSD datatype that best describes the value.
	fn type_(&self) -> Datatype;
}

pub type String = std::string::String;

impl XsdDatatype for String {
	fn type_(&self) -> Datatype {
		Datatype::String(None)
	}
}

pub type Boolean = bool;

impl XsdDatatype for Boolean {
	fn type_(&self) -> Datatype {
		Datatype::Boolean
	}
}

#[derive(Debug, Clone)]
pub struct Duration;

impl XsdDatatype for Duration {
	fn type_(&self) -> Datatype {
		Datatype::Duration(None)
	}
}

#[derive(Debug, Clone)]
pub struct DateTime;

impl XsdDatatype for DateTime {
	fn type_(&self) -> Datatype {
		Datatype::DateTime(None)
	}
}

#[derive(Debug, Clone)]
pub struct Time;

impl XsdDatatype for Time {
	fn type_(&self) -> Datatype {
		Datatype::Time
	}
}

#[derive(Debug, Clone)]
pub struct Date;

impl XsdDatatype for Date {
	fn type_(&self) -> Datatype {
		Datatype::Date
	}
}

#[derive(Debug, Clone)]
pub struct GYearMonth;

impl XsdDatatype for GYearMonth {
	fn type_(&self) -> Datatype {
		Datatype::GYearMonth
	}
}

#[derive(Debug, Clone)]
pub struct GYear;

impl XsdDatatype for GYear {
	fn type_(&self) -> Datatype {
		Datatype::GYear
	}
}

#[derive(Debug, Clone)]
pub struct GMonthDay;

impl XsdDatatype for GMonthDay {
	fn type_(&self) -> Datatype {
		Datatype::GMonthDay
	}
}

#[derive(Debug, Clone)]
pub struct GDay;

impl XsdDatatype for GDay {
	fn type_(&self) -> Datatype {
		Datatype::GDay
	}
}

#[derive(Debug, Clone)]
pub struct GMonth;

impl XsdDatatype for GMonth {
	fn type_(&self) -> Datatype {
		Datatype::GMonth
	}
}

#[derive(Debug, Clone)]
pub struct HexBinary;

impl XsdDatatype for HexBinary {
	fn type_(&self) -> Datatype {
		Datatype::HexBinary
	}
}

#[derive(Debug, Clone)]
pub struct Base64Binary;

impl XsdDatatype for Base64Binary {
	fn type_(&self) -> Datatype {
		Datatype::Base64Binary
	}
}

#[derive(Debug, Clone)]
pub struct AnyUri;

impl XsdDatatype for AnyUri {
	fn type_(&self) -> Datatype {
		Datatype::AnyUri
	}
}

#[derive(Debug, Clone)]
pub struct QName;

impl XsdDatatype for QName {
	fn type_(&self) -> Datatype {
		Datatype::QName
	}
}

#[derive(Debug, Clone)]
pub struct Notation;

impl XsdDatatype for Notation {
	fn type_(&self) -> Datatype {
		Datatype::Notation
	}
}

/// XSD datatype value.
#[derive(Debug, Clone)]
pub enum Value {
	String(String),
	Boolean(Boolean),
	Decimal(Decimal),
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
	HexBinary(HexBinary),
	Base64Binary(Base64Binary),
	AnyUri(AnyUri),
	QName(QName),
	Notation(Notation),
}
