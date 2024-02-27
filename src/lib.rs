//! This crate aims at providing safe representations
//! of [XSD built-in data types][xsd].
//!
//! [xsd]: <https://www.w3.org/TR/xmlschema-2/#built-in-datatypes>
//!
//! # Usage
//!
//! For each XSD datatype, this library provides a two families of types:
//! one representing the *lexical space* of the datatype (see the [`lexical`]
//! module), and one representing the *value space* of the datatype (see the
//! [`value`] module).
//!
//! For instance, assume we wish to store the lexical representation of an
//! [XSD decimal][xsd-decimal] datatype value. We can use the
//! [`lexical::Decimal`] type (or its owned variant, [`lexical::DecimalBuf`])
//!
//! [`lexical`]: crate::lexical
//! [`value`]: crate::value
//! [xsd-decimal]: <https://www.w3.org/TR/xmlschema11-2/#decimal>
//! [`lexical::Decimal`]: crate::lexical::Decimal
//! [`lexical::DecimalBuf`]: crate::lexical::DecimalBuf
//!
//! ```
//! let string = "3.141592653589793";
//!
//! // Parse the lexical representation (lexical domain).
//! let lexical_repr = xsd_types::lexical::Decimal::new(string).unwrap();
//!
//! // Interprets the lexical representation (value domain).
//! use xsd_types::lexical::LexicalFormOf;
//! let value_repr: xsd_types::Decimal = lexical_repr.try_as_value().unwrap();
//! ```
//!
//! Of course it is possible to parse the value directly into the value domain
//! using [`FromStr`](::core::str::FromStr):
//! ```
//! let value_repr: xsd_types::Decimal = "3.141592653589793".parse().unwrap();
//! ```
//!
//! ## Any value
//!
//! The [`Value`] type provides a simple way to represent *any* XSD value.
//!
//! ```
//! use xsd_types::{XSD_DATE, Datatype, Value};
//! let dt = Datatype::from_iri(XSD_DATE).unwrap();
//! let value: Value = dt.parse("1758-12-25").unwrap(); // Halley is back!
//! ```
//!
//! [`Value`]: crate::Value
use iref::Iri;
use static_iref::iri;

/// Lexical domain types.
pub mod lexical;
pub(crate) mod utils;

/// Value domain types.
pub mod value;

use lexical::{Lexical, LexicalFormOf};
pub use value::*;

mod types;

pub use types::*;

/// XSD primitive datatype.
pub enum PrimitiveDatatype {
	String,
	Boolean,
	Decimal,
	Float,
	Double,
	Duration,
	DateTime,
	Time,
	Date,
	GYearMonth,
	GYear,
	GMonthDay,
	GDay,
	GMonth,
	HexBinary,
	Base64Binary,
	AnyUri,
	QName,
	Notation,
}

/// <http://www.w3.org/2001/XMLSchema#duration> datatype IRI.
pub const XSD_DURATION: &Iri = iri!("http://www.w3.org/2001/XMLSchema#duration");

/// <http://www.w3.org/2001/XMLSchema#dateTime> datatype IRI.
pub const XSD_DATE_TIME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#dateTime");

/// <http://www.w3.org/2001/XMLSchema#time> datatype IRI.
pub const XSD_TIME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#time");

/// <http://www.w3.org/2001/XMLSchema#date> datatype IRI.
pub const XSD_DATE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#date");

/// <http://www.w3.org/2001/XMLSchema#gYearMonth> datatype IRI.
pub const XSD_G_YEAR_MONTH: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gYearMonth");

/// <http://www.w3.org/2001/XMLSchema#gYear> datatype IRI.
pub const XSD_G_YEAR: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gYear");

/// <http://www.w3.org/2001/XMLSchema#gMonthDay> datatype IRI.
pub const XSD_G_MONTH_DAY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gMonthDay");

/// <http://www.w3.org/2001/XMLSchema#gDay> datatype IRI.
pub const XSD_G_DAY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gDay");

/// <http://www.w3.org/2001/XMLSchema#gMonth> datatype IRI.
pub const XSD_G_MONTH: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gMonth");

/// <http://www.w3.org/2001/XMLSchema#string> datatype IRI.
pub const XSD_STRING: &Iri = iri!("http://www.w3.org/2001/XMLSchema#string");

/// <http://www.w3.org/2001/XMLSchema#boolean> datatype IRI.
pub const XSD_BOOLEAN: &Iri = iri!("http://www.w3.org/2001/XMLSchema#boolean");

/// <http://www.w3.org/2001/XMLSchema#base64Binary> datatype IRI.
pub const XSD_BASE64_BINARY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#base64Binary");

/// <http://www.w3.org/2001/XMLSchema#hexBinary> datatype IRI.
pub const XSD_HEX_BINARY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#hexBinary");

/// <http://www.w3.org/2001/XMLSchema#float> datatype IRI.
pub const XSD_FLOAT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#float");

/// <http://www.w3.org/2001/XMLSchema#decimal> datatype IRI.
pub const XSD_DECIMAL: &Iri = iri!("http://www.w3.org/2001/XMLSchema#decimal");

/// <http://www.w3.org/2001/XMLSchema#double> datatype IRI.
pub const XSD_DOUBLE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#double");

/// <http://www.w3.org/2001/XMLSchema#anyURI> datatype IRI.
pub const XSD_ANY_URI: &Iri = iri!("http://www.w3.org/2001/XMLSchema#anyURI");

/// <http://www.w3.org/2001/XMLSchema#QName> datatype IRI.
pub const XSD_Q_NAME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#QName");

