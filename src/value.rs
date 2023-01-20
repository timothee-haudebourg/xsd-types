mod decimal;

pub use decimal::*;

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

pub type Float = f32;

impl XsdDatatype for Float {
	fn type_(&self) -> Datatype {
		Datatype::Float
	}
}

pub type Double = f64;

impl XsdDatatype for Double {
	fn type_(&self) -> Datatype {
		Datatype::Double
	}
}

pub struct Duration;

impl XsdDatatype for Duration {
	fn type_(&self) -> Datatype {
		Datatype::Duration(None)
	}
}

pub struct DateTime;

impl XsdDatatype for DateTime {
	fn type_(&self) -> Datatype {
		Datatype::DateTime(None)
	}
}

pub struct Time;

impl XsdDatatype for Time {
	fn type_(&self) -> Datatype {
		Datatype::Time
	}
}

pub struct Date;

impl XsdDatatype for Date {
	fn type_(&self) -> Datatype {
		Datatype::Date
	}
}

pub struct GYearMonth;

impl XsdDatatype for GYearMonth {
	fn type_(&self) -> Datatype {
		Datatype::GYearMonth
	}
}

pub struct GYear;

impl XsdDatatype for GYear {
	fn type_(&self) -> Datatype {
		Datatype::GYear
	}
}

pub struct GMonthDay;

impl XsdDatatype for GMonthDay {
	fn type_(&self) -> Datatype {
		Datatype::GMonthDay
	}
}

pub struct GDay;

impl XsdDatatype for GDay {
	fn type_(&self) -> Datatype {
		Datatype::GDay
	}
}

pub struct GMonth;

impl XsdDatatype for GMonth {
	fn type_(&self) -> Datatype {
		Datatype::GMonth
	}
}

pub struct HexBinary;

impl XsdDatatype for HexBinary {
	fn type_(&self) -> Datatype {
		Datatype::HexBinary
	}
}

pub struct Base64Binary;

impl XsdDatatype for Base64Binary {
	fn type_(&self) -> Datatype {
		Datatype::Base64Binary
	}
}

pub struct AnyUri;

impl XsdDatatype for AnyUri {
	fn type_(&self) -> Datatype {
		Datatype::AnyUri
	}
}

pub struct QName;

impl XsdDatatype for QName {
	fn type_(&self) -> Datatype {
		Datatype::QName
	}
}

pub struct Notation;

impl XsdDatatype for Notation {
	fn type_(&self) -> Datatype {
		Datatype::Notation
	}
}

/// XSD datatype value.
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
