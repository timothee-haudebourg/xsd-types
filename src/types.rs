use super::{
	AnyUri, AnyUriBuf, Base64Binary, Base64BinaryBuf, Boolean, Byte, Date, DateTime, Decimal,
	Double, Duration, Float, GDay, GMonth, GMonthDay, GYear, GYearMonth, HexBinary, HexBinaryBuf,
	Id, IdBuf, IdRef, IdRefBuf, Int, Integer, Language, LanguageBuf, Long, NCName, NCNameBuf,
	NMToken, NMTokenBuf, Name, NameBuf, NegativeInteger, NonNegativeInteger, NonPositiveInteger,
	NormalizedStr, NormalizedString, Notation, PositiveInteger, QName, Short, Time, Token,
	TokenBuf, UnsignedByte, UnsignedInt, UnsignedLong, UnsignedShort,
};
use crate::{
	ParseRdf, XsdValue, XSD_ANY_URI, XSD_BASE64_BINARY, XSD_BOOLEAN, XSD_BYTE, XSD_DATE,
	XSD_DATE_TIME, XSD_DECIMAL, XSD_DOUBLE, XSD_DURATION, XSD_FLOAT, XSD_G_DAY, XSD_G_MONTH,
	XSD_G_MONTH_DAY, XSD_G_YEAR, XSD_G_YEAR_MONTH, XSD_HEX_BINARY, XSD_ID, XSD_IDREF, XSD_INT,
	XSD_INTEGER, XSD_LANGUAGE, XSD_LONG, XSD_NAME, XSD_NC_NAME, XSD_NEGATIVE_INTEGER, XSD_NMTOKEN,
	XSD_NON_NEGATIVE_INTEGER, XSD_NON_POSITIVE_INTEGER, XSD_NORMALIZED_STRING, XSD_NOTATION,
	XSD_POSITIVE_INTEGER, XSD_Q_NAME, XSD_SHORT, XSD_STRING, XSD_TIME, XSD_TOKEN,
	XSD_UNSIGNED_BYTE, XSD_UNSIGNED_INT, XSD_UNSIGNED_LONG, XSD_UNSIGNED_SHORT,
};
use iref::Iri;
use std::fmt;
pub struct ParseError;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Datatype {
	Boolean,
	Float,
	Double,
	Decimal(DecimalDatatype),
	String(StringDatatype),
	Duration,
	DateTime,
	Time,
	Date,
	GYearMonth,
	GYear,
	GMonthDay,
	GDay,
	GMonth,
	Base64Binary,
	HexBinary,
	AnyUri,
	QName,
	Notation,
}
impl Datatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_BOOLEAN {
			return Some(Self::Boolean);
		}
		if iri == XSD_FLOAT {
			return Some(Self::Float);
		}
		if iri == XSD_DOUBLE {
			return Some(Self::Double);
		}
		if let Some(t) = DecimalDatatype::from_iri(iri) {
			return Some(Self::Decimal(t));
		}
		if let Some(t) = StringDatatype::from_iri(iri) {
			return Some(Self::String(t));
		}
		if iri == XSD_DURATION {
			return Some(Self::Duration);
		}
		if iri == XSD_DATE_TIME {
			return Some(Self::DateTime);
		}
		if iri == XSD_TIME {
			return Some(Self::Time);
		}
		if iri == XSD_DATE {
			return Some(Self::Date);
		}
		if iri == XSD_G_YEAR_MONTH {
			return Some(Self::GYearMonth);
		}
		if iri == XSD_G_YEAR {
			return Some(Self::GYear);
		}
		if iri == XSD_G_MONTH_DAY {
			return Some(Self::GMonthDay);
		}
		if iri == XSD_G_DAY {
			return Some(Self::GDay);
		}
		if iri == XSD_G_MONTH {
			return Some(Self::GMonth);
		}
		if iri == XSD_BASE64_BINARY {
			return Some(Self::Base64Binary);
		}
		if iri == XSD_HEX_BINARY {
			return Some(Self::HexBinary);
		}
		if iri == XSD_ANY_URI {
			return Some(Self::AnyUri);
		}
		if iri == XSD_Q_NAME {
			return Some(Self::QName);
		}
		if iri == XSD_NOTATION {
			return Some(Self::Notation);
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Boolean => XSD_BOOLEAN,
			Self::Float => XSD_FLOAT,
			Self::Double => XSD_DOUBLE,
			Self::Decimal(t) => t.iri(),
			Self::String(t) => t.iri(),
			Self::Duration => XSD_DURATION,
			Self::DateTime => XSD_DATE_TIME,
			Self::Time => XSD_TIME,
			Self::Date => XSD_DATE,
			Self::GYearMonth => XSD_G_YEAR_MONTH,
			Self::GYear => XSD_G_YEAR,
			Self::GMonthDay => XSD_G_MONTH_DAY,
			Self::GDay => XSD_G_DAY,
			Self::GMonth => XSD_G_MONTH,
			Self::Base64Binary => XSD_BASE64_BINARY,
			Self::HexBinary => XSD_HEX_BINARY,
			Self::AnyUri => XSD_ANY_URI,
			Self::QName => XSD_Q_NAME,
			Self::Notation => XSD_NOTATION,
		}
	}
	pub fn parse(&self, value: &str) -> Result<Value, ParseError> {
		match self {
			Self::Boolean => ParseRdf::parse_rdf(value)
				.map(Value::Boolean)
				.map_err(|_| ParseError),
			Self::Float => ParseRdf::parse_rdf(value)
				.map(Value::Float)
				.map_err(|_| ParseError),
			Self::Double => ParseRdf::parse_rdf(value)
				.map(Value::Double)
				.map_err(|_| ParseError),
			Self::Decimal(t) => t.parse(value).map(Into::into),
			Self::String(t) => t.parse(value).map(Into::into),
			// Self::Duration => ParseRdf::parse_rdf(value).map(Value::Duration).map_err(|_| ParseError),
			Self::DateTime => ParseRdf::parse_rdf(value)
				.map(Value::DateTime)
				.map_err(|_| ParseError),
			// Self::Time => ParseRdf::parse_rdf(value).map(Value::Time).map_err(|_| ParseError),
			// Self::Date => ParseRdf::parse_rdf(value).map(Value::Date).map_err(|_| ParseError),
			// Self::GYearMonth => ParseRdf::parse_rdf(value).map(Value::GYearMonth).map_err(|_| ParseError),
			// Self::GYear => ParseRdf::parse_rdf(value).map(Value::GYear).map_err(|_| ParseError),
			// Self::GMonthDay => ParseRdf::parse_rdf(value).map(Value::GMonthDay).map_err(|_| ParseError),
			// Self::GDay => ParseRdf::parse_rdf(value).map(Value::GDay).map_err(|_| ParseError),
			// Self::GMonth => ParseRdf::parse_rdf(value).map(Value::GMonth).map_err(|_| ParseError),
			Self::Base64Binary => ParseRdf::parse_rdf(value)
				.map(Value::Base64Binary)
				.map_err(|_| ParseError),
			Self::HexBinary => ParseRdf::parse_rdf(value)
				.map(Value::HexBinary)
				.map_err(|_| ParseError),
			Self::AnyUri => ParseRdf::parse_rdf(value)
				.map(Value::AnyUri)
				.map_err(|_| ParseError),
			// Self::QName => ParseRdf::parse_rdf(value).map(Value::QName).map_err(|_| ParseError),
			// Self::Notation => ParseRdf::parse_rdf(value).map(Value::Notation).map_err(|_| ParseError),
			_ => Err(ParseError), // TODO
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DecimalDatatype {
	Decimal,
	Integer(IntegerDatatype),
}
impl DecimalDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_DECIMAL {
			return Some(Self::Decimal);
		}
		if let Some(t) = IntegerDatatype::from_iri(iri) {
			return Some(Self::Integer(t));
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Decimal => XSD_DECIMAL,
			Self::Integer(t) => t.iri(),
		}
	}
	pub fn parse(&self, value: &str) -> Result<DecimalValue, ParseError> {
		match self {
			Self::Decimal => ParseRdf::parse_rdf(value)
				.map(DecimalValue::Decimal)
				.map_err(|_| ParseError),
			Self::Integer(t) => t.parse(value).map(Into::into),
		}
	}
}
impl From<IntegerDatatype> for DecimalDatatype {
	fn from(value: IntegerDatatype) -> Self {
		Self::Integer(value)
	}
}
impl From<NonPositiveIntegerDatatype> for DecimalDatatype {
	fn from(value: NonPositiveIntegerDatatype) -> Self {
		Self::Integer(IntegerDatatype::NonPositiveInteger(value))
	}
}
impl From<NonNegativeIntegerDatatype> for DecimalDatatype {
	fn from(value: NonNegativeIntegerDatatype) -> Self {
		Self::Integer(IntegerDatatype::NonNegativeInteger(value))
	}
}
impl From<UnsignedLongDatatype> for DecimalDatatype {
	fn from(value: UnsignedLongDatatype) -> Self {
		Self::Integer(IntegerDatatype::NonNegativeInteger(
			NonNegativeIntegerDatatype::UnsignedLong(value),
		))
	}
}
impl From<UnsignedIntDatatype> for DecimalDatatype {
	fn from(value: UnsignedIntDatatype) -> Self {
		Self::Integer(IntegerDatatype::NonNegativeInteger(
			NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(value)),
		))
	}
}
impl From<UnsignedShortDatatype> for DecimalDatatype {
	fn from(value: UnsignedShortDatatype) -> Self {
		Self::Integer(IntegerDatatype::NonNegativeInteger(
			NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
				UnsignedIntDatatype::UnsignedShort(value),
			)),
		))
	}
}
impl From<LongDatatype> for DecimalDatatype {
	fn from(value: LongDatatype) -> Self {
		Self::Integer(IntegerDatatype::Long(value))
	}
}
impl From<IntDatatype> for DecimalDatatype {
	fn from(value: IntDatatype) -> Self {
		Self::Integer(IntegerDatatype::Long(LongDatatype::Int(value)))
	}
}
impl From<ShortDatatype> for DecimalDatatype {
	fn from(value: ShortDatatype) -> Self {
		Self::Integer(IntegerDatatype::Long(LongDatatype::Int(
			IntDatatype::Short(value),
		)))
	}
}
impl TryFrom<DecimalDatatype> for IntegerDatatype {
	type Error = DecimalDatatype;
	fn try_from(value: DecimalDatatype) -> Result<Self, DecimalDatatype> {
		match value {
			DecimalDatatype::Integer(value) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalDatatype> for NonPositiveIntegerDatatype {
	type Error = DecimalDatatype;
	fn try_from(value: DecimalDatatype) -> Result<Self, DecimalDatatype> {
		match value {
			DecimalDatatype::Integer(IntegerDatatype::NonPositiveInteger(value)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalDatatype> for NonNegativeIntegerDatatype {
	type Error = DecimalDatatype;
	fn try_from(value: DecimalDatatype) -> Result<Self, DecimalDatatype> {
		match value {
			DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(value)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalDatatype> for UnsignedLongDatatype {
	type Error = DecimalDatatype;
	fn try_from(value: DecimalDatatype) -> Result<Self, DecimalDatatype> {
		match value {
			DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(value),
			)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalDatatype> for UnsignedIntDatatype {
	type Error = DecimalDatatype;
	fn try_from(value: DecimalDatatype) -> Result<Self, DecimalDatatype> {
		match value {
			DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(value)),
			)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalDatatype> for UnsignedShortDatatype {
	type Error = DecimalDatatype;
	fn try_from(value: DecimalDatatype) -> Result<Self, DecimalDatatype> {
		match value {
			DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedShort(value),
				)),
			)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalDatatype> for LongDatatype {
	type Error = DecimalDatatype;
	fn try_from(value: DecimalDatatype) -> Result<Self, DecimalDatatype> {
		match value {
			DecimalDatatype::Integer(IntegerDatatype::Long(value)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalDatatype> for IntDatatype {
	type Error = DecimalDatatype;
	fn try_from(value: DecimalDatatype) -> Result<Self, DecimalDatatype> {
		match value {
			DecimalDatatype::Integer(IntegerDatatype::Long(LongDatatype::Int(value))) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalDatatype> for ShortDatatype {
	type Error = DecimalDatatype;
	fn try_from(value: DecimalDatatype) -> Result<Self, DecimalDatatype> {
		match value {
			DecimalDatatype::Integer(IntegerDatatype::Long(LongDatatype::Int(
				IntDatatype::Short(value),
			))) => Ok(value),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StringDatatype {
	String,
	NormalizedString(NormalizedStringDatatype),
}
impl StringDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_STRING {
			return Some(Self::String);
		}
		if let Some(t) = NormalizedStringDatatype::from_iri(iri) {
			return Some(Self::NormalizedString(t));
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::String => XSD_STRING,
			Self::NormalizedString(t) => t.iri(),
		}
	}
	pub fn parse(&self, value: &str) -> Result<StringValue, ParseError> {
		match self {
			Self::String => ParseRdf::parse_rdf(value)
				.map(StringValue::String)
				.map_err(|_| ParseError),
			Self::NormalizedString(t) => t.parse(value).map(Into::into),
		}
	}
}
impl From<NormalizedStringDatatype> for StringDatatype {
	fn from(value: NormalizedStringDatatype) -> Self {
		Self::NormalizedString(value)
	}
}
impl From<TokenDatatype> for StringDatatype {
	fn from(value: TokenDatatype) -> Self {
		Self::NormalizedString(NormalizedStringDatatype::Token(value))
	}
}
impl From<NameDatatype> for StringDatatype {
	fn from(value: NameDatatype) -> Self {
		Self::NormalizedString(NormalizedStringDatatype::Token(TokenDatatype::Name(value)))
	}
}
impl From<NCNameDatatype> for StringDatatype {
	fn from(value: NCNameDatatype) -> Self {
		Self::NormalizedString(NormalizedStringDatatype::Token(TokenDatatype::Name(
			NameDatatype::NCName(value),
		)))
	}
}
impl TryFrom<StringDatatype> for NormalizedStringDatatype {
	type Error = StringDatatype;
	fn try_from(value: StringDatatype) -> Result<Self, StringDatatype> {
		match value {
			StringDatatype::NormalizedString(value) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<StringDatatype> for TokenDatatype {
	type Error = StringDatatype;
	fn try_from(value: StringDatatype) -> Result<Self, StringDatatype> {
		match value {
			StringDatatype::NormalizedString(NormalizedStringDatatype::Token(value)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<StringDatatype> for NameDatatype {
	type Error = StringDatatype;
	fn try_from(value: StringDatatype) -> Result<Self, StringDatatype> {
		match value {
			StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Name(value),
			)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<StringDatatype> for NCNameDatatype {
	type Error = StringDatatype;
	fn try_from(value: StringDatatype) -> Result<Self, StringDatatype> {
		match value {
			StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Name(NameDatatype::NCName(value)),
			)) => Ok(value),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone)]
pub enum Value {
	Boolean(Boolean),
	Float(Float),
	Double(Double),
	Decimal(Decimal),
	Integer(Integer),
	NonPositiveInteger(NonPositiveInteger),
	NegativeInteger(NegativeInteger),
	NonNegativeInteger(NonNegativeInteger),
	PositiveInteger(PositiveInteger),
	UnsignedLong(UnsignedLong),
	UnsignedInt(UnsignedInt),
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
	Long(Long),
	Int(Int),
	Short(Short),
	Byte(Byte),
	String(String),
	NormalizedString(NormalizedString),
	Token(TokenBuf),
	Language(LanguageBuf),
	Name(NameBuf),
	NCName(NCNameBuf),
	Id(IdBuf),
	IdRef(IdRefBuf),
	NMToken(NMTokenBuf),
	Duration(Duration),
	DateTime(DateTime),
	Time(Time),
	Date(Date),
	GYearMonth(GYearMonth),
	GYear(GYear),
	GMonthDay(GMonthDay),
	GDay(GDay),
	GMonth(GMonth),
	Base64Binary(Base64BinaryBuf),
	HexBinary(HexBinaryBuf),
	AnyUri(AnyUriBuf),
	QName(QName),
	Notation(Notation),
}
impl Value {
	pub fn datatype(&self) -> Datatype {
		match self {
			Self::Boolean(_) => Datatype::Boolean,
			Self::Float(_) => Datatype::Float,
			Self::Double(_) => Datatype::Double,
			Self::Decimal(_) => Datatype::Decimal(DecimalDatatype::Decimal),
			Self::Integer(_) => {
				Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Integer))
			}
			Self::NonPositiveInteger(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonPositiveInteger(NonPositiveIntegerDatatype::NonPositiveInteger),
			)),
			Self::NegativeInteger(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonPositiveInteger(NonPositiveIntegerDatatype::NegativeInteger),
			)),
			Self::NonNegativeInteger(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::NonNegativeInteger),
			)),
			Self::PositiveInteger(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::PositiveInteger),
			)),
			Self::UnsignedLong(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
					UnsignedLongDatatype::UnsignedLong,
				)),
			)),
			Self::UnsignedInt(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
					UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedInt),
				)),
			)),
			Self::UnsignedShort(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
					UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedShort(
						UnsignedShortDatatype::UnsignedShort,
					)),
				)),
			)),
			Self::UnsignedByte(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
					UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedShort(
						UnsignedShortDatatype::UnsignedByte,
					)),
				)),
			)),
			Self::Long(_) => Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(
				LongDatatype::Long,
			))),
			Self::Int(_) => Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(
				LongDatatype::Int(IntDatatype::Int),
			))),
			Self::Short(_) => Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(
				LongDatatype::Int(IntDatatype::Short(ShortDatatype::Short)),
			))),
			Self::Byte(_) => Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(
				LongDatatype::Int(IntDatatype::Short(ShortDatatype::Byte)),
			))),
			Self::String(_) => Datatype::String(StringDatatype::String),
			Self::NormalizedString(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::NormalizedString,
			)),
			Self::Token(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Token),
			)),
			Self::Language(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Language),
			)),
			Self::Name(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::Name)),
			)),
			Self::NCName(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::NCName(
					NCNameDatatype::NCName,
				))),
			)),
			Self::Id(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::NCName(
					NCNameDatatype::Id,
				))),
			)),
			Self::IdRef(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::NCName(
					NCNameDatatype::IdRef,
				))),
			)),
			Self::NMToken(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::NMToken),
			)),
			Self::Duration(_) => Datatype::Duration,
			Self::DateTime(_) => Datatype::DateTime,
			Self::Time(_) => Datatype::Time,
			Self::Date(_) => Datatype::Date,
			Self::GYearMonth(_) => Datatype::GYearMonth,
			Self::GYear(_) => Datatype::GYear,
			Self::GMonthDay(_) => Datatype::GMonthDay,
			Self::GDay(_) => Datatype::GDay,
			Self::GMonth(_) => Datatype::GMonth,
			Self::Base64Binary(_) => Datatype::Base64Binary,
			Self::HexBinary(_) => Datatype::HexBinary,
			Self::AnyUri(_) => Datatype::AnyUri,
			Self::QName(_) => Datatype::QName,
			Self::Notation(_) => Datatype::Notation,
		}
	}
}
impl XsdValue for Value {
	fn datatype(&self) -> Datatype {
		self.datatype()
	}
}
impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Boolean(v) => v.fmt(f),
			Self::Float(v) => v.fmt(f),
			Self::Double(v) => v.fmt(f),
			Self::Decimal(v) => v.fmt(f),
			Self::Integer(v) => v.fmt(f),
			Self::NonPositiveInteger(v) => v.fmt(f),
			Self::NegativeInteger(v) => v.fmt(f),
			Self::NonNegativeInteger(v) => v.fmt(f),
			Self::PositiveInteger(v) => v.fmt(f),
			Self::UnsignedLong(v) => v.fmt(f),
			Self::UnsignedInt(v) => v.fmt(f),
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
			Self::Long(v) => v.fmt(f),
			Self::Int(v) => v.fmt(f),
			Self::Short(v) => v.fmt(f),
			Self::Byte(v) => v.fmt(f),
			Self::String(v) => v.fmt(f),
			Self::NormalizedString(v) => v.fmt(f),
			Self::Token(v) => v.fmt(f),
			Self::Language(v) => v.fmt(f),
			Self::Name(v) => v.fmt(f),
			Self::NCName(v) => v.fmt(f),
			Self::Id(v) => v.fmt(f),
			Self::IdRef(v) => v.fmt(f),
			Self::NMToken(v) => v.fmt(f),
			Self::Duration(v) => v.fmt(f),
			Self::DateTime(v) => v.fmt(f),
			Self::Time(v) => v.fmt(f),
			Self::Date(v) => v.fmt(f),
			Self::GYearMonth(v) => v.fmt(f),
			Self::GYear(v) => v.fmt(f),
			Self::GMonthDay(v) => v.fmt(f),
			Self::GDay(v) => v.fmt(f),
			Self::GMonth(v) => v.fmt(f),
			Self::Base64Binary(v) => v.fmt(f),
			Self::HexBinary(v) => v.fmt(f),
			Self::AnyUri(v) => v.fmt(f),
			Self::QName(v) => v.fmt(f),
			Self::Notation(v) => v.fmt(f),
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub enum ValueRef<'a> {
	Boolean(Boolean),
	Float(Float),
	Double(Double),
	Decimal(&'a Decimal),
	Integer(&'a Integer),
	NonPositiveInteger(&'a NonPositiveInteger),
	NegativeInteger(&'a NegativeInteger),
	NonNegativeInteger(&'a NonNegativeInteger),
	PositiveInteger(&'a PositiveInteger),
	UnsignedLong(UnsignedLong),
	UnsignedInt(UnsignedInt),
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
	Long(Long),
	Int(Int),
	Short(Short),
	Byte(Byte),
	String(&'a str),
	NormalizedString(&'a NormalizedStr),
	Token(&'a Token),
	Language(&'a Language),
	Name(&'a Name),
	NCName(&'a NCName),
	Id(&'a Id),
	IdRef(&'a IdRef),
	NMToken(&'a NMToken),
	Duration(Duration),
	DateTime(DateTime),
	Time(Time),
	Date(Date),
	GYearMonth(GYearMonth),
	GYear(GYear),
	GMonthDay(GMonthDay),
	GDay(GDay),
	GMonth(GMonth),
	Base64Binary(&'a Base64Binary),
	HexBinary(&'a HexBinary),
	AnyUri(&'a AnyUri),
	QName(&'a QName),
	Notation(&'a Notation),
}
impl<'a> fmt::Display for ValueRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Boolean(v) => v.fmt(f),
			Self::Float(v) => v.fmt(f),
			Self::Double(v) => v.fmt(f),
			Self::Decimal(v) => v.fmt(f),
			Self::Integer(v) => v.fmt(f),
			Self::NonPositiveInteger(v) => v.fmt(f),
			Self::NegativeInteger(v) => v.fmt(f),
			Self::NonNegativeInteger(v) => v.fmt(f),
			Self::PositiveInteger(v) => v.fmt(f),
			Self::UnsignedLong(v) => v.fmt(f),
			Self::UnsignedInt(v) => v.fmt(f),
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
			Self::Long(v) => v.fmt(f),
			Self::Int(v) => v.fmt(f),
			Self::Short(v) => v.fmt(f),
			Self::Byte(v) => v.fmt(f),
			Self::String(v) => v.fmt(f),
			Self::NormalizedString(v) => v.fmt(f),
			Self::Token(v) => v.fmt(f),
			Self::Language(v) => v.fmt(f),
			Self::Name(v) => v.fmt(f),
			Self::NCName(v) => v.fmt(f),
			Self::Id(v) => v.fmt(f),
			Self::IdRef(v) => v.fmt(f),
			Self::NMToken(v) => v.fmt(f),
			Self::Duration(v) => v.fmt(f),
			Self::DateTime(v) => v.fmt(f),
			Self::Time(v) => v.fmt(f),
			Self::Date(v) => v.fmt(f),
			Self::GYearMonth(v) => v.fmt(f),
			Self::GYear(v) => v.fmt(f),
			Self::GMonthDay(v) => v.fmt(f),
			Self::GDay(v) => v.fmt(f),
			Self::GMonth(v) => v.fmt(f),
			Self::Base64Binary(v) => v.fmt(f),
			Self::HexBinary(v) => v.fmt(f),
			Self::AnyUri(v) => v.fmt(f),
			Self::QName(v) => v.fmt(f),
			Self::Notation(v) => v.fmt(f),
		}
	}
}
impl Value {
	pub fn as_ref(&self) -> ValueRef {
		match self {
			Self::Boolean(value) => ValueRef::Boolean(*value),
			Self::Float(value) => ValueRef::Float(*value),
			Self::Double(value) => ValueRef::Double(*value),
			Self::Decimal(value) => ValueRef::Decimal(value),
			Self::Integer(value) => ValueRef::Integer(value),
			Self::NonPositiveInteger(value) => ValueRef::NonPositiveInteger(value),
			Self::NegativeInteger(value) => ValueRef::NegativeInteger(value),
			Self::NonNegativeInteger(value) => ValueRef::NonNegativeInteger(value),
			Self::PositiveInteger(value) => ValueRef::PositiveInteger(value),
			Self::UnsignedLong(value) => ValueRef::UnsignedLong(*value),
			Self::UnsignedInt(value) => ValueRef::UnsignedInt(*value),
			Self::UnsignedShort(value) => ValueRef::UnsignedShort(*value),
			Self::UnsignedByte(value) => ValueRef::UnsignedByte(*value),
			Self::Long(value) => ValueRef::Long(*value),
			Self::Int(value) => ValueRef::Int(*value),
			Self::Short(value) => ValueRef::Short(*value),
			Self::Byte(value) => ValueRef::Byte(*value),
			Self::String(value) => ValueRef::String(value),
			Self::NormalizedString(value) => ValueRef::NormalizedString(value),
			Self::Token(value) => ValueRef::Token(value),
			Self::Language(value) => ValueRef::Language(value),
			Self::Name(value) => ValueRef::Name(value),
			Self::NCName(value) => ValueRef::NCName(value),
			Self::Id(value) => ValueRef::Id(value),
			Self::IdRef(value) => ValueRef::IdRef(value),
			Self::NMToken(value) => ValueRef::NMToken(value),
			Self::Duration(value) => ValueRef::Duration(*value),
			Self::DateTime(value) => ValueRef::DateTime(*value),
			Self::Time(value) => ValueRef::Time(*value),
			Self::Date(value) => ValueRef::Date(*value),
			Self::GYearMonth(value) => ValueRef::GYearMonth(*value),
			Self::GYear(value) => ValueRef::GYear(*value),
			Self::GMonthDay(value) => ValueRef::GMonthDay(*value),
			Self::GDay(value) => ValueRef::GDay(*value),
			Self::GMonth(value) => ValueRef::GMonth(*value),
			Self::Base64Binary(value) => ValueRef::Base64Binary(value),
			Self::HexBinary(value) => ValueRef::HexBinary(value),
			Self::AnyUri(value) => ValueRef::AnyUri(value),
			Self::QName(value) => ValueRef::QName(value),
			Self::Notation(value) => ValueRef::Notation(value),
		}
	}
}
impl<'a> ValueRef<'a> {
	pub fn datatype(&self) -> Datatype {
		match self {
			Self::Boolean(_) => Datatype::Boolean,
			Self::Float(_) => Datatype::Float,
			Self::Double(_) => Datatype::Double,
			Self::Decimal(_) => Datatype::Decimal(DecimalDatatype::Decimal),
			Self::Integer(_) => {
				Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Integer))
			}
			Self::NonPositiveInteger(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonPositiveInteger(NonPositiveIntegerDatatype::NonPositiveInteger),
			)),
			Self::NegativeInteger(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonPositiveInteger(NonPositiveIntegerDatatype::NegativeInteger),
			)),
			Self::NonNegativeInteger(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::NonNegativeInteger),
			)),
			Self::PositiveInteger(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::PositiveInteger),
			)),
			Self::UnsignedLong(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
					UnsignedLongDatatype::UnsignedLong,
				)),
			)),
			Self::UnsignedInt(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
					UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedInt),
				)),
			)),
			Self::UnsignedShort(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
					UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedShort(
						UnsignedShortDatatype::UnsignedShort,
					)),
				)),
			)),
			Self::UnsignedByte(_) => Datatype::Decimal(DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
					UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedShort(
						UnsignedShortDatatype::UnsignedByte,
					)),
				)),
			)),
			Self::Long(_) => Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(
				LongDatatype::Long,
			))),
			Self::Int(_) => Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(
				LongDatatype::Int(IntDatatype::Int),
			))),
			Self::Short(_) => Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(
				LongDatatype::Int(IntDatatype::Short(ShortDatatype::Short)),
			))),
			Self::Byte(_) => Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(
				LongDatatype::Int(IntDatatype::Short(ShortDatatype::Byte)),
			))),
			Self::String(_) => Datatype::String(StringDatatype::String),
			Self::NormalizedString(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::NormalizedString,
			)),
			Self::Token(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Token),
			)),
			Self::Language(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Language),
			)),
			Self::Name(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::Name)),
			)),
			Self::NCName(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::NCName(
					NCNameDatatype::NCName,
				))),
			)),
			Self::Id(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::NCName(
					NCNameDatatype::Id,
				))),
			)),
			Self::IdRef(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::NCName(
					NCNameDatatype::IdRef,
				))),
			)),
			Self::NMToken(_) => Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::NMToken),
			)),
			Self::Duration(_) => Datatype::Duration,
			Self::DateTime(_) => Datatype::DateTime,
			Self::Time(_) => Datatype::Time,
			Self::Date(_) => Datatype::Date,
			Self::GYearMonth(_) => Datatype::GYearMonth,
			Self::GYear(_) => Datatype::GYear,
			Self::GMonthDay(_) => Datatype::GMonthDay,
			Self::GDay(_) => Datatype::GDay,
			Self::GMonth(_) => Datatype::GMonth,
			Self::Base64Binary(_) => Datatype::Base64Binary,
			Self::HexBinary(_) => Datatype::HexBinary,
			Self::AnyUri(_) => Datatype::AnyUri,
			Self::QName(_) => Datatype::QName,
			Self::Notation(_) => Datatype::Notation,
		}
	}
	pub fn into_owned(self) -> Value {
		match self {
			Self::Boolean(value) => Value::Boolean(value),
			Self::Float(value) => Value::Float(value),
			Self::Double(value) => Value::Double(value),
			Self::Decimal(value) => Value::Decimal(value.to_owned()),
			Self::Integer(value) => Value::Integer(value.to_owned()),
			Self::NonPositiveInteger(value) => Value::NonPositiveInteger(value.to_owned()),
			Self::NegativeInteger(value) => Value::NegativeInteger(value.to_owned()),
			Self::NonNegativeInteger(value) => Value::NonNegativeInteger(value.to_owned()),
			Self::PositiveInteger(value) => Value::PositiveInteger(value.to_owned()),
			Self::UnsignedLong(value) => Value::UnsignedLong(value),
			Self::UnsignedInt(value) => Value::UnsignedInt(value),
			Self::UnsignedShort(value) => Value::UnsignedShort(value),
			Self::UnsignedByte(value) => Value::UnsignedByte(value),
			Self::Long(value) => Value::Long(value),
			Self::Int(value) => Value::Int(value),
			Self::Short(value) => Value::Short(value),
			Self::Byte(value) => Value::Byte(value),
			Self::String(value) => Value::String(value.to_owned()),
			Self::NormalizedString(value) => Value::NormalizedString(value.to_owned()),
			Self::Token(value) => Value::Token(value.to_owned()),
			Self::Language(value) => Value::Language(value.to_owned()),
			Self::Name(value) => Value::Name(value.to_owned()),
			Self::NCName(value) => Value::NCName(value.to_owned()),
			Self::Id(value) => Value::Id(value.to_owned()),
			Self::IdRef(value) => Value::IdRef(value.to_owned()),
			Self::NMToken(value) => Value::NMToken(value.to_owned()),
			Self::Duration(value) => Value::Duration(value),
			Self::DateTime(value) => Value::DateTime(value),
			Self::Time(value) => Value::Time(value),
			Self::Date(value) => Value::Date(value),
			Self::GYearMonth(value) => Value::GYearMonth(value),
			Self::GYear(value) => Value::GYear(value),
			Self::GMonthDay(value) => Value::GMonthDay(value),
			Self::GDay(value) => Value::GDay(value),
			Self::GMonth(value) => Value::GMonth(value),
			Self::Base64Binary(value) => Value::Base64Binary(value.to_owned()),
			Self::HexBinary(value) => Value::HexBinary(value.to_owned()),
			Self::AnyUri(value) => Value::AnyUri(value.to_owned()),
			Self::QName(value) => Value::QName(value.to_owned()),
			Self::Notation(value) => Value::Notation(value.to_owned()),
		}
	}
	pub fn cloned(&self) -> Value {
		self.into_owned()
	}
}
impl<'a> XsdValue for ValueRef<'a> {
	fn datatype(&self) -> Datatype {
		self.datatype()
	}
}
impl From<DecimalDatatype> for Datatype {
	fn from(value: DecimalDatatype) -> Self {
		Self::Decimal(value)
	}
}
impl From<IntegerDatatype> for Datatype {
	fn from(value: IntegerDatatype) -> Self {
		Self::Decimal(DecimalDatatype::Integer(value))
	}
}
impl From<NonPositiveIntegerDatatype> for Datatype {
	fn from(value: NonPositiveIntegerDatatype) -> Self {
		Self::Decimal(DecimalDatatype::Integer(
			IntegerDatatype::NonPositiveInteger(value),
		))
	}
}
impl From<NonNegativeIntegerDatatype> for Datatype {
	fn from(value: NonNegativeIntegerDatatype) -> Self {
		Self::Decimal(DecimalDatatype::Integer(
			IntegerDatatype::NonNegativeInteger(value),
		))
	}
}
impl From<UnsignedLongDatatype> for Datatype {
	fn from(value: UnsignedLongDatatype) -> Self {
		Self::Decimal(DecimalDatatype::Integer(
			IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(value)),
		))
	}
}
impl From<UnsignedIntDatatype> for Datatype {
	fn from(value: UnsignedIntDatatype) -> Self {
		Self::Decimal(DecimalDatatype::Integer(
			IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
				UnsignedLongDatatype::UnsignedInt(value),
			)),
		))
	}
}
impl From<UnsignedShortDatatype> for Datatype {
	fn from(value: UnsignedShortDatatype) -> Self {
		Self::Decimal(DecimalDatatype::Integer(
			IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
				UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedShort(value)),
			)),
		))
	}
}
impl From<LongDatatype> for Datatype {
	fn from(value: LongDatatype) -> Self {
		Self::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(value)))
	}
}
impl From<IntDatatype> for Datatype {
	fn from(value: IntDatatype) -> Self {
		Self::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(
			LongDatatype::Int(value),
		)))
	}
}
impl From<ShortDatatype> for Datatype {
	fn from(value: ShortDatatype) -> Self {
		Self::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(
			LongDatatype::Int(IntDatatype::Short(value)),
		)))
	}
}
impl TryFrom<Datatype> for DecimalDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::Decimal(value) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for IntegerDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::Decimal(DecimalDatatype::Integer(value)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for NonPositiveIntegerDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::NonPositiveInteger(
				value,
			))) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for NonNegativeIntegerDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				value,
			))) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for UnsignedLongDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(value),
			))) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for UnsignedIntDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(value)),
			))) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for UnsignedShortDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedShort(value),
				)),
			))) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for LongDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(value))) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for IntDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(
				LongDatatype::Int(value),
			))) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for ShortDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::Decimal(DecimalDatatype::Integer(IntegerDatatype::Long(
				LongDatatype::Int(IntDatatype::Short(value)),
			))) => Ok(value),
			other => Err(other),
		}
	}
}
impl From<DecimalValue> for Value {
	fn from(value: DecimalValue) -> Self {
		match value {
			DecimalValue::Decimal(value) => Self::Decimal(value),
			DecimalValue::Integer(value) => Self::Integer(value),
			DecimalValue::NonPositiveInteger(value) => Self::NonPositiveInteger(value),
			DecimalValue::NegativeInteger(value) => Self::NegativeInteger(value),
			DecimalValue::NonNegativeInteger(value) => Self::NonNegativeInteger(value),
			DecimalValue::PositiveInteger(value) => Self::PositiveInteger(value),
			DecimalValue::UnsignedLong(value) => Self::UnsignedLong(value),
			DecimalValue::UnsignedInt(value) => Self::UnsignedInt(value),
			DecimalValue::UnsignedShort(value) => Self::UnsignedShort(value),
			DecimalValue::UnsignedByte(value) => Self::UnsignedByte(value),
			DecimalValue::Long(value) => Self::Long(value),
			DecimalValue::Int(value) => Self::Int(value),
			DecimalValue::Short(value) => Self::Short(value),
			DecimalValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl From<IntegerValue> for Value {
	fn from(value: IntegerValue) -> Self {
		match value {
			IntegerValue::Integer(value) => Self::Integer(value),
			IntegerValue::NonPositiveInteger(value) => Self::NonPositiveInteger(value),
			IntegerValue::NegativeInteger(value) => Self::NegativeInteger(value),
			IntegerValue::NonNegativeInteger(value) => Self::NonNegativeInteger(value),
			IntegerValue::PositiveInteger(value) => Self::PositiveInteger(value),
			IntegerValue::UnsignedLong(value) => Self::UnsignedLong(value),
			IntegerValue::UnsignedInt(value) => Self::UnsignedInt(value),
			IntegerValue::UnsignedShort(value) => Self::UnsignedShort(value),
			IntegerValue::UnsignedByte(value) => Self::UnsignedByte(value),
			IntegerValue::Long(value) => Self::Long(value),
			IntegerValue::Int(value) => Self::Int(value),
			IntegerValue::Short(value) => Self::Short(value),
			IntegerValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl From<NonPositiveIntegerValue> for Value {
	fn from(value: NonPositiveIntegerValue) -> Self {
		match value {
			NonPositiveIntegerValue::NonPositiveInteger(value) => Self::NonPositiveInteger(value),
			NonPositiveIntegerValue::NegativeInteger(value) => Self::NegativeInteger(value),
		}
	}
}
impl From<NonNegativeIntegerValue> for Value {
	fn from(value: NonNegativeIntegerValue) -> Self {
		match value {
			NonNegativeIntegerValue::NonNegativeInteger(value) => Self::NonNegativeInteger(value),
			NonNegativeIntegerValue::PositiveInteger(value) => Self::PositiveInteger(value),
			NonNegativeIntegerValue::UnsignedLong(value) => Self::UnsignedLong(value),
			NonNegativeIntegerValue::UnsignedInt(value) => Self::UnsignedInt(value),
			NonNegativeIntegerValue::UnsignedShort(value) => Self::UnsignedShort(value),
			NonNegativeIntegerValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<UnsignedLongValue> for Value {
	fn from(value: UnsignedLongValue) -> Self {
		match value {
			UnsignedLongValue::UnsignedLong(value) => Self::UnsignedLong(value),
			UnsignedLongValue::UnsignedInt(value) => Self::UnsignedInt(value),
			UnsignedLongValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedLongValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<UnsignedIntValue> for Value {
	fn from(value: UnsignedIntValue) -> Self {
		match value {
			UnsignedIntValue::UnsignedInt(value) => Self::UnsignedInt(value),
			UnsignedIntValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedIntValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<UnsignedShortValue> for Value {
	fn from(value: UnsignedShortValue) -> Self {
		match value {
			UnsignedShortValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedShortValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<LongValue> for Value {
	fn from(value: LongValue) -> Self {
		match value {
			LongValue::Long(value) => Self::Long(value),
			LongValue::Int(value) => Self::Int(value),
			LongValue::Short(value) => Self::Short(value),
			LongValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl From<IntValue> for Value {
	fn from(value: IntValue) -> Self {
		match value {
			IntValue::Int(value) => Self::Int(value),
			IntValue::Short(value) => Self::Short(value),
			IntValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl From<ShortValue> for Value {
	fn from(value: ShortValue) -> Self {
		match value {
			ShortValue::Short(value) => Self::Short(value),
			ShortValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl TryFrom<Value> for DecimalValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::Decimal(value) => Ok(Self::Decimal(value)),
			Value::Integer(value) => Ok(Self::Integer(value)),
			Value::NonPositiveInteger(value) => Ok(Self::NonPositiveInteger(value)),
			Value::NegativeInteger(value) => Ok(Self::NegativeInteger(value)),
			Value::NonNegativeInteger(value) => Ok(Self::NonNegativeInteger(value)),
			Value::PositiveInteger(value) => Ok(Self::PositiveInteger(value)),
			Value::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			Value::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			Value::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			Value::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			Value::Long(value) => Ok(Self::Long(value)),
			Value::Int(value) => Ok(Self::Int(value)),
			Value::Short(value) => Ok(Self::Short(value)),
			Value::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for IntegerValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::Integer(value) => Ok(Self::Integer(value)),
			Value::NonPositiveInteger(value) => Ok(Self::NonPositiveInteger(value)),
			Value::NegativeInteger(value) => Ok(Self::NegativeInteger(value)),
			Value::NonNegativeInteger(value) => Ok(Self::NonNegativeInteger(value)),
			Value::PositiveInteger(value) => Ok(Self::PositiveInteger(value)),
			Value::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			Value::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			Value::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			Value::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			Value::Long(value) => Ok(Self::Long(value)),
			Value::Int(value) => Ok(Self::Int(value)),
			Value::Short(value) => Ok(Self::Short(value)),
			Value::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for NonPositiveIntegerValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::NonPositiveInteger(value) => Ok(Self::NonPositiveInteger(value)),
			Value::NegativeInteger(value) => Ok(Self::NegativeInteger(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for NonNegativeIntegerValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::NonNegativeInteger(value) => Ok(Self::NonNegativeInteger(value)),
			Value::PositiveInteger(value) => Ok(Self::PositiveInteger(value)),
			Value::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			Value::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			Value::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			Value::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for UnsignedLongValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			Value::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			Value::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			Value::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for UnsignedIntValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			Value::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			Value::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for UnsignedShortValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			Value::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for LongValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::Long(value) => Ok(Self::Long(value)),
			Value::Int(value) => Ok(Self::Int(value)),
			Value::Short(value) => Ok(Self::Short(value)),
			Value::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for IntValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::Int(value) => Ok(Self::Int(value)),
			Value::Short(value) => Ok(Self::Short(value)),
			Value::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for ShortValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::Short(value) => Ok(Self::Short(value)),
			Value::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl<'a> From<DecimalValueRef<'a>> for ValueRef<'a> {
	fn from(value: DecimalValueRef<'a>) -> Self {
		match value {
			DecimalValueRef::Decimal(value) => Self::Decimal(value),
			DecimalValueRef::Integer(value) => Self::Integer(value),
			DecimalValueRef::NonPositiveInteger(value) => Self::NonPositiveInteger(value),
			DecimalValueRef::NegativeInteger(value) => Self::NegativeInteger(value),
			DecimalValueRef::NonNegativeInteger(value) => Self::NonNegativeInteger(value),
			DecimalValueRef::PositiveInteger(value) => Self::PositiveInteger(value),
			DecimalValueRef::UnsignedLong(value) => Self::UnsignedLong(value),
			DecimalValueRef::UnsignedInt(value) => Self::UnsignedInt(value),
			DecimalValueRef::UnsignedShort(value) => Self::UnsignedShort(value),
			DecimalValueRef::UnsignedByte(value) => Self::UnsignedByte(value),
			DecimalValueRef::Long(value) => Self::Long(value),
			DecimalValueRef::Int(value) => Self::Int(value),
			DecimalValueRef::Short(value) => Self::Short(value),
			DecimalValueRef::Byte(value) => Self::Byte(value),
		}
	}
}
impl<'a> From<IntegerValueRef<'a>> for ValueRef<'a> {
	fn from(value: IntegerValueRef<'a>) -> Self {
		match value {
			IntegerValueRef::Integer(value) => Self::Integer(value),
			IntegerValueRef::NonPositiveInteger(value) => Self::NonPositiveInteger(value),
			IntegerValueRef::NegativeInteger(value) => Self::NegativeInteger(value),
			IntegerValueRef::NonNegativeInteger(value) => Self::NonNegativeInteger(value),
			IntegerValueRef::PositiveInteger(value) => Self::PositiveInteger(value),
			IntegerValueRef::UnsignedLong(value) => Self::UnsignedLong(value),
			IntegerValueRef::UnsignedInt(value) => Self::UnsignedInt(value),
			IntegerValueRef::UnsignedShort(value) => Self::UnsignedShort(value),
			IntegerValueRef::UnsignedByte(value) => Self::UnsignedByte(value),
			IntegerValueRef::Long(value) => Self::Long(value),
			IntegerValueRef::Int(value) => Self::Int(value),
			IntegerValueRef::Short(value) => Self::Short(value),
			IntegerValueRef::Byte(value) => Self::Byte(value),
		}
	}
}
impl<'a> From<NonPositiveIntegerValueRef<'a>> for ValueRef<'a> {
	fn from(value: NonPositiveIntegerValueRef<'a>) -> Self {
		match value {
			NonPositiveIntegerValueRef::NonPositiveInteger(value) => {
				Self::NonPositiveInteger(value)
			}
			NonPositiveIntegerValueRef::NegativeInteger(value) => Self::NegativeInteger(value),
		}
	}
}
impl<'a> From<NonNegativeIntegerValueRef<'a>> for ValueRef<'a> {
	fn from(value: NonNegativeIntegerValueRef<'a>) -> Self {
		match value {
			NonNegativeIntegerValueRef::NonNegativeInteger(value) => {
				Self::NonNegativeInteger(value)
			}
			NonNegativeIntegerValueRef::PositiveInteger(value) => Self::PositiveInteger(value),
			NonNegativeIntegerValueRef::UnsignedLong(value) => Self::UnsignedLong(value),
			NonNegativeIntegerValueRef::UnsignedInt(value) => Self::UnsignedInt(value),
			NonNegativeIntegerValueRef::UnsignedShort(value) => Self::UnsignedShort(value),
			NonNegativeIntegerValueRef::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl<'a> TryFrom<ValueRef<'a>> for DecimalValueRef<'a> {
	type Error = ValueRef<'a>;
	fn try_from(value: ValueRef<'a>) -> Result<Self, ValueRef<'a>> {
		match value {
			ValueRef::Decimal(value) => Ok(Self::Decimal(value)),
			ValueRef::Integer(value) => Ok(Self::Integer(value)),
			ValueRef::NonPositiveInteger(value) => Ok(Self::NonPositiveInteger(value)),
			ValueRef::NegativeInteger(value) => Ok(Self::NegativeInteger(value)),
			ValueRef::NonNegativeInteger(value) => Ok(Self::NonNegativeInteger(value)),
			ValueRef::PositiveInteger(value) => Ok(Self::PositiveInteger(value)),
			ValueRef::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			ValueRef::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			ValueRef::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			ValueRef::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			ValueRef::Long(value) => Ok(Self::Long(value)),
			ValueRef::Int(value) => Ok(Self::Int(value)),
			ValueRef::Short(value) => Ok(Self::Short(value)),
			ValueRef::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<ValueRef<'a>> for IntegerValueRef<'a> {
	type Error = ValueRef<'a>;
	fn try_from(value: ValueRef<'a>) -> Result<Self, ValueRef<'a>> {
		match value {
			ValueRef::Integer(value) => Ok(Self::Integer(value)),
			ValueRef::NonPositiveInteger(value) => Ok(Self::NonPositiveInteger(value)),
			ValueRef::NegativeInteger(value) => Ok(Self::NegativeInteger(value)),
			ValueRef::NonNegativeInteger(value) => Ok(Self::NonNegativeInteger(value)),
			ValueRef::PositiveInteger(value) => Ok(Self::PositiveInteger(value)),
			ValueRef::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			ValueRef::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			ValueRef::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			ValueRef::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			ValueRef::Long(value) => Ok(Self::Long(value)),
			ValueRef::Int(value) => Ok(Self::Int(value)),
			ValueRef::Short(value) => Ok(Self::Short(value)),
			ValueRef::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<ValueRef<'a>> for NonPositiveIntegerValueRef<'a> {
	type Error = ValueRef<'a>;
	fn try_from(value: ValueRef<'a>) -> Result<Self, ValueRef<'a>> {
		match value {
			ValueRef::NonPositiveInteger(value) => Ok(Self::NonPositiveInteger(value)),
			ValueRef::NegativeInteger(value) => Ok(Self::NegativeInteger(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<ValueRef<'a>> for NonNegativeIntegerValueRef<'a> {
	type Error = ValueRef<'a>;
	fn try_from(value: ValueRef<'a>) -> Result<Self, ValueRef<'a>> {
		match value {
			ValueRef::NonNegativeInteger(value) => Ok(Self::NonNegativeInteger(value)),
			ValueRef::PositiveInteger(value) => Ok(Self::PositiveInteger(value)),
			ValueRef::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			ValueRef::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			ValueRef::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			ValueRef::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone)]
pub enum DecimalValue {
	Decimal(Decimal),
	Integer(Integer),
	NonPositiveInteger(NonPositiveInteger),
	NegativeInteger(NegativeInteger),
	NonNegativeInteger(NonNegativeInteger),
	PositiveInteger(PositiveInteger),
	UnsignedLong(UnsignedLong),
	UnsignedInt(UnsignedInt),
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
	Long(Long),
	Int(Int),
	Short(Short),
	Byte(Byte),
}
impl DecimalValue {
	pub fn datatype(&self) -> DecimalDatatype {
		match self {
			Self::Decimal(_) => DecimalDatatype::Decimal,
			Self::Integer(_) => DecimalDatatype::Integer(IntegerDatatype::Integer),
			Self::NonPositiveInteger(_) => DecimalDatatype::Integer(
				IntegerDatatype::NonPositiveInteger(NonPositiveIntegerDatatype::NonPositiveInteger),
			),
			Self::NegativeInteger(_) => DecimalDatatype::Integer(
				IntegerDatatype::NonPositiveInteger(NonPositiveIntegerDatatype::NegativeInteger),
			),
			Self::NonNegativeInteger(_) => DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::NonNegativeInteger),
			),
			Self::PositiveInteger(_) => DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::PositiveInteger),
			),
			Self::UnsignedLong(_) => DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedLong),
			)),
			Self::UnsignedInt(_) => DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedInt,
				)),
			)),
			Self::UnsignedShort(_) => {
				DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
					NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
						UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedShort),
					)),
				))
			}
			Self::UnsignedByte(_) => DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedByte),
				)),
			)),
			Self::Long(_) => DecimalDatatype::Integer(IntegerDatatype::Long(LongDatatype::Long)),
			Self::Int(_) => {
				DecimalDatatype::Integer(IntegerDatatype::Long(LongDatatype::Int(IntDatatype::Int)))
			}
			Self::Short(_) => DecimalDatatype::Integer(IntegerDatatype::Long(LongDatatype::Int(
				IntDatatype::Short(ShortDatatype::Short),
			))),
			Self::Byte(_) => DecimalDatatype::Integer(IntegerDatatype::Long(LongDatatype::Int(
				IntDatatype::Short(ShortDatatype::Byte),
			))),
		}
	}
}
impl XsdValue for DecimalValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for DecimalValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Decimal(v) => v.fmt(f),
			Self::Integer(v) => v.fmt(f),
			Self::NonPositiveInteger(v) => v.fmt(f),
			Self::NegativeInteger(v) => v.fmt(f),
			Self::NonNegativeInteger(v) => v.fmt(f),
			Self::PositiveInteger(v) => v.fmt(f),
			Self::UnsignedLong(v) => v.fmt(f),
			Self::UnsignedInt(v) => v.fmt(f),
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
			Self::Long(v) => v.fmt(f),
			Self::Int(v) => v.fmt(f),
			Self::Short(v) => v.fmt(f),
			Self::Byte(v) => v.fmt(f),
		}
	}
}
impl From<IntegerValue> for DecimalValue {
	fn from(value: IntegerValue) -> Self {
		match value {
			IntegerValue::Integer(value) => Self::Integer(value),
			IntegerValue::NonPositiveInteger(value) => Self::NonPositiveInteger(value),
			IntegerValue::NegativeInteger(value) => Self::NegativeInteger(value),
			IntegerValue::NonNegativeInteger(value) => Self::NonNegativeInteger(value),
			IntegerValue::PositiveInteger(value) => Self::PositiveInteger(value),
			IntegerValue::UnsignedLong(value) => Self::UnsignedLong(value),
			IntegerValue::UnsignedInt(value) => Self::UnsignedInt(value),
			IntegerValue::UnsignedShort(value) => Self::UnsignedShort(value),
			IntegerValue::UnsignedByte(value) => Self::UnsignedByte(value),
			IntegerValue::Long(value) => Self::Long(value),
			IntegerValue::Int(value) => Self::Int(value),
			IntegerValue::Short(value) => Self::Short(value),
			IntegerValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl From<NonPositiveIntegerValue> for DecimalValue {
	fn from(value: NonPositiveIntegerValue) -> Self {
		match value {
			NonPositiveIntegerValue::NonPositiveInteger(value) => Self::NonPositiveInteger(value),
			NonPositiveIntegerValue::NegativeInteger(value) => Self::NegativeInteger(value),
		}
	}
}
impl From<NonNegativeIntegerValue> for DecimalValue {
	fn from(value: NonNegativeIntegerValue) -> Self {
		match value {
			NonNegativeIntegerValue::NonNegativeInteger(value) => Self::NonNegativeInteger(value),
			NonNegativeIntegerValue::PositiveInteger(value) => Self::PositiveInteger(value),
			NonNegativeIntegerValue::UnsignedLong(value) => Self::UnsignedLong(value),
			NonNegativeIntegerValue::UnsignedInt(value) => Self::UnsignedInt(value),
			NonNegativeIntegerValue::UnsignedShort(value) => Self::UnsignedShort(value),
			NonNegativeIntegerValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<UnsignedLongValue> for DecimalValue {
	fn from(value: UnsignedLongValue) -> Self {
		match value {
			UnsignedLongValue::UnsignedLong(value) => Self::UnsignedLong(value),
			UnsignedLongValue::UnsignedInt(value) => Self::UnsignedInt(value),
			UnsignedLongValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedLongValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<UnsignedIntValue> for DecimalValue {
	fn from(value: UnsignedIntValue) -> Self {
		match value {
			UnsignedIntValue::UnsignedInt(value) => Self::UnsignedInt(value),
			UnsignedIntValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedIntValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<UnsignedShortValue> for DecimalValue {
	fn from(value: UnsignedShortValue) -> Self {
		match value {
			UnsignedShortValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedShortValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<LongValue> for DecimalValue {
	fn from(value: LongValue) -> Self {
		match value {
			LongValue::Long(value) => Self::Long(value),
			LongValue::Int(value) => Self::Int(value),
			LongValue::Short(value) => Self::Short(value),
			LongValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl From<IntValue> for DecimalValue {
	fn from(value: IntValue) -> Self {
		match value {
			IntValue::Int(value) => Self::Int(value),
			IntValue::Short(value) => Self::Short(value),
			IntValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl From<ShortValue> for DecimalValue {
	fn from(value: ShortValue) -> Self {
		match value {
			ShortValue::Short(value) => Self::Short(value),
			ShortValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl TryFrom<DecimalValue> for IntegerValue {
	type Error = DecimalValue;
	fn try_from(value: DecimalValue) -> Result<Self, DecimalValue> {
		match value {
			DecimalValue::Integer(value) => Ok(Self::Integer(value)),
			DecimalValue::NonPositiveInteger(value) => Ok(Self::NonPositiveInteger(value)),
			DecimalValue::NegativeInteger(value) => Ok(Self::NegativeInteger(value)),
			DecimalValue::NonNegativeInteger(value) => Ok(Self::NonNegativeInteger(value)),
			DecimalValue::PositiveInteger(value) => Ok(Self::PositiveInteger(value)),
			DecimalValue::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			DecimalValue::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			DecimalValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			DecimalValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			DecimalValue::Long(value) => Ok(Self::Long(value)),
			DecimalValue::Int(value) => Ok(Self::Int(value)),
			DecimalValue::Short(value) => Ok(Self::Short(value)),
			DecimalValue::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalValue> for NonPositiveIntegerValue {
	type Error = DecimalValue;
	fn try_from(value: DecimalValue) -> Result<Self, DecimalValue> {
		match value {
			DecimalValue::NonPositiveInteger(value) => Ok(Self::NonPositiveInteger(value)),
			DecimalValue::NegativeInteger(value) => Ok(Self::NegativeInteger(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalValue> for NonNegativeIntegerValue {
	type Error = DecimalValue;
	fn try_from(value: DecimalValue) -> Result<Self, DecimalValue> {
		match value {
			DecimalValue::NonNegativeInteger(value) => Ok(Self::NonNegativeInteger(value)),
			DecimalValue::PositiveInteger(value) => Ok(Self::PositiveInteger(value)),
			DecimalValue::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			DecimalValue::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			DecimalValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			DecimalValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalValue> for UnsignedLongValue {
	type Error = DecimalValue;
	fn try_from(value: DecimalValue) -> Result<Self, DecimalValue> {
		match value {
			DecimalValue::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			DecimalValue::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			DecimalValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			DecimalValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalValue> for UnsignedIntValue {
	type Error = DecimalValue;
	fn try_from(value: DecimalValue) -> Result<Self, DecimalValue> {
		match value {
			DecimalValue::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			DecimalValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			DecimalValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalValue> for UnsignedShortValue {
	type Error = DecimalValue;
	fn try_from(value: DecimalValue) -> Result<Self, DecimalValue> {
		match value {
			DecimalValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			DecimalValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalValue> for LongValue {
	type Error = DecimalValue;
	fn try_from(value: DecimalValue) -> Result<Self, DecimalValue> {
		match value {
			DecimalValue::Long(value) => Ok(Self::Long(value)),
			DecimalValue::Int(value) => Ok(Self::Int(value)),
			DecimalValue::Short(value) => Ok(Self::Short(value)),
			DecimalValue::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalValue> for IntValue {
	type Error = DecimalValue;
	fn try_from(value: DecimalValue) -> Result<Self, DecimalValue> {
		match value {
			DecimalValue::Int(value) => Ok(Self::Int(value)),
			DecimalValue::Short(value) => Ok(Self::Short(value)),
			DecimalValue::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<DecimalValue> for ShortValue {
	type Error = DecimalValue;
	fn try_from(value: DecimalValue) -> Result<Self, DecimalValue> {
		match value {
			DecimalValue::Short(value) => Ok(Self::Short(value)),
			DecimalValue::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntegerDatatype {
	Integer,
	NonPositiveInteger(NonPositiveIntegerDatatype),
	NonNegativeInteger(NonNegativeIntegerDatatype),
	Long(LongDatatype),
}
impl IntegerDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_INTEGER {
			return Some(Self::Integer);
		}
		if let Some(t) = NonPositiveIntegerDatatype::from_iri(iri) {
			return Some(Self::NonPositiveInteger(t));
		}
		if let Some(t) = NonNegativeIntegerDatatype::from_iri(iri) {
			return Some(Self::NonNegativeInteger(t));
		}
		if let Some(t) = LongDatatype::from_iri(iri) {
			return Some(Self::Long(t));
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Integer => XSD_INTEGER,
			Self::NonPositiveInteger(t) => t.iri(),
			Self::NonNegativeInteger(t) => t.iri(),
			Self::Long(t) => t.iri(),
		}
	}
	pub fn parse(&self, value: &str) -> Result<IntegerValue, ParseError> {
		match self {
			Self::Integer => ParseRdf::parse_rdf(value)
				.map(IntegerValue::Integer)
				.map_err(|_| ParseError),
			Self::NonPositiveInteger(t) => t.parse(value).map(Into::into),
			Self::NonNegativeInteger(t) => t.parse(value).map(Into::into),
			Self::Long(t) => t.parse(value).map(Into::into),
		}
	}
}
impl From<NonPositiveIntegerDatatype> for IntegerDatatype {
	fn from(value: NonPositiveIntegerDatatype) -> Self {
		Self::NonPositiveInteger(value)
	}
}
impl TryFrom<IntegerDatatype> for NonPositiveIntegerDatatype {
	type Error = IntegerDatatype;
	fn try_from(value: IntegerDatatype) -> Result<Self, IntegerDatatype> {
		match value {
			IntegerDatatype::NonPositiveInteger(value) => Ok(value),
			other => Err(other),
		}
	}
}
impl From<NonNegativeIntegerDatatype> for IntegerDatatype {
	fn from(value: NonNegativeIntegerDatatype) -> Self {
		Self::NonNegativeInteger(value)
	}
}
impl From<UnsignedLongDatatype> for IntegerDatatype {
	fn from(value: UnsignedLongDatatype) -> Self {
		Self::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(value))
	}
}
impl From<UnsignedIntDatatype> for IntegerDatatype {
	fn from(value: UnsignedIntDatatype) -> Self {
		Self::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
			UnsignedLongDatatype::UnsignedInt(value),
		))
	}
}
impl From<UnsignedShortDatatype> for IntegerDatatype {
	fn from(value: UnsignedShortDatatype) -> Self {
		Self::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
			UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedShort(value)),
		))
	}
}
impl TryFrom<IntegerDatatype> for NonNegativeIntegerDatatype {
	type Error = IntegerDatatype;
	fn try_from(value: IntegerDatatype) -> Result<Self, IntegerDatatype> {
		match value {
			IntegerDatatype::NonNegativeInteger(value) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<IntegerDatatype> for UnsignedLongDatatype {
	type Error = IntegerDatatype;
	fn try_from(value: IntegerDatatype) -> Result<Self, IntegerDatatype> {
		match value {
			IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
				value,
			)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<IntegerDatatype> for UnsignedIntDatatype {
	type Error = IntegerDatatype;
	fn try_from(value: IntegerDatatype) -> Result<Self, IntegerDatatype> {
		match value {
			IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
				UnsignedLongDatatype::UnsignedInt(value),
			)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<IntegerDatatype> for UnsignedShortDatatype {
	type Error = IntegerDatatype;
	fn try_from(value: IntegerDatatype) -> Result<Self, IntegerDatatype> {
		match value {
			IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
				UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedShort(value)),
			)) => Ok(value),
			other => Err(other),
		}
	}
}
impl From<LongDatatype> for IntegerDatatype {
	fn from(value: LongDatatype) -> Self {
		Self::Long(value)
	}
}
impl From<IntDatatype> for IntegerDatatype {
	fn from(value: IntDatatype) -> Self {
		Self::Long(LongDatatype::Int(value))
	}
}
impl From<ShortDatatype> for IntegerDatatype {
	fn from(value: ShortDatatype) -> Self {
		Self::Long(LongDatatype::Int(IntDatatype::Short(value)))
	}
}
impl TryFrom<IntegerDatatype> for LongDatatype {
	type Error = IntegerDatatype;
	fn try_from(value: IntegerDatatype) -> Result<Self, IntegerDatatype> {
		match value {
			IntegerDatatype::Long(value) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<IntegerDatatype> for IntDatatype {
	type Error = IntegerDatatype;
	fn try_from(value: IntegerDatatype) -> Result<Self, IntegerDatatype> {
		match value {
			IntegerDatatype::Long(LongDatatype::Int(value)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<IntegerDatatype> for ShortDatatype {
	type Error = IntegerDatatype;
	fn try_from(value: IntegerDatatype) -> Result<Self, IntegerDatatype> {
		match value {
			IntegerDatatype::Long(LongDatatype::Int(IntDatatype::Short(value))) => Ok(value),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub enum DecimalValueRef<'a> {
	Decimal(&'a Decimal),
	Integer(&'a Integer),
	NonPositiveInteger(&'a NonPositiveInteger),
	NegativeInteger(&'a NegativeInteger),
	NonNegativeInteger(&'a NonNegativeInteger),
	PositiveInteger(&'a PositiveInteger),
	UnsignedLong(UnsignedLong),
	UnsignedInt(UnsignedInt),
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
	Long(Long),
	Int(Int),
	Short(Short),
	Byte(Byte),
}
impl DecimalValue {
	pub fn as_ref(&self) -> DecimalValueRef {
		match self {
			Self::Decimal(value) => DecimalValueRef::Decimal(value),
			Self::Integer(value) => DecimalValueRef::Integer(value),
			Self::NonPositiveInteger(value) => DecimalValueRef::NonPositiveInteger(value),
			Self::NegativeInteger(value) => DecimalValueRef::NegativeInteger(value),
			Self::NonNegativeInteger(value) => DecimalValueRef::NonNegativeInteger(value),
			Self::PositiveInteger(value) => DecimalValueRef::PositiveInteger(value),
			Self::UnsignedLong(value) => DecimalValueRef::UnsignedLong(*value),
			Self::UnsignedInt(value) => DecimalValueRef::UnsignedInt(*value),
			Self::UnsignedShort(value) => DecimalValueRef::UnsignedShort(*value),
			Self::UnsignedByte(value) => DecimalValueRef::UnsignedByte(*value),
			Self::Long(value) => DecimalValueRef::Long(*value),
			Self::Int(value) => DecimalValueRef::Int(*value),
			Self::Short(value) => DecimalValueRef::Short(*value),
			Self::Byte(value) => DecimalValueRef::Byte(*value),
		}
	}
}
impl<'a> DecimalValueRef<'a> {
	pub fn datatype(&self) -> DecimalDatatype {
		match self {
			Self::Decimal(_) => DecimalDatatype::Decimal,
			Self::Integer(_) => DecimalDatatype::Integer(IntegerDatatype::Integer),
			Self::NonPositiveInteger(_) => DecimalDatatype::Integer(
				IntegerDatatype::NonPositiveInteger(NonPositiveIntegerDatatype::NonPositiveInteger),
			),
			Self::NegativeInteger(_) => DecimalDatatype::Integer(
				IntegerDatatype::NonPositiveInteger(NonPositiveIntegerDatatype::NegativeInteger),
			),
			Self::NonNegativeInteger(_) => DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::NonNegativeInteger),
			),
			Self::PositiveInteger(_) => DecimalDatatype::Integer(
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::PositiveInteger),
			),
			Self::UnsignedLong(_) => DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedLong),
			)),
			Self::UnsignedInt(_) => DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedInt,
				)),
			)),
			Self::UnsignedShort(_) => {
				DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
					NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
						UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedShort),
					)),
				))
			}
			Self::UnsignedByte(_) => DecimalDatatype::Integer(IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedByte),
				)),
			)),
			Self::Long(_) => DecimalDatatype::Integer(IntegerDatatype::Long(LongDatatype::Long)),
			Self::Int(_) => {
				DecimalDatatype::Integer(IntegerDatatype::Long(LongDatatype::Int(IntDatatype::Int)))
			}
			Self::Short(_) => DecimalDatatype::Integer(IntegerDatatype::Long(LongDatatype::Int(
				IntDatatype::Short(ShortDatatype::Short),
			))),
			Self::Byte(_) => DecimalDatatype::Integer(IntegerDatatype::Long(LongDatatype::Int(
				IntDatatype::Short(ShortDatatype::Byte),
			))),
		}
	}
	pub fn cloned(&self) -> DecimalValue {
		match *self {
			Self::Decimal(value) => DecimalValue::Decimal(value.to_owned()),
			Self::Integer(value) => DecimalValue::Integer(value.to_owned()),
			Self::NonPositiveInteger(value) => DecimalValue::NonPositiveInteger(value.to_owned()),
			Self::NegativeInteger(value) => DecimalValue::NegativeInteger(value.to_owned()),
			Self::NonNegativeInteger(value) => DecimalValue::NonNegativeInteger(value.to_owned()),
			Self::PositiveInteger(value) => DecimalValue::PositiveInteger(value.to_owned()),
			Self::UnsignedLong(value) => DecimalValue::UnsignedLong(value),
			Self::UnsignedInt(value) => DecimalValue::UnsignedInt(value),
			Self::UnsignedShort(value) => DecimalValue::UnsignedShort(value),
			Self::UnsignedByte(value) => DecimalValue::UnsignedByte(value),
			Self::Long(value) => DecimalValue::Long(value),
			Self::Int(value) => DecimalValue::Int(value),
			Self::Short(value) => DecimalValue::Short(value),
			Self::Byte(value) => DecimalValue::Byte(value),
		}
	}
}
impl<'a> XsdValue for DecimalValueRef<'a> {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl<'a> fmt::Display for DecimalValueRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Decimal(v) => v.fmt(f),
			Self::Integer(v) => v.fmt(f),
			Self::NonPositiveInteger(v) => v.fmt(f),
			Self::NegativeInteger(v) => v.fmt(f),
			Self::NonNegativeInteger(v) => v.fmt(f),
			Self::PositiveInteger(v) => v.fmt(f),
			Self::UnsignedLong(v) => v.fmt(f),
			Self::UnsignedInt(v) => v.fmt(f),
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
			Self::Long(v) => v.fmt(f),
			Self::Int(v) => v.fmt(f),
			Self::Short(v) => v.fmt(f),
			Self::Byte(v) => v.fmt(f),
		}
	}
}
impl<'a> From<IntegerValueRef<'a>> for DecimalValueRef<'a> {
	fn from(value: IntegerValueRef<'a>) -> Self {
		match value {
			IntegerValueRef::Integer(value) => Self::Integer(value),
			IntegerValueRef::NonPositiveInteger(value) => Self::NonPositiveInteger(value),
			IntegerValueRef::NegativeInteger(value) => Self::NegativeInteger(value),
			IntegerValueRef::NonNegativeInteger(value) => Self::NonNegativeInteger(value),
			IntegerValueRef::PositiveInteger(value) => Self::PositiveInteger(value),
			IntegerValueRef::UnsignedLong(value) => Self::UnsignedLong(value),
			IntegerValueRef::UnsignedInt(value) => Self::UnsignedInt(value),
			IntegerValueRef::UnsignedShort(value) => Self::UnsignedShort(value),
			IntegerValueRef::UnsignedByte(value) => Self::UnsignedByte(value),
			IntegerValueRef::Long(value) => Self::Long(value),
			IntegerValueRef::Int(value) => Self::Int(value),
			IntegerValueRef::Short(value) => Self::Short(value),
			IntegerValueRef::Byte(value) => Self::Byte(value),
		}
	}
}
impl<'a> From<NonPositiveIntegerValueRef<'a>> for DecimalValueRef<'a> {
	fn from(value: NonPositiveIntegerValueRef<'a>) -> Self {
		match value {
			NonPositiveIntegerValueRef::NonPositiveInteger(value) => {
				Self::NonPositiveInteger(value)
			}
			NonPositiveIntegerValueRef::NegativeInteger(value) => Self::NegativeInteger(value),
		}
	}
}
impl<'a> From<NonNegativeIntegerValueRef<'a>> for DecimalValueRef<'a> {
	fn from(value: NonNegativeIntegerValueRef<'a>) -> Self {
		match value {
			NonNegativeIntegerValueRef::NonNegativeInteger(value) => {
				Self::NonNegativeInteger(value)
			}
			NonNegativeIntegerValueRef::PositiveInteger(value) => Self::PositiveInteger(value),
			NonNegativeIntegerValueRef::UnsignedLong(value) => Self::UnsignedLong(value),
			NonNegativeIntegerValueRef::UnsignedInt(value) => Self::UnsignedInt(value),
			NonNegativeIntegerValueRef::UnsignedShort(value) => Self::UnsignedShort(value),
			NonNegativeIntegerValueRef::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl<'a> TryFrom<DecimalValueRef<'a>> for IntegerValueRef<'a> {
	type Error = DecimalValueRef<'a>;
	fn try_from(value: DecimalValueRef<'a>) -> Result<Self, DecimalValueRef<'a>> {
		match value {
			DecimalValueRef::Integer(value) => Ok(Self::Integer(value)),
			DecimalValueRef::NonPositiveInteger(value) => Ok(Self::NonPositiveInteger(value)),
			DecimalValueRef::NegativeInteger(value) => Ok(Self::NegativeInteger(value)),
			DecimalValueRef::NonNegativeInteger(value) => Ok(Self::NonNegativeInteger(value)),
			DecimalValueRef::PositiveInteger(value) => Ok(Self::PositiveInteger(value)),
			DecimalValueRef::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			DecimalValueRef::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			DecimalValueRef::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			DecimalValueRef::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			DecimalValueRef::Long(value) => Ok(Self::Long(value)),
			DecimalValueRef::Int(value) => Ok(Self::Int(value)),
			DecimalValueRef::Short(value) => Ok(Self::Short(value)),
			DecimalValueRef::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<DecimalValueRef<'a>> for NonPositiveIntegerValueRef<'a> {
	type Error = DecimalValueRef<'a>;
	fn try_from(value: DecimalValueRef<'a>) -> Result<Self, DecimalValueRef<'a>> {
		match value {
			DecimalValueRef::NonPositiveInteger(value) => Ok(Self::NonPositiveInteger(value)),
			DecimalValueRef::NegativeInteger(value) => Ok(Self::NegativeInteger(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<DecimalValueRef<'a>> for NonNegativeIntegerValueRef<'a> {
	type Error = DecimalValueRef<'a>;
	fn try_from(value: DecimalValueRef<'a>) -> Result<Self, DecimalValueRef<'a>> {
		match value {
			DecimalValueRef::NonNegativeInteger(value) => Ok(Self::NonNegativeInteger(value)),
			DecimalValueRef::PositiveInteger(value) => Ok(Self::PositiveInteger(value)),
			DecimalValueRef::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			DecimalValueRef::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			DecimalValueRef::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			DecimalValueRef::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone)]
pub enum IntegerValue {
	Integer(Integer),
	NonPositiveInteger(NonPositiveInteger),
	NegativeInteger(NegativeInteger),
	NonNegativeInteger(NonNegativeInteger),
	PositiveInteger(PositiveInteger),
	UnsignedLong(UnsignedLong),
	UnsignedInt(UnsignedInt),
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
	Long(Long),
	Int(Int),
	Short(Short),
	Byte(Byte),
}
impl IntegerValue {
	pub fn datatype(&self) -> IntegerDatatype {
		match self {
			Self::Integer(_) => IntegerDatatype::Integer,
			Self::NonPositiveInteger(_) => {
				IntegerDatatype::NonPositiveInteger(NonPositiveIntegerDatatype::NonPositiveInteger)
			}
			Self::NegativeInteger(_) => {
				IntegerDatatype::NonPositiveInteger(NonPositiveIntegerDatatype::NegativeInteger)
			}
			Self::NonNegativeInteger(_) => {
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::NonNegativeInteger)
			}
			Self::PositiveInteger(_) => {
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::PositiveInteger)
			}
			Self::UnsignedLong(_) => IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedLong),
			),
			Self::UnsignedInt(_) => {
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
					UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedInt),
				))
			}
			Self::UnsignedShort(_) => IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedShort),
				)),
			),
			Self::UnsignedByte(_) => IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedByte),
				)),
			),
			Self::Long(_) => IntegerDatatype::Long(LongDatatype::Long),
			Self::Int(_) => IntegerDatatype::Long(LongDatatype::Int(IntDatatype::Int)),
			Self::Short(_) => {
				IntegerDatatype::Long(LongDatatype::Int(IntDatatype::Short(ShortDatatype::Short)))
			}
			Self::Byte(_) => {
				IntegerDatatype::Long(LongDatatype::Int(IntDatatype::Short(ShortDatatype::Byte)))
			}
		}
	}
}
impl XsdValue for IntegerValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for IntegerValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Integer(v) => v.fmt(f),
			Self::NonPositiveInteger(v) => v.fmt(f),
			Self::NegativeInteger(v) => v.fmt(f),
			Self::NonNegativeInteger(v) => v.fmt(f),
			Self::PositiveInteger(v) => v.fmt(f),
			Self::UnsignedLong(v) => v.fmt(f),
			Self::UnsignedInt(v) => v.fmt(f),
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
			Self::Long(v) => v.fmt(f),
			Self::Int(v) => v.fmt(f),
			Self::Short(v) => v.fmt(f),
			Self::Byte(v) => v.fmt(f),
		}
	}
}
impl From<NonPositiveIntegerValue> for IntegerValue {
	fn from(value: NonPositiveIntegerValue) -> Self {
		match value {
			NonPositiveIntegerValue::NonPositiveInteger(value) => Self::NonPositiveInteger(value),
			NonPositiveIntegerValue::NegativeInteger(value) => Self::NegativeInteger(value),
		}
	}
}
impl TryFrom<IntegerValue> for NonPositiveIntegerValue {
	type Error = IntegerValue;
	fn try_from(value: IntegerValue) -> Result<Self, IntegerValue> {
		match value {
			IntegerValue::NonPositiveInteger(value) => Ok(Self::NonPositiveInteger(value)),
			IntegerValue::NegativeInteger(value) => Ok(Self::NegativeInteger(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonPositiveIntegerDatatype {
	NonPositiveInteger,
	NegativeInteger,
}
impl NonPositiveIntegerDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_NON_POSITIVE_INTEGER {
			return Some(Self::NonPositiveInteger);
		}
		if iri == XSD_NEGATIVE_INTEGER {
			return Some(Self::NegativeInteger);
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::NonPositiveInteger => XSD_NON_POSITIVE_INTEGER,
			Self::NegativeInteger => XSD_NEGATIVE_INTEGER,
		}
	}
	pub fn parse(&self, value: &str) -> Result<NonPositiveIntegerValue, ParseError> {
		match self {
			Self::NonPositiveInteger => ParseRdf::parse_rdf(value)
				.map(NonPositiveIntegerValue::NonPositiveInteger)
				.map_err(|_| ParseError),
			Self::NegativeInteger => ParseRdf::parse_rdf(value)
				.map(NonPositiveIntegerValue::NegativeInteger)
				.map_err(|_| ParseError),
		}
	}
}
impl From<NonNegativeIntegerValue> for IntegerValue {
	fn from(value: NonNegativeIntegerValue) -> Self {
		match value {
			NonNegativeIntegerValue::NonNegativeInteger(value) => Self::NonNegativeInteger(value),
			NonNegativeIntegerValue::PositiveInteger(value) => Self::PositiveInteger(value),
			NonNegativeIntegerValue::UnsignedLong(value) => Self::UnsignedLong(value),
			NonNegativeIntegerValue::UnsignedInt(value) => Self::UnsignedInt(value),
			NonNegativeIntegerValue::UnsignedShort(value) => Self::UnsignedShort(value),
			NonNegativeIntegerValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<UnsignedLongValue> for IntegerValue {
	fn from(value: UnsignedLongValue) -> Self {
		match value {
			UnsignedLongValue::UnsignedLong(value) => Self::UnsignedLong(value),
			UnsignedLongValue::UnsignedInt(value) => Self::UnsignedInt(value),
			UnsignedLongValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedLongValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<UnsignedIntValue> for IntegerValue {
	fn from(value: UnsignedIntValue) -> Self {
		match value {
			UnsignedIntValue::UnsignedInt(value) => Self::UnsignedInt(value),
			UnsignedIntValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedIntValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<UnsignedShortValue> for IntegerValue {
	fn from(value: UnsignedShortValue) -> Self {
		match value {
			UnsignedShortValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedShortValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl TryFrom<IntegerValue> for NonNegativeIntegerValue {
	type Error = IntegerValue;
	fn try_from(value: IntegerValue) -> Result<Self, IntegerValue> {
		match value {
			IntegerValue::NonNegativeInteger(value) => Ok(Self::NonNegativeInteger(value)),
			IntegerValue::PositiveInteger(value) => Ok(Self::PositiveInteger(value)),
			IntegerValue::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			IntegerValue::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			IntegerValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			IntegerValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<IntegerValue> for UnsignedLongValue {
	type Error = IntegerValue;
	fn try_from(value: IntegerValue) -> Result<Self, IntegerValue> {
		match value {
			IntegerValue::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			IntegerValue::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			IntegerValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			IntegerValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<IntegerValue> for UnsignedIntValue {
	type Error = IntegerValue;
	fn try_from(value: IntegerValue) -> Result<Self, IntegerValue> {
		match value {
			IntegerValue::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			IntegerValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			IntegerValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<IntegerValue> for UnsignedShortValue {
	type Error = IntegerValue;
	fn try_from(value: IntegerValue) -> Result<Self, IntegerValue> {
		match value {
			IntegerValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			IntegerValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonNegativeIntegerDatatype {
	NonNegativeInteger,
	PositiveInteger,
	UnsignedLong(UnsignedLongDatatype),
}
impl NonNegativeIntegerDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_NON_NEGATIVE_INTEGER {
			return Some(Self::NonNegativeInteger);
		}
		if iri == XSD_POSITIVE_INTEGER {
			return Some(Self::PositiveInteger);
		}
		if let Some(t) = UnsignedLongDatatype::from_iri(iri) {
			return Some(Self::UnsignedLong(t));
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::NonNegativeInteger => XSD_NON_NEGATIVE_INTEGER,
			Self::PositiveInteger => XSD_POSITIVE_INTEGER,
			Self::UnsignedLong(t) => t.iri(),
		}
	}
	pub fn parse(&self, value: &str) -> Result<NonNegativeIntegerValue, ParseError> {
		match self {
			Self::NonNegativeInteger => ParseRdf::parse_rdf(value)
				.map(NonNegativeIntegerValue::NonNegativeInteger)
				.map_err(|_| ParseError),
			Self::PositiveInteger => ParseRdf::parse_rdf(value)
				.map(NonNegativeIntegerValue::PositiveInteger)
				.map_err(|_| ParseError),
			Self::UnsignedLong(t) => t.parse(value).map(Into::into),
		}
	}
}
impl From<UnsignedLongDatatype> for NonNegativeIntegerDatatype {
	fn from(value: UnsignedLongDatatype) -> Self {
		Self::UnsignedLong(value)
	}
}
impl From<UnsignedIntDatatype> for NonNegativeIntegerDatatype {
	fn from(value: UnsignedIntDatatype) -> Self {
		Self::UnsignedLong(UnsignedLongDatatype::UnsignedInt(value))
	}
}
impl From<UnsignedShortDatatype> for NonNegativeIntegerDatatype {
	fn from(value: UnsignedShortDatatype) -> Self {
		Self::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
			UnsignedIntDatatype::UnsignedShort(value),
		))
	}
}
impl TryFrom<NonNegativeIntegerDatatype> for UnsignedLongDatatype {
	type Error = NonNegativeIntegerDatatype;
	fn try_from(value: NonNegativeIntegerDatatype) -> Result<Self, NonNegativeIntegerDatatype> {
		match value {
			NonNegativeIntegerDatatype::UnsignedLong(value) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<NonNegativeIntegerDatatype> for UnsignedIntDatatype {
	type Error = NonNegativeIntegerDatatype;
	fn try_from(value: NonNegativeIntegerDatatype) -> Result<Self, NonNegativeIntegerDatatype> {
		match value {
			NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(value)) => {
				Ok(value)
			}
			other => Err(other),
		}
	}
}
impl TryFrom<NonNegativeIntegerDatatype> for UnsignedShortDatatype {
	type Error = NonNegativeIntegerDatatype;
	fn try_from(value: NonNegativeIntegerDatatype) -> Result<Self, NonNegativeIntegerDatatype> {
		match value {
			NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
				UnsignedIntDatatype::UnsignedShort(value),
			)) => Ok(value),
			other => Err(other),
		}
	}
}
impl From<LongValue> for IntegerValue {
	fn from(value: LongValue) -> Self {
		match value {
			LongValue::Long(value) => Self::Long(value),
			LongValue::Int(value) => Self::Int(value),
			LongValue::Short(value) => Self::Short(value),
			LongValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl From<IntValue> for IntegerValue {
	fn from(value: IntValue) -> Self {
		match value {
			IntValue::Int(value) => Self::Int(value),
			IntValue::Short(value) => Self::Short(value),
			IntValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl From<ShortValue> for IntegerValue {
	fn from(value: ShortValue) -> Self {
		match value {
			ShortValue::Short(value) => Self::Short(value),
			ShortValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl TryFrom<IntegerValue> for LongValue {
	type Error = IntegerValue;
	fn try_from(value: IntegerValue) -> Result<Self, IntegerValue> {
		match value {
			IntegerValue::Long(value) => Ok(Self::Long(value)),
			IntegerValue::Int(value) => Ok(Self::Int(value)),
			IntegerValue::Short(value) => Ok(Self::Short(value)),
			IntegerValue::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<IntegerValue> for IntValue {
	type Error = IntegerValue;
	fn try_from(value: IntegerValue) -> Result<Self, IntegerValue> {
		match value {
			IntegerValue::Int(value) => Ok(Self::Int(value)),
			IntegerValue::Short(value) => Ok(Self::Short(value)),
			IntegerValue::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<IntegerValue> for ShortValue {
	type Error = IntegerValue;
	fn try_from(value: IntegerValue) -> Result<Self, IntegerValue> {
		match value {
			IntegerValue::Short(value) => Ok(Self::Short(value)),
			IntegerValue::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LongDatatype {
	Long,
	Int(IntDatatype),
}
impl LongDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_LONG {
			return Some(Self::Long);
		}
		if let Some(t) = IntDatatype::from_iri(iri) {
			return Some(Self::Int(t));
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Long => XSD_LONG,
			Self::Int(t) => t.iri(),
		}
	}
	pub fn parse(&self, value: &str) -> Result<LongValue, ParseError> {
		match self {
			Self::Long => ParseRdf::parse_rdf(value)
				.map(LongValue::Long)
				.map_err(|_| ParseError),
			Self::Int(t) => t.parse(value).map(Into::into),
		}
	}
}
impl From<IntDatatype> for LongDatatype {
	fn from(value: IntDatatype) -> Self {
		Self::Int(value)
	}
}
impl From<ShortDatatype> for LongDatatype {
	fn from(value: ShortDatatype) -> Self {
		Self::Int(IntDatatype::Short(value))
	}
}
impl TryFrom<LongDatatype> for IntDatatype {
	type Error = LongDatatype;
	fn try_from(value: LongDatatype) -> Result<Self, LongDatatype> {
		match value {
			LongDatatype::Int(value) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<LongDatatype> for ShortDatatype {
	type Error = LongDatatype;
	fn try_from(value: LongDatatype) -> Result<Self, LongDatatype> {
		match value {
			LongDatatype::Int(IntDatatype::Short(value)) => Ok(value),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub enum IntegerValueRef<'a> {
	Integer(&'a Integer),
	NonPositiveInteger(&'a NonPositiveInteger),
	NegativeInteger(&'a NegativeInteger),
	NonNegativeInteger(&'a NonNegativeInteger),
	PositiveInteger(&'a PositiveInteger),
	UnsignedLong(UnsignedLong),
	UnsignedInt(UnsignedInt),
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
	Long(Long),
	Int(Int),
	Short(Short),
	Byte(Byte),
}
impl IntegerValue {
	pub fn as_ref(&self) -> IntegerValueRef {
		match self {
			Self::Integer(value) => IntegerValueRef::Integer(value),
			Self::NonPositiveInteger(value) => IntegerValueRef::NonPositiveInteger(value),
			Self::NegativeInteger(value) => IntegerValueRef::NegativeInteger(value),
			Self::NonNegativeInteger(value) => IntegerValueRef::NonNegativeInteger(value),
			Self::PositiveInteger(value) => IntegerValueRef::PositiveInteger(value),
			Self::UnsignedLong(value) => IntegerValueRef::UnsignedLong(*value),
			Self::UnsignedInt(value) => IntegerValueRef::UnsignedInt(*value),
			Self::UnsignedShort(value) => IntegerValueRef::UnsignedShort(*value),
			Self::UnsignedByte(value) => IntegerValueRef::UnsignedByte(*value),
			Self::Long(value) => IntegerValueRef::Long(*value),
			Self::Int(value) => IntegerValueRef::Int(*value),
			Self::Short(value) => IntegerValueRef::Short(*value),
			Self::Byte(value) => IntegerValueRef::Byte(*value),
		}
	}
}
impl<'a> IntegerValueRef<'a> {
	pub fn datatype(&self) -> IntegerDatatype {
		match self {
			Self::Integer(_) => IntegerDatatype::Integer,
			Self::NonPositiveInteger(_) => {
				IntegerDatatype::NonPositiveInteger(NonPositiveIntegerDatatype::NonPositiveInteger)
			}
			Self::NegativeInteger(_) => {
				IntegerDatatype::NonPositiveInteger(NonPositiveIntegerDatatype::NegativeInteger)
			}
			Self::NonNegativeInteger(_) => {
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::NonNegativeInteger)
			}
			Self::PositiveInteger(_) => {
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::PositiveInteger)
			}
			Self::UnsignedLong(_) => IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedLong),
			),
			Self::UnsignedInt(_) => {
				IntegerDatatype::NonNegativeInteger(NonNegativeIntegerDatatype::UnsignedLong(
					UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedInt),
				))
			}
			Self::UnsignedShort(_) => IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedShort),
				)),
			),
			Self::UnsignedByte(_) => IntegerDatatype::NonNegativeInteger(
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedByte),
				)),
			),
			Self::Long(_) => IntegerDatatype::Long(LongDatatype::Long),
			Self::Int(_) => IntegerDatatype::Long(LongDatatype::Int(IntDatatype::Int)),
			Self::Short(_) => {
				IntegerDatatype::Long(LongDatatype::Int(IntDatatype::Short(ShortDatatype::Short)))
			}
			Self::Byte(_) => {
				IntegerDatatype::Long(LongDatatype::Int(IntDatatype::Short(ShortDatatype::Byte)))
			}
		}
	}
	pub fn cloned(&self) -> IntegerValue {
		match *self {
			Self::Integer(value) => IntegerValue::Integer(value.to_owned()),
			Self::NonPositiveInteger(value) => IntegerValue::NonPositiveInteger(value.to_owned()),
			Self::NegativeInteger(value) => IntegerValue::NegativeInteger(value.to_owned()),
			Self::NonNegativeInteger(value) => IntegerValue::NonNegativeInteger(value.to_owned()),
			Self::PositiveInteger(value) => IntegerValue::PositiveInteger(value.to_owned()),
			Self::UnsignedLong(value) => IntegerValue::UnsignedLong(value),
			Self::UnsignedInt(value) => IntegerValue::UnsignedInt(value),
			Self::UnsignedShort(value) => IntegerValue::UnsignedShort(value),
			Self::UnsignedByte(value) => IntegerValue::UnsignedByte(value),
			Self::Long(value) => IntegerValue::Long(value),
			Self::Int(value) => IntegerValue::Int(value),
			Self::Short(value) => IntegerValue::Short(value),
			Self::Byte(value) => IntegerValue::Byte(value),
		}
	}
}
impl<'a> XsdValue for IntegerValueRef<'a> {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl<'a> fmt::Display for IntegerValueRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Integer(v) => v.fmt(f),
			Self::NonPositiveInteger(v) => v.fmt(f),
			Self::NegativeInteger(v) => v.fmt(f),
			Self::NonNegativeInteger(v) => v.fmt(f),
			Self::PositiveInteger(v) => v.fmt(f),
			Self::UnsignedLong(v) => v.fmt(f),
			Self::UnsignedInt(v) => v.fmt(f),
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
			Self::Long(v) => v.fmt(f),
			Self::Int(v) => v.fmt(f),
			Self::Short(v) => v.fmt(f),
			Self::Byte(v) => v.fmt(f),
		}
	}
}
impl<'a> From<NonPositiveIntegerValueRef<'a>> for IntegerValueRef<'a> {
	fn from(value: NonPositiveIntegerValueRef<'a>) -> Self {
		match value {
			NonPositiveIntegerValueRef::NonPositiveInteger(value) => {
				Self::NonPositiveInteger(value)
			}
			NonPositiveIntegerValueRef::NegativeInteger(value) => Self::NegativeInteger(value),
		}
	}
}
impl<'a> TryFrom<IntegerValueRef<'a>> for NonPositiveIntegerValueRef<'a> {
	type Error = IntegerValueRef<'a>;
	fn try_from(value: IntegerValueRef<'a>) -> Result<Self, IntegerValueRef<'a>> {
		match value {
			IntegerValueRef::NonPositiveInteger(value) => Ok(Self::NonPositiveInteger(value)),
			IntegerValueRef::NegativeInteger(value) => Ok(Self::NegativeInteger(value)),
			other => Err(other),
		}
	}
}
impl<'a> From<NonNegativeIntegerValueRef<'a>> for IntegerValueRef<'a> {
	fn from(value: NonNegativeIntegerValueRef<'a>) -> Self {
		match value {
			NonNegativeIntegerValueRef::NonNegativeInteger(value) => {
				Self::NonNegativeInteger(value)
			}
			NonNegativeIntegerValueRef::PositiveInteger(value) => Self::PositiveInteger(value),
			NonNegativeIntegerValueRef::UnsignedLong(value) => Self::UnsignedLong(value),
			NonNegativeIntegerValueRef::UnsignedInt(value) => Self::UnsignedInt(value),
			NonNegativeIntegerValueRef::UnsignedShort(value) => Self::UnsignedShort(value),
			NonNegativeIntegerValueRef::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl<'a> TryFrom<IntegerValueRef<'a>> for NonNegativeIntegerValueRef<'a> {
	type Error = IntegerValueRef<'a>;
	fn try_from(value: IntegerValueRef<'a>) -> Result<Self, IntegerValueRef<'a>> {
		match value {
			IntegerValueRef::NonNegativeInteger(value) => Ok(Self::NonNegativeInteger(value)),
			IntegerValueRef::PositiveInteger(value) => Ok(Self::PositiveInteger(value)),
			IntegerValueRef::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			IntegerValueRef::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			IntegerValueRef::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			IntegerValueRef::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone)]
pub enum NonPositiveIntegerValue {
	NonPositiveInteger(NonPositiveInteger),
	NegativeInteger(NegativeInteger),
}
impl NonPositiveIntegerValue {
	pub fn datatype(&self) -> NonPositiveIntegerDatatype {
		match self {
			Self::NonPositiveInteger(_) => NonPositiveIntegerDatatype::NonPositiveInteger,
			Self::NegativeInteger(_) => NonPositiveIntegerDatatype::NegativeInteger,
		}
	}
}
impl XsdValue for NonPositiveIntegerValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for NonPositiveIntegerValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NonPositiveInteger(v) => v.fmt(f),
			Self::NegativeInteger(v) => v.fmt(f),
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub enum NonPositiveIntegerValueRef<'a> {
	NonPositiveInteger(&'a NonPositiveInteger),
	NegativeInteger(&'a NegativeInteger),
}
impl NonPositiveIntegerValue {
	pub fn as_ref(&self) -> NonPositiveIntegerValueRef {
		match self {
			Self::NonPositiveInteger(value) => {
				NonPositiveIntegerValueRef::NonPositiveInteger(value)
			}
			Self::NegativeInteger(value) => NonPositiveIntegerValueRef::NegativeInteger(value),
		}
	}
}
impl<'a> NonPositiveIntegerValueRef<'a> {
	pub fn datatype(&self) -> NonPositiveIntegerDatatype {
		match self {
			Self::NonPositiveInteger(_) => NonPositiveIntegerDatatype::NonPositiveInteger,
			Self::NegativeInteger(_) => NonPositiveIntegerDatatype::NegativeInteger,
		}
	}
	pub fn cloned(&self) -> NonPositiveIntegerValue {
		match *self {
			Self::NonPositiveInteger(value) => {
				NonPositiveIntegerValue::NonPositiveInteger(value.to_owned())
			}
			Self::NegativeInteger(value) => {
				NonPositiveIntegerValue::NegativeInteger(value.to_owned())
			}
		}
	}
}
impl<'a> XsdValue for NonPositiveIntegerValueRef<'a> {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl<'a> fmt::Display for NonPositiveIntegerValueRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NonPositiveInteger(v) => v.fmt(f),
			Self::NegativeInteger(v) => v.fmt(f),
		}
	}
}
#[derive(Debug, Clone)]
pub enum NonNegativeIntegerValue {
	NonNegativeInteger(NonNegativeInteger),
	PositiveInteger(PositiveInteger),
	UnsignedLong(UnsignedLong),
	UnsignedInt(UnsignedInt),
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
}
impl NonNegativeIntegerValue {
	pub fn datatype(&self) -> NonNegativeIntegerDatatype {
		match self {
			Self::NonNegativeInteger(_) => NonNegativeIntegerDatatype::NonNegativeInteger,
			Self::PositiveInteger(_) => NonNegativeIntegerDatatype::PositiveInteger,
			Self::UnsignedLong(_) => {
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedLong)
			}
			Self::UnsignedInt(_) => NonNegativeIntegerDatatype::UnsignedLong(
				UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedInt),
			),
			Self::UnsignedShort(_) => {
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedShort),
				))
			}
			Self::UnsignedByte(_) => {
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedByte),
				))
			}
		}
	}
}
impl XsdValue for NonNegativeIntegerValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for NonNegativeIntegerValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NonNegativeInteger(v) => v.fmt(f),
			Self::PositiveInteger(v) => v.fmt(f),
			Self::UnsignedLong(v) => v.fmt(f),
			Self::UnsignedInt(v) => v.fmt(f),
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
		}
	}
}
impl From<UnsignedLongValue> for NonNegativeIntegerValue {
	fn from(value: UnsignedLongValue) -> Self {
		match value {
			UnsignedLongValue::UnsignedLong(value) => Self::UnsignedLong(value),
			UnsignedLongValue::UnsignedInt(value) => Self::UnsignedInt(value),
			UnsignedLongValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedLongValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<UnsignedIntValue> for NonNegativeIntegerValue {
	fn from(value: UnsignedIntValue) -> Self {
		match value {
			UnsignedIntValue::UnsignedInt(value) => Self::UnsignedInt(value),
			UnsignedIntValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedIntValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<UnsignedShortValue> for NonNegativeIntegerValue {
	fn from(value: UnsignedShortValue) -> Self {
		match value {
			UnsignedShortValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedShortValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl TryFrom<NonNegativeIntegerValue> for UnsignedLongValue {
	type Error = NonNegativeIntegerValue;
	fn try_from(value: NonNegativeIntegerValue) -> Result<Self, NonNegativeIntegerValue> {
		match value {
			NonNegativeIntegerValue::UnsignedLong(value) => Ok(Self::UnsignedLong(value)),
			NonNegativeIntegerValue::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			NonNegativeIntegerValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			NonNegativeIntegerValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<NonNegativeIntegerValue> for UnsignedIntValue {
	type Error = NonNegativeIntegerValue;
	fn try_from(value: NonNegativeIntegerValue) -> Result<Self, NonNegativeIntegerValue> {
		match value {
			NonNegativeIntegerValue::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			NonNegativeIntegerValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			NonNegativeIntegerValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<NonNegativeIntegerValue> for UnsignedShortValue {
	type Error = NonNegativeIntegerValue;
	fn try_from(value: NonNegativeIntegerValue) -> Result<Self, NonNegativeIntegerValue> {
		match value {
			NonNegativeIntegerValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			NonNegativeIntegerValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnsignedLongDatatype {
	UnsignedLong,
	UnsignedInt(UnsignedIntDatatype),
}
impl UnsignedLongDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_UNSIGNED_LONG {
			return Some(Self::UnsignedLong);
		}
		if let Some(t) = UnsignedIntDatatype::from_iri(iri) {
			return Some(Self::UnsignedInt(t));
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::UnsignedLong => XSD_UNSIGNED_LONG,
			Self::UnsignedInt(t) => t.iri(),
		}
	}
	pub fn parse(&self, value: &str) -> Result<UnsignedLongValue, ParseError> {
		match self {
			Self::UnsignedLong => ParseRdf::parse_rdf(value)
				.map(UnsignedLongValue::UnsignedLong)
				.map_err(|_| ParseError),
			Self::UnsignedInt(t) => t.parse(value).map(Into::into),
		}
	}
}
impl From<UnsignedIntDatatype> for UnsignedLongDatatype {
	fn from(value: UnsignedIntDatatype) -> Self {
		Self::UnsignedInt(value)
	}
}
impl From<UnsignedShortDatatype> for UnsignedLongDatatype {
	fn from(value: UnsignedShortDatatype) -> Self {
		Self::UnsignedInt(UnsignedIntDatatype::UnsignedShort(value))
	}
}
impl TryFrom<UnsignedLongDatatype> for UnsignedIntDatatype {
	type Error = UnsignedLongDatatype;
	fn try_from(value: UnsignedLongDatatype) -> Result<Self, UnsignedLongDatatype> {
		match value {
			UnsignedLongDatatype::UnsignedInt(value) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<UnsignedLongDatatype> for UnsignedShortDatatype {
	type Error = UnsignedLongDatatype;
	fn try_from(value: UnsignedLongDatatype) -> Result<Self, UnsignedLongDatatype> {
		match value {
			UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedShort(value)) => {
				Ok(value)
			}
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub enum NonNegativeIntegerValueRef<'a> {
	NonNegativeInteger(&'a NonNegativeInteger),
	PositiveInteger(&'a PositiveInteger),
	UnsignedLong(UnsignedLong),
	UnsignedInt(UnsignedInt),
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
}
impl NonNegativeIntegerValue {
	pub fn as_ref(&self) -> NonNegativeIntegerValueRef {
		match self {
			Self::NonNegativeInteger(value) => {
				NonNegativeIntegerValueRef::NonNegativeInteger(value)
			}
			Self::PositiveInteger(value) => NonNegativeIntegerValueRef::PositiveInteger(value),
			Self::UnsignedLong(value) => NonNegativeIntegerValueRef::UnsignedLong(*value),
			Self::UnsignedInt(value) => NonNegativeIntegerValueRef::UnsignedInt(*value),
			Self::UnsignedShort(value) => NonNegativeIntegerValueRef::UnsignedShort(*value),
			Self::UnsignedByte(value) => NonNegativeIntegerValueRef::UnsignedByte(*value),
		}
	}
}
impl<'a> NonNegativeIntegerValueRef<'a> {
	pub fn datatype(&self) -> NonNegativeIntegerDatatype {
		match self {
			Self::NonNegativeInteger(_) => NonNegativeIntegerDatatype::NonNegativeInteger,
			Self::PositiveInteger(_) => NonNegativeIntegerDatatype::PositiveInteger,
			Self::UnsignedLong(_) => {
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedLong)
			}
			Self::UnsignedInt(_) => NonNegativeIntegerDatatype::UnsignedLong(
				UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedInt),
			),
			Self::UnsignedShort(_) => {
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedShort),
				))
			}
			Self::UnsignedByte(_) => {
				NonNegativeIntegerDatatype::UnsignedLong(UnsignedLongDatatype::UnsignedInt(
					UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedByte),
				))
			}
		}
	}
	pub fn cloned(&self) -> NonNegativeIntegerValue {
		match *self {
			Self::NonNegativeInteger(value) => {
				NonNegativeIntegerValue::NonNegativeInteger(value.to_owned())
			}
			Self::PositiveInteger(value) => {
				NonNegativeIntegerValue::PositiveInteger(value.to_owned())
			}
			Self::UnsignedLong(value) => NonNegativeIntegerValue::UnsignedLong(value),
			Self::UnsignedInt(value) => NonNegativeIntegerValue::UnsignedInt(value),
			Self::UnsignedShort(value) => NonNegativeIntegerValue::UnsignedShort(value),
			Self::UnsignedByte(value) => NonNegativeIntegerValue::UnsignedByte(value),
		}
	}
}
impl<'a> XsdValue for NonNegativeIntegerValueRef<'a> {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl<'a> fmt::Display for NonNegativeIntegerValueRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NonNegativeInteger(v) => v.fmt(f),
			Self::PositiveInteger(v) => v.fmt(f),
			Self::UnsignedLong(v) => v.fmt(f),
			Self::UnsignedInt(v) => v.fmt(f),
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
		}
	}
}
#[derive(Debug, Clone)]
pub enum UnsignedLongValue {
	UnsignedLong(UnsignedLong),
	UnsignedInt(UnsignedInt),
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
}
impl UnsignedLongValue {
	pub fn datatype(&self) -> UnsignedLongDatatype {
		match self {
			Self::UnsignedLong(_) => UnsignedLongDatatype::UnsignedLong,
			Self::UnsignedInt(_) => {
				UnsignedLongDatatype::UnsignedInt(UnsignedIntDatatype::UnsignedInt)
			}
			Self::UnsignedShort(_) => UnsignedLongDatatype::UnsignedInt(
				UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedShort),
			),
			Self::UnsignedByte(_) => UnsignedLongDatatype::UnsignedInt(
				UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedByte),
			),
		}
	}
}
impl XsdValue for UnsignedLongValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for UnsignedLongValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::UnsignedLong(v) => v.fmt(f),
			Self::UnsignedInt(v) => v.fmt(f),
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
		}
	}
}
impl From<UnsignedIntValue> for UnsignedLongValue {
	fn from(value: UnsignedIntValue) -> Self {
		match value {
			UnsignedIntValue::UnsignedInt(value) => Self::UnsignedInt(value),
			UnsignedIntValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedIntValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl From<UnsignedShortValue> for UnsignedLongValue {
	fn from(value: UnsignedShortValue) -> Self {
		match value {
			UnsignedShortValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedShortValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl TryFrom<UnsignedLongValue> for UnsignedIntValue {
	type Error = UnsignedLongValue;
	fn try_from(value: UnsignedLongValue) -> Result<Self, UnsignedLongValue> {
		match value {
			UnsignedLongValue::UnsignedInt(value) => Ok(Self::UnsignedInt(value)),
			UnsignedLongValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			UnsignedLongValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<UnsignedLongValue> for UnsignedShortValue {
	type Error = UnsignedLongValue;
	fn try_from(value: UnsignedLongValue) -> Result<Self, UnsignedLongValue> {
		match value {
			UnsignedLongValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			UnsignedLongValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnsignedIntDatatype {
	UnsignedInt,
	UnsignedShort(UnsignedShortDatatype),
}
impl UnsignedIntDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_UNSIGNED_INT {
			return Some(Self::UnsignedInt);
		}
		if let Some(t) = UnsignedShortDatatype::from_iri(iri) {
			return Some(Self::UnsignedShort(t));
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::UnsignedInt => XSD_UNSIGNED_INT,
			Self::UnsignedShort(t) => t.iri(),
		}
	}
	pub fn parse(&self, value: &str) -> Result<UnsignedIntValue, ParseError> {
		match self {
			Self::UnsignedInt => ParseRdf::parse_rdf(value)
				.map(UnsignedIntValue::UnsignedInt)
				.map_err(|_| ParseError),
			Self::UnsignedShort(t) => t.parse(value).map(Into::into),
		}
	}
}
impl From<UnsignedShortDatatype> for UnsignedIntDatatype {
	fn from(value: UnsignedShortDatatype) -> Self {
		Self::UnsignedShort(value)
	}
}
impl TryFrom<UnsignedIntDatatype> for UnsignedShortDatatype {
	type Error = UnsignedIntDatatype;
	fn try_from(value: UnsignedIntDatatype) -> Result<Self, UnsignedIntDatatype> {
		match value {
			UnsignedIntDatatype::UnsignedShort(value) => Ok(value),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone)]
pub enum UnsignedIntValue {
	UnsignedInt(UnsignedInt),
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
}
impl UnsignedIntValue {
	pub fn datatype(&self) -> UnsignedIntDatatype {
		match self {
			Self::UnsignedInt(_) => UnsignedIntDatatype::UnsignedInt,
			Self::UnsignedShort(_) => {
				UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedShort)
			}
			Self::UnsignedByte(_) => {
				UnsignedIntDatatype::UnsignedShort(UnsignedShortDatatype::UnsignedByte)
			}
		}
	}
}
impl XsdValue for UnsignedIntValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for UnsignedIntValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::UnsignedInt(v) => v.fmt(f),
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
		}
	}
}
impl From<UnsignedShortValue> for UnsignedIntValue {
	fn from(value: UnsignedShortValue) -> Self {
		match value {
			UnsignedShortValue::UnsignedShort(value) => Self::UnsignedShort(value),
			UnsignedShortValue::UnsignedByte(value) => Self::UnsignedByte(value),
		}
	}
}
impl TryFrom<UnsignedIntValue> for UnsignedShortValue {
	type Error = UnsignedIntValue;
	fn try_from(value: UnsignedIntValue) -> Result<Self, UnsignedIntValue> {
		match value {
			UnsignedIntValue::UnsignedShort(value) => Ok(Self::UnsignedShort(value)),
			UnsignedIntValue::UnsignedByte(value) => Ok(Self::UnsignedByte(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnsignedShortDatatype {
	UnsignedShort,
	UnsignedByte,
}
impl UnsignedShortDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_UNSIGNED_SHORT {
			return Some(Self::UnsignedShort);
		}
		if iri == XSD_UNSIGNED_BYTE {
			return Some(Self::UnsignedByte);
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::UnsignedShort => XSD_UNSIGNED_SHORT,
			Self::UnsignedByte => XSD_UNSIGNED_BYTE,
		}
	}
	pub fn parse(&self, value: &str) -> Result<UnsignedShortValue, ParseError> {
		match self {
			Self::UnsignedShort => ParseRdf::parse_rdf(value)
				.map(UnsignedShortValue::UnsignedShort)
				.map_err(|_| ParseError),
			Self::UnsignedByte => ParseRdf::parse_rdf(value)
				.map(UnsignedShortValue::UnsignedByte)
				.map_err(|_| ParseError),
		}
	}
}
#[derive(Debug, Clone)]
pub enum UnsignedShortValue {
	UnsignedShort(UnsignedShort),
	UnsignedByte(UnsignedByte),
}
impl UnsignedShortValue {
	pub fn datatype(&self) -> UnsignedShortDatatype {
		match self {
			Self::UnsignedShort(_) => UnsignedShortDatatype::UnsignedShort,
			Self::UnsignedByte(_) => UnsignedShortDatatype::UnsignedByte,
		}
	}
}
impl XsdValue for UnsignedShortValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for UnsignedShortValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::UnsignedShort(v) => v.fmt(f),
			Self::UnsignedByte(v) => v.fmt(f),
		}
	}
}
#[derive(Debug, Clone)]
pub enum LongValue {
	Long(Long),
	Int(Int),
	Short(Short),
	Byte(Byte),
}
impl LongValue {
	pub fn datatype(&self) -> LongDatatype {
		match self {
			Self::Long(_) => LongDatatype::Long,
			Self::Int(_) => LongDatatype::Int(IntDatatype::Int),
			Self::Short(_) => LongDatatype::Int(IntDatatype::Short(ShortDatatype::Short)),
			Self::Byte(_) => LongDatatype::Int(IntDatatype::Short(ShortDatatype::Byte)),
		}
	}
}
impl XsdValue for LongValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for LongValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Long(v) => v.fmt(f),
			Self::Int(v) => v.fmt(f),
			Self::Short(v) => v.fmt(f),
			Self::Byte(v) => v.fmt(f),
		}
	}
}
impl From<IntValue> for LongValue {
	fn from(value: IntValue) -> Self {
		match value {
			IntValue::Int(value) => Self::Int(value),
			IntValue::Short(value) => Self::Short(value),
			IntValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl From<ShortValue> for LongValue {
	fn from(value: ShortValue) -> Self {
		match value {
			ShortValue::Short(value) => Self::Short(value),
			ShortValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl TryFrom<LongValue> for IntValue {
	type Error = LongValue;
	fn try_from(value: LongValue) -> Result<Self, LongValue> {
		match value {
			LongValue::Int(value) => Ok(Self::Int(value)),
			LongValue::Short(value) => Ok(Self::Short(value)),
			LongValue::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<LongValue> for ShortValue {
	type Error = LongValue;
	fn try_from(value: LongValue) -> Result<Self, LongValue> {
		match value {
			LongValue::Short(value) => Ok(Self::Short(value)),
			LongValue::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntDatatype {
	Int,
	Short(ShortDatatype),
}
impl IntDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_INT {
			return Some(Self::Int);
		}
		if let Some(t) = ShortDatatype::from_iri(iri) {
			return Some(Self::Short(t));
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Int => XSD_INT,
			Self::Short(t) => t.iri(),
		}
	}
	pub fn parse(&self, value: &str) -> Result<IntValue, ParseError> {
		match self {
			Self::Int => ParseRdf::parse_rdf(value)
				.map(IntValue::Int)
				.map_err(|_| ParseError),
			Self::Short(t) => t.parse(value).map(Into::into),
		}
	}
}
impl From<ShortDatatype> for IntDatatype {
	fn from(value: ShortDatatype) -> Self {
		Self::Short(value)
	}
}
impl TryFrom<IntDatatype> for ShortDatatype {
	type Error = IntDatatype;
	fn try_from(value: IntDatatype) -> Result<Self, IntDatatype> {
		match value {
			IntDatatype::Short(value) => Ok(value),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone)]
pub enum IntValue {
	Int(Int),
	Short(Short),
	Byte(Byte),
}
impl IntValue {
	pub fn datatype(&self) -> IntDatatype {
		match self {
			Self::Int(_) => IntDatatype::Int,
			Self::Short(_) => IntDatatype::Short(ShortDatatype::Short),
			Self::Byte(_) => IntDatatype::Short(ShortDatatype::Byte),
		}
	}
}
impl XsdValue for IntValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for IntValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Int(v) => v.fmt(f),
			Self::Short(v) => v.fmt(f),
			Self::Byte(v) => v.fmt(f),
		}
	}
}
impl From<ShortValue> for IntValue {
	fn from(value: ShortValue) -> Self {
		match value {
			ShortValue::Short(value) => Self::Short(value),
			ShortValue::Byte(value) => Self::Byte(value),
		}
	}
}
impl TryFrom<IntValue> for ShortValue {
	type Error = IntValue;
	fn try_from(value: IntValue) -> Result<Self, IntValue> {
		match value {
			IntValue::Short(value) => Ok(Self::Short(value)),
			IntValue::Byte(value) => Ok(Self::Byte(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShortDatatype {
	Short,
	Byte,
}
impl ShortDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_SHORT {
			return Some(Self::Short);
		}
		if iri == XSD_BYTE {
			return Some(Self::Byte);
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Short => XSD_SHORT,
			Self::Byte => XSD_BYTE,
		}
	}
	pub fn parse(&self, value: &str) -> Result<ShortValue, ParseError> {
		match self {
			Self::Short => ParseRdf::parse_rdf(value)
				.map(ShortValue::Short)
				.map_err(|_| ParseError),
			Self::Byte => ParseRdf::parse_rdf(value)
				.map(ShortValue::Byte)
				.map_err(|_| ParseError),
		}
	}
}
#[derive(Debug, Clone)]
pub enum ShortValue {
	Short(Short),
	Byte(Byte),
}
impl ShortValue {
	pub fn datatype(&self) -> ShortDatatype {
		match self {
			Self::Short(_) => ShortDatatype::Short,
			Self::Byte(_) => ShortDatatype::Byte,
		}
	}
}
impl XsdValue for ShortValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for ShortValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Short(v) => v.fmt(f),
			Self::Byte(v) => v.fmt(f),
		}
	}
}
impl From<StringDatatype> for Datatype {
	fn from(value: StringDatatype) -> Self {
		Self::String(value)
	}
}
impl From<NormalizedStringDatatype> for Datatype {
	fn from(value: NormalizedStringDatatype) -> Self {
		Self::String(StringDatatype::NormalizedString(value))
	}
}
impl From<TokenDatatype> for Datatype {
	fn from(value: TokenDatatype) -> Self {
		Self::String(StringDatatype::NormalizedString(
			NormalizedStringDatatype::Token(value),
		))
	}
}
impl From<NameDatatype> for Datatype {
	fn from(value: NameDatatype) -> Self {
		Self::String(StringDatatype::NormalizedString(
			NormalizedStringDatatype::Token(TokenDatatype::Name(value)),
		))
	}
}
impl From<NCNameDatatype> for Datatype {
	fn from(value: NCNameDatatype) -> Self {
		Self::String(StringDatatype::NormalizedString(
			NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::NCName(value))),
		))
	}
}
impl TryFrom<Datatype> for StringDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::String(value) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for NormalizedStringDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::String(StringDatatype::NormalizedString(value)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for TokenDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(value),
			)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for NameDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Name(value)),
			)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<Datatype> for NCNameDatatype {
	type Error = Datatype;
	fn try_from(value: Datatype) -> Result<Self, Datatype> {
		match value {
			Datatype::String(StringDatatype::NormalizedString(
				NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::NCName(value))),
			)) => Ok(value),
			other => Err(other),
		}
	}
}
impl From<StringValue> for Value {
	fn from(value: StringValue) -> Self {
		match value {
			StringValue::String(value) => Self::String(value),
			StringValue::NormalizedString(value) => Self::NormalizedString(value),
			StringValue::Token(value) => Self::Token(value),
			StringValue::Language(value) => Self::Language(value),
			StringValue::Name(value) => Self::Name(value),
			StringValue::NCName(value) => Self::NCName(value),
			StringValue::Id(value) => Self::Id(value),
			StringValue::IdRef(value) => Self::IdRef(value),
			StringValue::NMToken(value) => Self::NMToken(value),
		}
	}
}
impl From<NormalizedStringValue> for Value {
	fn from(value: NormalizedStringValue) -> Self {
		match value {
			NormalizedStringValue::NormalizedString(value) => Self::NormalizedString(value),
			NormalizedStringValue::Token(value) => Self::Token(value),
			NormalizedStringValue::Language(value) => Self::Language(value),
			NormalizedStringValue::Name(value) => Self::Name(value),
			NormalizedStringValue::NCName(value) => Self::NCName(value),
			NormalizedStringValue::Id(value) => Self::Id(value),
			NormalizedStringValue::IdRef(value) => Self::IdRef(value),
			NormalizedStringValue::NMToken(value) => Self::NMToken(value),
		}
	}
}
impl From<TokenValue> for Value {
	fn from(value: TokenValue) -> Self {
		match value {
			TokenValue::Token(value) => Self::Token(value),
			TokenValue::Language(value) => Self::Language(value),
			TokenValue::Name(value) => Self::Name(value),
			TokenValue::NCName(value) => Self::NCName(value),
			TokenValue::Id(value) => Self::Id(value),
			TokenValue::IdRef(value) => Self::IdRef(value),
			TokenValue::NMToken(value) => Self::NMToken(value),
		}
	}
}
impl From<NameValue> for Value {
	fn from(value: NameValue) -> Self {
		match value {
			NameValue::Name(value) => Self::Name(value),
			NameValue::NCName(value) => Self::NCName(value),
			NameValue::Id(value) => Self::Id(value),
			NameValue::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl From<NCNameValue> for Value {
	fn from(value: NCNameValue) -> Self {
		match value {
			NCNameValue::NCName(value) => Self::NCName(value),
			NCNameValue::Id(value) => Self::Id(value),
			NCNameValue::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl TryFrom<Value> for StringValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::String(value) => Ok(Self::String(value)),
			Value::NormalizedString(value) => Ok(Self::NormalizedString(value)),
			Value::Token(value) => Ok(Self::Token(value)),
			Value::Language(value) => Ok(Self::Language(value)),
			Value::Name(value) => Ok(Self::Name(value)),
			Value::NCName(value) => Ok(Self::NCName(value)),
			Value::Id(value) => Ok(Self::Id(value)),
			Value::IdRef(value) => Ok(Self::IdRef(value)),
			Value::NMToken(value) => Ok(Self::NMToken(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for NormalizedStringValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::NormalizedString(value) => Ok(Self::NormalizedString(value)),
			Value::Token(value) => Ok(Self::Token(value)),
			Value::Language(value) => Ok(Self::Language(value)),
			Value::Name(value) => Ok(Self::Name(value)),
			Value::NCName(value) => Ok(Self::NCName(value)),
			Value::Id(value) => Ok(Self::Id(value)),
			Value::IdRef(value) => Ok(Self::IdRef(value)),
			Value::NMToken(value) => Ok(Self::NMToken(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for TokenValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::Token(value) => Ok(Self::Token(value)),
			Value::Language(value) => Ok(Self::Language(value)),
			Value::Name(value) => Ok(Self::Name(value)),
			Value::NCName(value) => Ok(Self::NCName(value)),
			Value::Id(value) => Ok(Self::Id(value)),
			Value::IdRef(value) => Ok(Self::IdRef(value)),
			Value::NMToken(value) => Ok(Self::NMToken(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for NameValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::Name(value) => Ok(Self::Name(value)),
			Value::NCName(value) => Ok(Self::NCName(value)),
			Value::Id(value) => Ok(Self::Id(value)),
			Value::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<Value> for NCNameValue {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Value> {
		match value {
			Value::NCName(value) => Ok(Self::NCName(value)),
			Value::Id(value) => Ok(Self::Id(value)),
			Value::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
impl<'a> From<StringValueRef<'a>> for ValueRef<'a> {
	fn from(value: StringValueRef<'a>) -> Self {
		match value {
			StringValueRef::String(value) => Self::String(value),
			StringValueRef::NormalizedString(value) => Self::NormalizedString(value),
			StringValueRef::Token(value) => Self::Token(value),
			StringValueRef::Language(value) => Self::Language(value),
			StringValueRef::Name(value) => Self::Name(value),
			StringValueRef::NCName(value) => Self::NCName(value),
			StringValueRef::Id(value) => Self::Id(value),
			StringValueRef::IdRef(value) => Self::IdRef(value),
			StringValueRef::NMToken(value) => Self::NMToken(value),
		}
	}
}
impl<'a> From<NormalizedStringValueRef<'a>> for ValueRef<'a> {
	fn from(value: NormalizedStringValueRef<'a>) -> Self {
		match value {
			NormalizedStringValueRef::NormalizedString(value) => Self::NormalizedString(value),
			NormalizedStringValueRef::Token(value) => Self::Token(value),
			NormalizedStringValueRef::Language(value) => Self::Language(value),
			NormalizedStringValueRef::Name(value) => Self::Name(value),
			NormalizedStringValueRef::NCName(value) => Self::NCName(value),
			NormalizedStringValueRef::Id(value) => Self::Id(value),
			NormalizedStringValueRef::IdRef(value) => Self::IdRef(value),
			NormalizedStringValueRef::NMToken(value) => Self::NMToken(value),
		}
	}
}
impl<'a> From<TokenValueRef<'a>> for ValueRef<'a> {
	fn from(value: TokenValueRef<'a>) -> Self {
		match value {
			TokenValueRef::Token(value) => Self::Token(value),
			TokenValueRef::Language(value) => Self::Language(value),
			TokenValueRef::Name(value) => Self::Name(value),
			TokenValueRef::NCName(value) => Self::NCName(value),
			TokenValueRef::Id(value) => Self::Id(value),
			TokenValueRef::IdRef(value) => Self::IdRef(value),
			TokenValueRef::NMToken(value) => Self::NMToken(value),
		}
	}
}
impl<'a> From<NameValueRef<'a>> for ValueRef<'a> {
	fn from(value: NameValueRef<'a>) -> Self {
		match value {
			NameValueRef::Name(value) => Self::Name(value),
			NameValueRef::NCName(value) => Self::NCName(value),
			NameValueRef::Id(value) => Self::Id(value),
			NameValueRef::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl<'a> From<NCNameValueRef<'a>> for ValueRef<'a> {
	fn from(value: NCNameValueRef<'a>) -> Self {
		match value {
			NCNameValueRef::NCName(value) => Self::NCName(value),
			NCNameValueRef::Id(value) => Self::Id(value),
			NCNameValueRef::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl<'a> TryFrom<ValueRef<'a>> for StringValueRef<'a> {
	type Error = ValueRef<'a>;
	fn try_from(value: ValueRef<'a>) -> Result<Self, ValueRef<'a>> {
		match value {
			ValueRef::String(value) => Ok(Self::String(value)),
			ValueRef::NormalizedString(value) => Ok(Self::NormalizedString(value)),
			ValueRef::Token(value) => Ok(Self::Token(value)),
			ValueRef::Language(value) => Ok(Self::Language(value)),
			ValueRef::Name(value) => Ok(Self::Name(value)),
			ValueRef::NCName(value) => Ok(Self::NCName(value)),
			ValueRef::Id(value) => Ok(Self::Id(value)),
			ValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			ValueRef::NMToken(value) => Ok(Self::NMToken(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<ValueRef<'a>> for NormalizedStringValueRef<'a> {
	type Error = ValueRef<'a>;
	fn try_from(value: ValueRef<'a>) -> Result<Self, ValueRef<'a>> {
		match value {
			ValueRef::NormalizedString(value) => Ok(Self::NormalizedString(value)),
			ValueRef::Token(value) => Ok(Self::Token(value)),
			ValueRef::Language(value) => Ok(Self::Language(value)),
			ValueRef::Name(value) => Ok(Self::Name(value)),
			ValueRef::NCName(value) => Ok(Self::NCName(value)),
			ValueRef::Id(value) => Ok(Self::Id(value)),
			ValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			ValueRef::NMToken(value) => Ok(Self::NMToken(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<ValueRef<'a>> for TokenValueRef<'a> {
	type Error = ValueRef<'a>;
	fn try_from(value: ValueRef<'a>) -> Result<Self, ValueRef<'a>> {
		match value {
			ValueRef::Token(value) => Ok(Self::Token(value)),
			ValueRef::Language(value) => Ok(Self::Language(value)),
			ValueRef::Name(value) => Ok(Self::Name(value)),
			ValueRef::NCName(value) => Ok(Self::NCName(value)),
			ValueRef::Id(value) => Ok(Self::Id(value)),
			ValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			ValueRef::NMToken(value) => Ok(Self::NMToken(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<ValueRef<'a>> for NameValueRef<'a> {
	type Error = ValueRef<'a>;
	fn try_from(value: ValueRef<'a>) -> Result<Self, ValueRef<'a>> {
		match value {
			ValueRef::Name(value) => Ok(Self::Name(value)),
			ValueRef::NCName(value) => Ok(Self::NCName(value)),
			ValueRef::Id(value) => Ok(Self::Id(value)),
			ValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<ValueRef<'a>> for NCNameValueRef<'a> {
	type Error = ValueRef<'a>;
	fn try_from(value: ValueRef<'a>) -> Result<Self, ValueRef<'a>> {
		match value {
			ValueRef::NCName(value) => Ok(Self::NCName(value)),
			ValueRef::Id(value) => Ok(Self::Id(value)),
			ValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone)]
pub enum StringValue {
	String(String),
	NormalizedString(NormalizedString),
	Token(TokenBuf),
	Language(LanguageBuf),
	Name(NameBuf),
	NCName(NCNameBuf),
	Id(IdBuf),
	IdRef(IdRefBuf),
	NMToken(NMTokenBuf),
}
impl StringValue {
	pub fn datatype(&self) -> StringDatatype {
		match self {
			Self::String(_) => StringDatatype::String,
			Self::NormalizedString(_) => {
				StringDatatype::NormalizedString(NormalizedStringDatatype::NormalizedString)
			}
			Self::Token(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Token,
			)),
			Self::Language(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Language,
			)),
			Self::Name(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Name(NameDatatype::Name),
			)),
			Self::NCName(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Name(NameDatatype::NCName(NCNameDatatype::NCName)),
			)),
			Self::Id(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Name(NameDatatype::NCName(NCNameDatatype::Id)),
			)),
			Self::IdRef(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Name(NameDatatype::NCName(NCNameDatatype::IdRef)),
			)),
			Self::NMToken(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::NMToken,
			)),
		}
	}
}
impl XsdValue for StringValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for StringValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(v) => v.fmt(f),
			Self::NormalizedString(v) => v.fmt(f),
			Self::Token(v) => v.fmt(f),
			Self::Language(v) => v.fmt(f),
			Self::Name(v) => v.fmt(f),
			Self::NCName(v) => v.fmt(f),
			Self::Id(v) => v.fmt(f),
			Self::IdRef(v) => v.fmt(f),
			Self::NMToken(v) => v.fmt(f),
		}
	}
}
impl From<NormalizedStringValue> for StringValue {
	fn from(value: NormalizedStringValue) -> Self {
		match value {
			NormalizedStringValue::NormalizedString(value) => Self::NormalizedString(value),
			NormalizedStringValue::Token(value) => Self::Token(value),
			NormalizedStringValue::Language(value) => Self::Language(value),
			NormalizedStringValue::Name(value) => Self::Name(value),
			NormalizedStringValue::NCName(value) => Self::NCName(value),
			NormalizedStringValue::Id(value) => Self::Id(value),
			NormalizedStringValue::IdRef(value) => Self::IdRef(value),
			NormalizedStringValue::NMToken(value) => Self::NMToken(value),
		}
	}
}
impl From<TokenValue> for StringValue {
	fn from(value: TokenValue) -> Self {
		match value {
			TokenValue::Token(value) => Self::Token(value),
			TokenValue::Language(value) => Self::Language(value),
			TokenValue::Name(value) => Self::Name(value),
			TokenValue::NCName(value) => Self::NCName(value),
			TokenValue::Id(value) => Self::Id(value),
			TokenValue::IdRef(value) => Self::IdRef(value),
			TokenValue::NMToken(value) => Self::NMToken(value),
		}
	}
}
impl From<NameValue> for StringValue {
	fn from(value: NameValue) -> Self {
		match value {
			NameValue::Name(value) => Self::Name(value),
			NameValue::NCName(value) => Self::NCName(value),
			NameValue::Id(value) => Self::Id(value),
			NameValue::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl From<NCNameValue> for StringValue {
	fn from(value: NCNameValue) -> Self {
		match value {
			NCNameValue::NCName(value) => Self::NCName(value),
			NCNameValue::Id(value) => Self::Id(value),
			NCNameValue::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl TryFrom<StringValue> for NormalizedStringValue {
	type Error = StringValue;
	fn try_from(value: StringValue) -> Result<Self, StringValue> {
		match value {
			StringValue::NormalizedString(value) => Ok(Self::NormalizedString(value)),
			StringValue::Token(value) => Ok(Self::Token(value)),
			StringValue::Language(value) => Ok(Self::Language(value)),
			StringValue::Name(value) => Ok(Self::Name(value)),
			StringValue::NCName(value) => Ok(Self::NCName(value)),
			StringValue::Id(value) => Ok(Self::Id(value)),
			StringValue::IdRef(value) => Ok(Self::IdRef(value)),
			StringValue::NMToken(value) => Ok(Self::NMToken(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<StringValue> for TokenValue {
	type Error = StringValue;
	fn try_from(value: StringValue) -> Result<Self, StringValue> {
		match value {
			StringValue::Token(value) => Ok(Self::Token(value)),
			StringValue::Language(value) => Ok(Self::Language(value)),
			StringValue::Name(value) => Ok(Self::Name(value)),
			StringValue::NCName(value) => Ok(Self::NCName(value)),
			StringValue::Id(value) => Ok(Self::Id(value)),
			StringValue::IdRef(value) => Ok(Self::IdRef(value)),
			StringValue::NMToken(value) => Ok(Self::NMToken(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<StringValue> for NameValue {
	type Error = StringValue;
	fn try_from(value: StringValue) -> Result<Self, StringValue> {
		match value {
			StringValue::Name(value) => Ok(Self::Name(value)),
			StringValue::NCName(value) => Ok(Self::NCName(value)),
			StringValue::Id(value) => Ok(Self::Id(value)),
			StringValue::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<StringValue> for NCNameValue {
	type Error = StringValue;
	fn try_from(value: StringValue) -> Result<Self, StringValue> {
		match value {
			StringValue::NCName(value) => Ok(Self::NCName(value)),
			StringValue::Id(value) => Ok(Self::Id(value)),
			StringValue::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NormalizedStringDatatype {
	NormalizedString,
	Token(TokenDatatype),
}
impl NormalizedStringDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_NORMALIZED_STRING {
			return Some(Self::NormalizedString);
		}
		if let Some(t) = TokenDatatype::from_iri(iri) {
			return Some(Self::Token(t));
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::NormalizedString => XSD_NORMALIZED_STRING,
			Self::Token(t) => t.iri(),
		}
	}
	pub fn parse(&self, value: &str) -> Result<NormalizedStringValue, ParseError> {
		match self {
			Self::NormalizedString => ParseRdf::parse_rdf(value)
				.map(NormalizedStringValue::NormalizedString)
				.map_err(|_| ParseError),
			Self::Token(t) => t.parse(value).map(Into::into),
		}
	}
}
impl From<TokenDatatype> for NormalizedStringDatatype {
	fn from(value: TokenDatatype) -> Self {
		Self::Token(value)
	}
}
impl From<NameDatatype> for NormalizedStringDatatype {
	fn from(value: NameDatatype) -> Self {
		Self::Token(TokenDatatype::Name(value))
	}
}
impl From<NCNameDatatype> for NormalizedStringDatatype {
	fn from(value: NCNameDatatype) -> Self {
		Self::Token(TokenDatatype::Name(NameDatatype::NCName(value)))
	}
}
impl TryFrom<NormalizedStringDatatype> for TokenDatatype {
	type Error = NormalizedStringDatatype;
	fn try_from(value: NormalizedStringDatatype) -> Result<Self, NormalizedStringDatatype> {
		match value {
			NormalizedStringDatatype::Token(value) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<NormalizedStringDatatype> for NameDatatype {
	type Error = NormalizedStringDatatype;
	fn try_from(value: NormalizedStringDatatype) -> Result<Self, NormalizedStringDatatype> {
		match value {
			NormalizedStringDatatype::Token(TokenDatatype::Name(value)) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<NormalizedStringDatatype> for NCNameDatatype {
	type Error = NormalizedStringDatatype;
	fn try_from(value: NormalizedStringDatatype) -> Result<Self, NormalizedStringDatatype> {
		match value {
			NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::NCName(value))) => {
				Ok(value)
			}
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub enum StringValueRef<'a> {
	String(&'a str),
	NormalizedString(&'a NormalizedStr),
	Token(&'a Token),
	Language(&'a Language),
	Name(&'a Name),
	NCName(&'a NCName),
	Id(&'a Id),
	IdRef(&'a IdRef),
	NMToken(&'a NMToken),
}
impl StringValue {
	pub fn as_ref(&self) -> StringValueRef {
		match self {
			Self::String(value) => StringValueRef::String(value),
			Self::NormalizedString(value) => StringValueRef::NormalizedString(value),
			Self::Token(value) => StringValueRef::Token(value),
			Self::Language(value) => StringValueRef::Language(value),
			Self::Name(value) => StringValueRef::Name(value),
			Self::NCName(value) => StringValueRef::NCName(value),
			Self::Id(value) => StringValueRef::Id(value),
			Self::IdRef(value) => StringValueRef::IdRef(value),
			Self::NMToken(value) => StringValueRef::NMToken(value),
		}
	}
}
impl<'a> StringValueRef<'a> {
	pub fn datatype(&self) -> StringDatatype {
		match self {
			Self::String(_) => StringDatatype::String,
			Self::NormalizedString(_) => {
				StringDatatype::NormalizedString(NormalizedStringDatatype::NormalizedString)
			}
			Self::Token(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Token,
			)),
			Self::Language(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Language,
			)),
			Self::Name(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Name(NameDatatype::Name),
			)),
			Self::NCName(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Name(NameDatatype::NCName(NCNameDatatype::NCName)),
			)),
			Self::Id(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Name(NameDatatype::NCName(NCNameDatatype::Id)),
			)),
			Self::IdRef(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::Name(NameDatatype::NCName(NCNameDatatype::IdRef)),
			)),
			Self::NMToken(_) => StringDatatype::NormalizedString(NormalizedStringDatatype::Token(
				TokenDatatype::NMToken,
			)),
		}
	}
	pub fn cloned(&self) -> StringValue {
		match *self {
			Self::String(value) => StringValue::String(value.to_owned()),
			Self::NormalizedString(value) => StringValue::NormalizedString(value.to_owned()),
			Self::Token(value) => StringValue::Token(value.to_owned()),
			Self::Language(value) => StringValue::Language(value.to_owned()),
			Self::Name(value) => StringValue::Name(value.to_owned()),
			Self::NCName(value) => StringValue::NCName(value.to_owned()),
			Self::Id(value) => StringValue::Id(value.to_owned()),
			Self::IdRef(value) => StringValue::IdRef(value.to_owned()),
			Self::NMToken(value) => StringValue::NMToken(value.to_owned()),
		}
	}
}
impl<'a> XsdValue for StringValueRef<'a> {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl<'a> fmt::Display for StringValueRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(v) => v.fmt(f),
			Self::NormalizedString(v) => v.fmt(f),
			Self::Token(v) => v.fmt(f),
			Self::Language(v) => v.fmt(f),
			Self::Name(v) => v.fmt(f),
			Self::NCName(v) => v.fmt(f),
			Self::Id(v) => v.fmt(f),
			Self::IdRef(v) => v.fmt(f),
			Self::NMToken(v) => v.fmt(f),
		}
	}
}
impl<'a> From<NormalizedStringValueRef<'a>> for StringValueRef<'a> {
	fn from(value: NormalizedStringValueRef<'a>) -> Self {
		match value {
			NormalizedStringValueRef::NormalizedString(value) => Self::NormalizedString(value),
			NormalizedStringValueRef::Token(value) => Self::Token(value),
			NormalizedStringValueRef::Language(value) => Self::Language(value),
			NormalizedStringValueRef::Name(value) => Self::Name(value),
			NormalizedStringValueRef::NCName(value) => Self::NCName(value),
			NormalizedStringValueRef::Id(value) => Self::Id(value),
			NormalizedStringValueRef::IdRef(value) => Self::IdRef(value),
			NormalizedStringValueRef::NMToken(value) => Self::NMToken(value),
		}
	}
}
impl<'a> From<TokenValueRef<'a>> for StringValueRef<'a> {
	fn from(value: TokenValueRef<'a>) -> Self {
		match value {
			TokenValueRef::Token(value) => Self::Token(value),
			TokenValueRef::Language(value) => Self::Language(value),
			TokenValueRef::Name(value) => Self::Name(value),
			TokenValueRef::NCName(value) => Self::NCName(value),
			TokenValueRef::Id(value) => Self::Id(value),
			TokenValueRef::IdRef(value) => Self::IdRef(value),
			TokenValueRef::NMToken(value) => Self::NMToken(value),
		}
	}
}
impl<'a> From<NameValueRef<'a>> for StringValueRef<'a> {
	fn from(value: NameValueRef<'a>) -> Self {
		match value {
			NameValueRef::Name(value) => Self::Name(value),
			NameValueRef::NCName(value) => Self::NCName(value),
			NameValueRef::Id(value) => Self::Id(value),
			NameValueRef::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl<'a> From<NCNameValueRef<'a>> for StringValueRef<'a> {
	fn from(value: NCNameValueRef<'a>) -> Self {
		match value {
			NCNameValueRef::NCName(value) => Self::NCName(value),
			NCNameValueRef::Id(value) => Self::Id(value),
			NCNameValueRef::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl<'a> TryFrom<StringValueRef<'a>> for NormalizedStringValueRef<'a> {
	type Error = StringValueRef<'a>;
	fn try_from(value: StringValueRef<'a>) -> Result<Self, StringValueRef<'a>> {
		match value {
			StringValueRef::NormalizedString(value) => Ok(Self::NormalizedString(value)),
			StringValueRef::Token(value) => Ok(Self::Token(value)),
			StringValueRef::Language(value) => Ok(Self::Language(value)),
			StringValueRef::Name(value) => Ok(Self::Name(value)),
			StringValueRef::NCName(value) => Ok(Self::NCName(value)),
			StringValueRef::Id(value) => Ok(Self::Id(value)),
			StringValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			StringValueRef::NMToken(value) => Ok(Self::NMToken(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<StringValueRef<'a>> for TokenValueRef<'a> {
	type Error = StringValueRef<'a>;
	fn try_from(value: StringValueRef<'a>) -> Result<Self, StringValueRef<'a>> {
		match value {
			StringValueRef::Token(value) => Ok(Self::Token(value)),
			StringValueRef::Language(value) => Ok(Self::Language(value)),
			StringValueRef::Name(value) => Ok(Self::Name(value)),
			StringValueRef::NCName(value) => Ok(Self::NCName(value)),
			StringValueRef::Id(value) => Ok(Self::Id(value)),
			StringValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			StringValueRef::NMToken(value) => Ok(Self::NMToken(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<StringValueRef<'a>> for NameValueRef<'a> {
	type Error = StringValueRef<'a>;
	fn try_from(value: StringValueRef<'a>) -> Result<Self, StringValueRef<'a>> {
		match value {
			StringValueRef::Name(value) => Ok(Self::Name(value)),
			StringValueRef::NCName(value) => Ok(Self::NCName(value)),
			StringValueRef::Id(value) => Ok(Self::Id(value)),
			StringValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<StringValueRef<'a>> for NCNameValueRef<'a> {
	type Error = StringValueRef<'a>;
	fn try_from(value: StringValueRef<'a>) -> Result<Self, StringValueRef<'a>> {
		match value {
			StringValueRef::NCName(value) => Ok(Self::NCName(value)),
			StringValueRef::Id(value) => Ok(Self::Id(value)),
			StringValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone)]
pub enum NormalizedStringValue {
	NormalizedString(NormalizedString),
	Token(TokenBuf),
	Language(LanguageBuf),
	Name(NameBuf),
	NCName(NCNameBuf),
	Id(IdBuf),
	IdRef(IdRefBuf),
	NMToken(NMTokenBuf),
}
impl NormalizedStringValue {
	pub fn datatype(&self) -> NormalizedStringDatatype {
		match self {
			Self::NormalizedString(_) => NormalizedStringDatatype::NormalizedString,
			Self::Token(_) => NormalizedStringDatatype::Token(TokenDatatype::Token),
			Self::Language(_) => NormalizedStringDatatype::Token(TokenDatatype::Language),
			Self::Name(_) => {
				NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::Name))
			}
			Self::NCName(_) => NormalizedStringDatatype::Token(TokenDatatype::Name(
				NameDatatype::NCName(NCNameDatatype::NCName),
			)),
			Self::Id(_) => NormalizedStringDatatype::Token(TokenDatatype::Name(
				NameDatatype::NCName(NCNameDatatype::Id),
			)),
			Self::IdRef(_) => NormalizedStringDatatype::Token(TokenDatatype::Name(
				NameDatatype::NCName(NCNameDatatype::IdRef),
			)),
			Self::NMToken(_) => NormalizedStringDatatype::Token(TokenDatatype::NMToken),
		}
	}
}
impl XsdValue for NormalizedStringValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for NormalizedStringValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NormalizedString(v) => v.fmt(f),
			Self::Token(v) => v.fmt(f),
			Self::Language(v) => v.fmt(f),
			Self::Name(v) => v.fmt(f),
			Self::NCName(v) => v.fmt(f),
			Self::Id(v) => v.fmt(f),
			Self::IdRef(v) => v.fmt(f),
			Self::NMToken(v) => v.fmt(f),
		}
	}
}
impl From<TokenValue> for NormalizedStringValue {
	fn from(value: TokenValue) -> Self {
		match value {
			TokenValue::Token(value) => Self::Token(value),
			TokenValue::Language(value) => Self::Language(value),
			TokenValue::Name(value) => Self::Name(value),
			TokenValue::NCName(value) => Self::NCName(value),
			TokenValue::Id(value) => Self::Id(value),
			TokenValue::IdRef(value) => Self::IdRef(value),
			TokenValue::NMToken(value) => Self::NMToken(value),
		}
	}
}
impl From<NameValue> for NormalizedStringValue {
	fn from(value: NameValue) -> Self {
		match value {
			NameValue::Name(value) => Self::Name(value),
			NameValue::NCName(value) => Self::NCName(value),
			NameValue::Id(value) => Self::Id(value),
			NameValue::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl From<NCNameValue> for NormalizedStringValue {
	fn from(value: NCNameValue) -> Self {
		match value {
			NCNameValue::NCName(value) => Self::NCName(value),
			NCNameValue::Id(value) => Self::Id(value),
			NCNameValue::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl TryFrom<NormalizedStringValue> for TokenValue {
	type Error = NormalizedStringValue;
	fn try_from(value: NormalizedStringValue) -> Result<Self, NormalizedStringValue> {
		match value {
			NormalizedStringValue::Token(value) => Ok(Self::Token(value)),
			NormalizedStringValue::Language(value) => Ok(Self::Language(value)),
			NormalizedStringValue::Name(value) => Ok(Self::Name(value)),
			NormalizedStringValue::NCName(value) => Ok(Self::NCName(value)),
			NormalizedStringValue::Id(value) => Ok(Self::Id(value)),
			NormalizedStringValue::IdRef(value) => Ok(Self::IdRef(value)),
			NormalizedStringValue::NMToken(value) => Ok(Self::NMToken(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<NormalizedStringValue> for NameValue {
	type Error = NormalizedStringValue;
	fn try_from(value: NormalizedStringValue) -> Result<Self, NormalizedStringValue> {
		match value {
			NormalizedStringValue::Name(value) => Ok(Self::Name(value)),
			NormalizedStringValue::NCName(value) => Ok(Self::NCName(value)),
			NormalizedStringValue::Id(value) => Ok(Self::Id(value)),
			NormalizedStringValue::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<NormalizedStringValue> for NCNameValue {
	type Error = NormalizedStringValue;
	fn try_from(value: NormalizedStringValue) -> Result<Self, NormalizedStringValue> {
		match value {
			NormalizedStringValue::NCName(value) => Ok(Self::NCName(value)),
			NormalizedStringValue::Id(value) => Ok(Self::Id(value)),
			NormalizedStringValue::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenDatatype {
	Token,
	Language,
	Name(NameDatatype),
	NMToken,
}
impl TokenDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_TOKEN {
			return Some(Self::Token);
		}
		if iri == XSD_LANGUAGE {
			return Some(Self::Language);
		}
		if let Some(t) = NameDatatype::from_iri(iri) {
			return Some(Self::Name(t));
		}
		if iri == XSD_NMTOKEN {
			return Some(Self::NMToken);
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Token => XSD_TOKEN,
			Self::Language => XSD_LANGUAGE,
			Self::Name(t) => t.iri(),
			Self::NMToken => XSD_NMTOKEN,
		}
	}
	pub fn parse(&self, value: &str) -> Result<TokenValue, ParseError> {
		match self {
			Self::Token => ParseRdf::parse_rdf(value)
				.map(TokenValue::Token)
				.map_err(|_| ParseError),
			Self::Language => ParseRdf::parse_rdf(value)
				.map(TokenValue::Language)
				.map_err(|_| ParseError),
			Self::Name(t) => t.parse(value).map(Into::into),
			Self::NMToken => ParseRdf::parse_rdf(value)
				.map(TokenValue::NMToken)
				.map_err(|_| ParseError),
		}
	}
}
impl From<NameDatatype> for TokenDatatype {
	fn from(value: NameDatatype) -> Self {
		Self::Name(value)
	}
}
impl From<NCNameDatatype> for TokenDatatype {
	fn from(value: NCNameDatatype) -> Self {
		Self::Name(NameDatatype::NCName(value))
	}
}
impl TryFrom<TokenDatatype> for NameDatatype {
	type Error = TokenDatatype;
	fn try_from(value: TokenDatatype) -> Result<Self, TokenDatatype> {
		match value {
			TokenDatatype::Name(value) => Ok(value),
			other => Err(other),
		}
	}
}
impl TryFrom<TokenDatatype> for NCNameDatatype {
	type Error = TokenDatatype;
	fn try_from(value: TokenDatatype) -> Result<Self, TokenDatatype> {
		match value {
			TokenDatatype::Name(NameDatatype::NCName(value)) => Ok(value),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub enum NormalizedStringValueRef<'a> {
	NormalizedString(&'a NormalizedStr),
	Token(&'a Token),
	Language(&'a Language),
	Name(&'a Name),
	NCName(&'a NCName),
	Id(&'a Id),
	IdRef(&'a IdRef),
	NMToken(&'a NMToken),
}
impl NormalizedStringValue {
	pub fn as_ref(&self) -> NormalizedStringValueRef {
		match self {
			Self::NormalizedString(value) => NormalizedStringValueRef::NormalizedString(value),
			Self::Token(value) => NormalizedStringValueRef::Token(value),
			Self::Language(value) => NormalizedStringValueRef::Language(value),
			Self::Name(value) => NormalizedStringValueRef::Name(value),
			Self::NCName(value) => NormalizedStringValueRef::NCName(value),
			Self::Id(value) => NormalizedStringValueRef::Id(value),
			Self::IdRef(value) => NormalizedStringValueRef::IdRef(value),
			Self::NMToken(value) => NormalizedStringValueRef::NMToken(value),
		}
	}
}
impl<'a> NormalizedStringValueRef<'a> {
	pub fn datatype(&self) -> NormalizedStringDatatype {
		match self {
			Self::NormalizedString(_) => NormalizedStringDatatype::NormalizedString,
			Self::Token(_) => NormalizedStringDatatype::Token(TokenDatatype::Token),
			Self::Language(_) => NormalizedStringDatatype::Token(TokenDatatype::Language),
			Self::Name(_) => {
				NormalizedStringDatatype::Token(TokenDatatype::Name(NameDatatype::Name))
			}
			Self::NCName(_) => NormalizedStringDatatype::Token(TokenDatatype::Name(
				NameDatatype::NCName(NCNameDatatype::NCName),
			)),
			Self::Id(_) => NormalizedStringDatatype::Token(TokenDatatype::Name(
				NameDatatype::NCName(NCNameDatatype::Id),
			)),
			Self::IdRef(_) => NormalizedStringDatatype::Token(TokenDatatype::Name(
				NameDatatype::NCName(NCNameDatatype::IdRef),
			)),
			Self::NMToken(_) => NormalizedStringDatatype::Token(TokenDatatype::NMToken),
		}
	}
	pub fn cloned(&self) -> NormalizedStringValue {
		match *self {
			Self::NormalizedString(value) => {
				NormalizedStringValue::NormalizedString(value.to_owned())
			}
			Self::Token(value) => NormalizedStringValue::Token(value.to_owned()),
			Self::Language(value) => NormalizedStringValue::Language(value.to_owned()),
			Self::Name(value) => NormalizedStringValue::Name(value.to_owned()),
			Self::NCName(value) => NormalizedStringValue::NCName(value.to_owned()),
			Self::Id(value) => NormalizedStringValue::Id(value.to_owned()),
			Self::IdRef(value) => NormalizedStringValue::IdRef(value.to_owned()),
			Self::NMToken(value) => NormalizedStringValue::NMToken(value.to_owned()),
		}
	}
}
impl<'a> XsdValue for NormalizedStringValueRef<'a> {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl<'a> fmt::Display for NormalizedStringValueRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NormalizedString(v) => v.fmt(f),
			Self::Token(v) => v.fmt(f),
			Self::Language(v) => v.fmt(f),
			Self::Name(v) => v.fmt(f),
			Self::NCName(v) => v.fmt(f),
			Self::Id(v) => v.fmt(f),
			Self::IdRef(v) => v.fmt(f),
			Self::NMToken(v) => v.fmt(f),
		}
	}
}
impl<'a> From<TokenValueRef<'a>> for NormalizedStringValueRef<'a> {
	fn from(value: TokenValueRef<'a>) -> Self {
		match value {
			TokenValueRef::Token(value) => Self::Token(value),
			TokenValueRef::Language(value) => Self::Language(value),
			TokenValueRef::Name(value) => Self::Name(value),
			TokenValueRef::NCName(value) => Self::NCName(value),
			TokenValueRef::Id(value) => Self::Id(value),
			TokenValueRef::IdRef(value) => Self::IdRef(value),
			TokenValueRef::NMToken(value) => Self::NMToken(value),
		}
	}
}
impl<'a> From<NameValueRef<'a>> for NormalizedStringValueRef<'a> {
	fn from(value: NameValueRef<'a>) -> Self {
		match value {
			NameValueRef::Name(value) => Self::Name(value),
			NameValueRef::NCName(value) => Self::NCName(value),
			NameValueRef::Id(value) => Self::Id(value),
			NameValueRef::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl<'a> From<NCNameValueRef<'a>> for NormalizedStringValueRef<'a> {
	fn from(value: NCNameValueRef<'a>) -> Self {
		match value {
			NCNameValueRef::NCName(value) => Self::NCName(value),
			NCNameValueRef::Id(value) => Self::Id(value),
			NCNameValueRef::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl<'a> TryFrom<NormalizedStringValueRef<'a>> for TokenValueRef<'a> {
	type Error = NormalizedStringValueRef<'a>;
	fn try_from(value: NormalizedStringValueRef<'a>) -> Result<Self, NormalizedStringValueRef<'a>> {
		match value {
			NormalizedStringValueRef::Token(value) => Ok(Self::Token(value)),
			NormalizedStringValueRef::Language(value) => Ok(Self::Language(value)),
			NormalizedStringValueRef::Name(value) => Ok(Self::Name(value)),
			NormalizedStringValueRef::NCName(value) => Ok(Self::NCName(value)),
			NormalizedStringValueRef::Id(value) => Ok(Self::Id(value)),
			NormalizedStringValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			NormalizedStringValueRef::NMToken(value) => Ok(Self::NMToken(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<NormalizedStringValueRef<'a>> for NameValueRef<'a> {
	type Error = NormalizedStringValueRef<'a>;
	fn try_from(value: NormalizedStringValueRef<'a>) -> Result<Self, NormalizedStringValueRef<'a>> {
		match value {
			NormalizedStringValueRef::Name(value) => Ok(Self::Name(value)),
			NormalizedStringValueRef::NCName(value) => Ok(Self::NCName(value)),
			NormalizedStringValueRef::Id(value) => Ok(Self::Id(value)),
			NormalizedStringValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<NormalizedStringValueRef<'a>> for NCNameValueRef<'a> {
	type Error = NormalizedStringValueRef<'a>;
	fn try_from(value: NormalizedStringValueRef<'a>) -> Result<Self, NormalizedStringValueRef<'a>> {
		match value {
			NormalizedStringValueRef::NCName(value) => Ok(Self::NCName(value)),
			NormalizedStringValueRef::Id(value) => Ok(Self::Id(value)),
			NormalizedStringValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone)]
pub enum TokenValue {
	Token(TokenBuf),
	Language(LanguageBuf),
	Name(NameBuf),
	NCName(NCNameBuf),
	Id(IdBuf),
	IdRef(IdRefBuf),
	NMToken(NMTokenBuf),
}
impl TokenValue {
	pub fn datatype(&self) -> TokenDatatype {
		match self {
			Self::Token(_) => TokenDatatype::Token,
			Self::Language(_) => TokenDatatype::Language,
			Self::Name(_) => TokenDatatype::Name(NameDatatype::Name),
			Self::NCName(_) => TokenDatatype::Name(NameDatatype::NCName(NCNameDatatype::NCName)),
			Self::Id(_) => TokenDatatype::Name(NameDatatype::NCName(NCNameDatatype::Id)),
			Self::IdRef(_) => TokenDatatype::Name(NameDatatype::NCName(NCNameDatatype::IdRef)),
			Self::NMToken(_) => TokenDatatype::NMToken,
		}
	}
}
impl XsdValue for TokenValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for TokenValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Token(v) => v.fmt(f),
			Self::Language(v) => v.fmt(f),
			Self::Name(v) => v.fmt(f),
			Self::NCName(v) => v.fmt(f),
			Self::Id(v) => v.fmt(f),
			Self::IdRef(v) => v.fmt(f),
			Self::NMToken(v) => v.fmt(f),
		}
	}
}
impl From<NameValue> for TokenValue {
	fn from(value: NameValue) -> Self {
		match value {
			NameValue::Name(value) => Self::Name(value),
			NameValue::NCName(value) => Self::NCName(value),
			NameValue::Id(value) => Self::Id(value),
			NameValue::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl From<NCNameValue> for TokenValue {
	fn from(value: NCNameValue) -> Self {
		match value {
			NCNameValue::NCName(value) => Self::NCName(value),
			NCNameValue::Id(value) => Self::Id(value),
			NCNameValue::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl TryFrom<TokenValue> for NameValue {
	type Error = TokenValue;
	fn try_from(value: TokenValue) -> Result<Self, TokenValue> {
		match value {
			TokenValue::Name(value) => Ok(Self::Name(value)),
			TokenValue::NCName(value) => Ok(Self::NCName(value)),
			TokenValue::Id(value) => Ok(Self::Id(value)),
			TokenValue::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
impl TryFrom<TokenValue> for NCNameValue {
	type Error = TokenValue;
	fn try_from(value: TokenValue) -> Result<Self, TokenValue> {
		match value {
			TokenValue::NCName(value) => Ok(Self::NCName(value)),
			TokenValue::Id(value) => Ok(Self::Id(value)),
			TokenValue::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NameDatatype {
	Name,
	NCName(NCNameDatatype),
}
impl NameDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_NAME {
			return Some(Self::Name);
		}
		if let Some(t) = NCNameDatatype::from_iri(iri) {
			return Some(Self::NCName(t));
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Name => XSD_NAME,
			Self::NCName(t) => t.iri(),
		}
	}
	pub fn parse(&self, value: &str) -> Result<NameValue, ParseError> {
		match self {
			Self::Name => ParseRdf::parse_rdf(value)
				.map(NameValue::Name)
				.map_err(|_| ParseError),
			Self::NCName(t) => t.parse(value).map(Into::into),
		}
	}
}
impl From<NCNameDatatype> for NameDatatype {
	fn from(value: NCNameDatatype) -> Self {
		Self::NCName(value)
	}
}
impl TryFrom<NameDatatype> for NCNameDatatype {
	type Error = NameDatatype;
	fn try_from(value: NameDatatype) -> Result<Self, NameDatatype> {
		match value {
			NameDatatype::NCName(value) => Ok(value),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub enum TokenValueRef<'a> {
	Token(&'a Token),
	Language(&'a Language),
	Name(&'a Name),
	NCName(&'a NCName),
	Id(&'a Id),
	IdRef(&'a IdRef),
	NMToken(&'a NMToken),
}
impl TokenValue {
	pub fn as_ref(&self) -> TokenValueRef {
		match self {
			Self::Token(value) => TokenValueRef::Token(value),
			Self::Language(value) => TokenValueRef::Language(value),
			Self::Name(value) => TokenValueRef::Name(value),
			Self::NCName(value) => TokenValueRef::NCName(value),
			Self::Id(value) => TokenValueRef::Id(value),
			Self::IdRef(value) => TokenValueRef::IdRef(value),
			Self::NMToken(value) => TokenValueRef::NMToken(value),
		}
	}
}
impl<'a> TokenValueRef<'a> {
	pub fn datatype(&self) -> TokenDatatype {
		match self {
			Self::Token(_) => TokenDatatype::Token,
			Self::Language(_) => TokenDatatype::Language,
			Self::Name(_) => TokenDatatype::Name(NameDatatype::Name),
			Self::NCName(_) => TokenDatatype::Name(NameDatatype::NCName(NCNameDatatype::NCName)),
			Self::Id(_) => TokenDatatype::Name(NameDatatype::NCName(NCNameDatatype::Id)),
			Self::IdRef(_) => TokenDatatype::Name(NameDatatype::NCName(NCNameDatatype::IdRef)),
			Self::NMToken(_) => TokenDatatype::NMToken,
		}
	}
	pub fn cloned(&self) -> TokenValue {
		match *self {
			Self::Token(value) => TokenValue::Token(value.to_owned()),
			Self::Language(value) => TokenValue::Language(value.to_owned()),
			Self::Name(value) => TokenValue::Name(value.to_owned()),
			Self::NCName(value) => TokenValue::NCName(value.to_owned()),
			Self::Id(value) => TokenValue::Id(value.to_owned()),
			Self::IdRef(value) => TokenValue::IdRef(value.to_owned()),
			Self::NMToken(value) => TokenValue::NMToken(value.to_owned()),
		}
	}
}
impl<'a> XsdValue for TokenValueRef<'a> {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl<'a> fmt::Display for TokenValueRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Token(v) => v.fmt(f),
			Self::Language(v) => v.fmt(f),
			Self::Name(v) => v.fmt(f),
			Self::NCName(v) => v.fmt(f),
			Self::Id(v) => v.fmt(f),
			Self::IdRef(v) => v.fmt(f),
			Self::NMToken(v) => v.fmt(f),
		}
	}
}
impl<'a> From<NameValueRef<'a>> for TokenValueRef<'a> {
	fn from(value: NameValueRef<'a>) -> Self {
		match value {
			NameValueRef::Name(value) => Self::Name(value),
			NameValueRef::NCName(value) => Self::NCName(value),
			NameValueRef::Id(value) => Self::Id(value),
			NameValueRef::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl<'a> From<NCNameValueRef<'a>> for TokenValueRef<'a> {
	fn from(value: NCNameValueRef<'a>) -> Self {
		match value {
			NCNameValueRef::NCName(value) => Self::NCName(value),
			NCNameValueRef::Id(value) => Self::Id(value),
			NCNameValueRef::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl<'a> TryFrom<TokenValueRef<'a>> for NameValueRef<'a> {
	type Error = TokenValueRef<'a>;
	fn try_from(value: TokenValueRef<'a>) -> Result<Self, TokenValueRef<'a>> {
		match value {
			TokenValueRef::Name(value) => Ok(Self::Name(value)),
			TokenValueRef::NCName(value) => Ok(Self::NCName(value)),
			TokenValueRef::Id(value) => Ok(Self::Id(value)),
			TokenValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
impl<'a> TryFrom<TokenValueRef<'a>> for NCNameValueRef<'a> {
	type Error = TokenValueRef<'a>;
	fn try_from(value: TokenValueRef<'a>) -> Result<Self, TokenValueRef<'a>> {
		match value {
			TokenValueRef::NCName(value) => Ok(Self::NCName(value)),
			TokenValueRef::Id(value) => Ok(Self::Id(value)),
			TokenValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone)]
pub enum NameValue {
	Name(NameBuf),
	NCName(NCNameBuf),
	Id(IdBuf),
	IdRef(IdRefBuf),
}
impl NameValue {
	pub fn datatype(&self) -> NameDatatype {
		match self {
			Self::Name(_) => NameDatatype::Name,
			Self::NCName(_) => NameDatatype::NCName(NCNameDatatype::NCName),
			Self::Id(_) => NameDatatype::NCName(NCNameDatatype::Id),
			Self::IdRef(_) => NameDatatype::NCName(NCNameDatatype::IdRef),
		}
	}
}
impl XsdValue for NameValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for NameValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Name(v) => v.fmt(f),
			Self::NCName(v) => v.fmt(f),
			Self::Id(v) => v.fmt(f),
			Self::IdRef(v) => v.fmt(f),
		}
	}
}
impl From<NCNameValue> for NameValue {
	fn from(value: NCNameValue) -> Self {
		match value {
			NCNameValue::NCName(value) => Self::NCName(value),
			NCNameValue::Id(value) => Self::Id(value),
			NCNameValue::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl TryFrom<NameValue> for NCNameValue {
	type Error = NameValue;
	fn try_from(value: NameValue) -> Result<Self, NameValue> {
		match value {
			NameValue::NCName(value) => Ok(Self::NCName(value)),
			NameValue::Id(value) => Ok(Self::Id(value)),
			NameValue::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NCNameDatatype {
	NCName,
	Id,
	IdRef,
}
impl NCNameDatatype {
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		if iri == XSD_NC_NAME {
			return Some(Self::NCName);
		}
		if iri == XSD_ID {
			return Some(Self::Id);
		}
		if iri == XSD_IDREF {
			return Some(Self::IdRef);
		}
		None
	}
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::NCName => XSD_NC_NAME,
			Self::Id => XSD_ID,
			Self::IdRef => XSD_IDREF,
		}
	}
	pub fn parse(&self, value: &str) -> Result<NCNameValue, ParseError> {
		match self {
			Self::NCName => ParseRdf::parse_rdf(value)
				.map(NCNameValue::NCName)
				.map_err(|_| ParseError),
			Self::Id => ParseRdf::parse_rdf(value)
				.map(NCNameValue::Id)
				.map_err(|_| ParseError),
			Self::IdRef => ParseRdf::parse_rdf(value)
				.map(NCNameValue::IdRef)
				.map_err(|_| ParseError),
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub enum NameValueRef<'a> {
	Name(&'a Name),
	NCName(&'a NCName),
	Id(&'a Id),
	IdRef(&'a IdRef),
}
impl NameValue {
	pub fn as_ref(&self) -> NameValueRef {
		match self {
			Self::Name(value) => NameValueRef::Name(value),
			Self::NCName(value) => NameValueRef::NCName(value),
			Self::Id(value) => NameValueRef::Id(value),
			Self::IdRef(value) => NameValueRef::IdRef(value),
		}
	}
}
impl<'a> NameValueRef<'a> {
	pub fn datatype(&self) -> NameDatatype {
		match self {
			Self::Name(_) => NameDatatype::Name,
			Self::NCName(_) => NameDatatype::NCName(NCNameDatatype::NCName),
			Self::Id(_) => NameDatatype::NCName(NCNameDatatype::Id),
			Self::IdRef(_) => NameDatatype::NCName(NCNameDatatype::IdRef),
		}
	}
	pub fn cloned(&self) -> NameValue {
		match *self {
			Self::Name(value) => NameValue::Name(value.to_owned()),
			Self::NCName(value) => NameValue::NCName(value.to_owned()),
			Self::Id(value) => NameValue::Id(value.to_owned()),
			Self::IdRef(value) => NameValue::IdRef(value.to_owned()),
		}
	}
}
impl<'a> XsdValue for NameValueRef<'a> {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl<'a> fmt::Display for NameValueRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Name(v) => v.fmt(f),
			Self::NCName(v) => v.fmt(f),
			Self::Id(v) => v.fmt(f),
			Self::IdRef(v) => v.fmt(f),
		}
	}
}
impl<'a> From<NCNameValueRef<'a>> for NameValueRef<'a> {
	fn from(value: NCNameValueRef<'a>) -> Self {
		match value {
			NCNameValueRef::NCName(value) => Self::NCName(value),
			NCNameValueRef::Id(value) => Self::Id(value),
			NCNameValueRef::IdRef(value) => Self::IdRef(value),
		}
	}
}
impl<'a> TryFrom<NameValueRef<'a>> for NCNameValueRef<'a> {
	type Error = NameValueRef<'a>;
	fn try_from(value: NameValueRef<'a>) -> Result<Self, NameValueRef<'a>> {
		match value {
			NameValueRef::NCName(value) => Ok(Self::NCName(value)),
			NameValueRef::Id(value) => Ok(Self::Id(value)),
			NameValueRef::IdRef(value) => Ok(Self::IdRef(value)),
			other => Err(other),
		}
	}
}
#[derive(Debug, Clone)]
pub enum NCNameValue {
	NCName(NCNameBuf),
	Id(IdBuf),
	IdRef(IdRefBuf),
}
impl NCNameValue {
	pub fn datatype(&self) -> NCNameDatatype {
		match self {
			Self::NCName(_) => NCNameDatatype::NCName,
			Self::Id(_) => NCNameDatatype::Id,
			Self::IdRef(_) => NCNameDatatype::IdRef,
		}
	}
}
impl XsdValue for NCNameValue {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl fmt::Display for NCNameValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NCName(v) => v.fmt(f),
			Self::Id(v) => v.fmt(f),
			Self::IdRef(v) => v.fmt(f),
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub enum NCNameValueRef<'a> {
	NCName(&'a NCName),
	Id(&'a Id),
	IdRef(&'a IdRef),
}
impl NCNameValue {
	pub fn as_ref(&self) -> NCNameValueRef {
		match self {
			Self::NCName(value) => NCNameValueRef::NCName(value),
			Self::Id(value) => NCNameValueRef::Id(value),
			Self::IdRef(value) => NCNameValueRef::IdRef(value),
		}
	}
}
impl<'a> NCNameValueRef<'a> {
	pub fn datatype(&self) -> NCNameDatatype {
		match self {
			Self::NCName(_) => NCNameDatatype::NCName,
			Self::Id(_) => NCNameDatatype::Id,
			Self::IdRef(_) => NCNameDatatype::IdRef,
		}
	}
	pub fn cloned(&self) -> NCNameValue {
		match *self {
			Self::NCName(value) => NCNameValue::NCName(value.to_owned()),
			Self::Id(value) => NCNameValue::Id(value.to_owned()),
			Self::IdRef(value) => NCNameValue::IdRef(value.to_owned()),
		}
	}
}
impl<'a> XsdValue for NCNameValueRef<'a> {
	fn datatype(&self) -> Datatype {
		self.datatype().into()
	}
}
impl<'a> fmt::Display for NCNameValueRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NCName(v) => v.fmt(f),
			Self::Id(v) => v.fmt(f),
			Self::IdRef(v) => v.fmt(f),
		}
	}
}