/// <http://www.w3.org/2001/XMLSchema#NOTATION> datatype IRI.
pub const XSD_NOTATION: &Iri = iri!("http://www.w3.org/2001/XMLSchema#NOTATION");

/// <http://www.w3.org/2001/XMLSchema#normalizedString> datatype IRI.
pub const XSD_NORMALIZED_STRING: &Iri = iri!("http://www.w3.org/2001/XMLSchema#normalizedString");

/// <http://www.w3.org/2001/XMLSchema#token> datatype IRI.
pub const XSD_TOKEN: &Iri = iri!("http://www.w3.org/2001/XMLSchema#token");

/// <http://www.w3.org/2001/XMLSchema#language> datatype IRI.
pub const XSD_LANGUAGE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#language");

/// <http://www.w3.org/2001/XMLSchema#Name> datatype IRI.
pub const XSD_NAME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#Name");

/// <http://www.w3.org/2001/XMLSchema#NMTOKEN> datatype IRI.
pub const XSD_NMTOKEN: &Iri = iri!("http://www.w3.org/2001/XMLSchema#NMTOKEN");

/// <http://www.w3.org/2001/XMLSchema#NCName> datatype IRI.
pub const XSD_NC_NAME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#NCName");

/// <http://www.w3.org/2001/XMLSchema#NMTOKENS> datatype IRI.
pub const XSD_NMTOKENS: &Iri = iri!("http://www.w3.org/2001/XMLSchema#NMTOKENS");

/// <http://www.w3.org/2001/XMLSchema#ID> datatype IRI.
pub const XSD_ID: &Iri = iri!("http://www.w3.org/2001/XMLSchema#ID");

/// <http://www.w3.org/2001/XMLSchema#IDREF> datatype IRI.
pub const XSD_IDREF: &Iri = iri!("http://www.w3.org/2001/XMLSchema#IDREF");

/// <http://www.w3.org/2001/XMLSchema#ENTITY> datatype IRI.
pub const XSD_ENTITY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#ENTITY");

/// <http://www.w3.org/2001/XMLSchema#IDREFS> datatype IRI.
pub const XSD_IDREFS: &Iri = iri!("http://www.w3.org/2001/XMLSchema#IDREFS");

/// <http://www.w3.org/2001/XMLSchema#ENTITIES> datatype IRI.
pub const XSD_ENTITIES: &Iri = iri!("http://www.w3.org/2001/XMLSchema#ENTITIES");

/// <http://www.w3.org/2001/XMLSchema#integer> datatype IRI.
pub const XSD_INTEGER: &Iri = iri!("http://www.w3.org/2001/XMLSchema#integer");

/// <http://www.w3.org/2001/XMLSchema#nonPositiveInteger> datatype IRI.
pub const XSD_NON_POSITIVE_INTEGER: &Iri =
	iri!("http://www.w3.org/2001/XMLSchema#nonPositiveInteger");

/// <http://www.w3.org/2001/XMLSchema#negativeInteger> datatype IRI.
pub const XSD_NEGATIVE_INTEGER: &Iri = iri!("http://www.w3.org/2001/XMLSchema#negativeInteger");

/// <http://www.w3.org/2001/XMLSchema#long> datatype IRI.
pub const XSD_LONG: &Iri = iri!("http://www.w3.org/2001/XMLSchema#long");

/// <http://www.w3.org/2001/XMLSchema#int> datatype IRI.
pub const XSD_INT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#int");

/// <http://www.w3.org/2001/XMLSchema#short> datatype IRI.
pub const XSD_SHORT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#short");

/// <http://www.w3.org/2001/XMLSchema#byte> datatype IRI.
pub const XSD_BYTE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#byte");

/// <http://www.w3.org/2001/XMLSchema#nonNegativeInteger> datatype IRI.
pub const XSD_NON_NEGATIVE_INTEGER: &Iri =
	iri!("http://www.w3.org/2001/XMLSchema#nonNegativeInteger");

/// <http://www.w3.org/2001/XMLSchema#unsignedLong> datatype IRI.
pub const XSD_UNSIGNED_LONG: &Iri = iri!("http://www.w3.org/2001/XMLSchema#unsignedLong");

/// <http://www.w3.org/2001/XMLSchema#unsignedInt> datatype IRI.
pub const XSD_UNSIGNED_INT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#unsignedInt");

/// <http://www.w3.org/2001/XMLSchema#unsignedShort> datatype IRI.
pub const XSD_UNSIGNED_SHORT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#unsignedShort");

/// <http://www.w3.org/2001/XMLSchema#unsignedByte> datatype IRI.
pub const XSD_UNSIGNED_BYTE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#unsignedByte");

/// <http://www.w3.org/2001/XMLSchema#positiveInteger> datatype IRI.
pub const XSD_POSITIVE_INTEGER: &Iri = iri!("http://www.w3.org/2001/XMLSchema#positiveInteger");

/// Parse a value directly from its XSD lexical form.
pub trait ParseXsd: Sized {
	type LexicalForm: LexicalFormOf<Self> + ?Sized;

	fn parse_xsd(lexical_value: &str) -> ParseXsdResult<Self, Self::LexicalForm> {
		Self::LexicalForm::parse(lexical_value)
			.map_err(ParseXsdError::InvalidLexicalForm)?
			.try_as_value()
			.map_err(ParseXsdError::InvalidValue)
	}
}

/// XSD lexical parse result.
pub type ParseXsdResult<T, L> =
	Result<T, ParseXsdError<<L as Lexical>::Error, <L as LexicalFormOf<T>>::ValueError>>;

/// XSD lexical parse error.
pub enum ParseXsdError<L, V> {
	InvalidLexicalForm(L),
	InvalidValue(V),
}
