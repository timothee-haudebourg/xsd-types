//! This crate aims at providing safe representations
//! of [XSD built-in data types](https://www.w3.org/TR/xmlschema-2/#built-in-datatypes).
//! For now, only numeric types are implemented.
use iref::Iri;
use static_iref::iri;

pub mod lexical;
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

pub const XSD_DURATION: &Iri = iri!("http://www.w3.org/2001/XMLSchema#duration");
pub const XSD_DATE_TIME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#dateTime");
pub const XSD_TIME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#time");
pub const XSD_DATE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#date");
pub const XSD_G_YEAR_MONTH: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gYearMonth");
pub const XSD_G_YEAR: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gYear");
pub const XSD_G_MONTH_DAY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gMonthDay");
pub const XSD_G_DAY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gDay");
pub const XSD_G_MONTH: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gMonth");
pub const XSD_STRING: &Iri = iri!("http://www.w3.org/2001/XMLSchema#string");
pub const XSD_BOOLEAN: &Iri = iri!("http://www.w3.org/2001/XMLSchema#boolean");
pub const XSD_BASE64_BINARY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#base64Binary");
pub const XSD_HEX_BINARY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#hexBinary");
pub const XSD_FLOAT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#float");
pub const XSD_DECIMAL: &Iri = iri!("http://www.w3.org/2001/XMLSchema#decimal");
pub const XSD_DOUBLE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#double");
pub const XSD_ANY_URI: &Iri = iri!("http://www.w3.org/2001/XMLSchema#anyURI");
pub const XSD_Q_NAME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#QName");
pub const XSD_NOTATION: &Iri = iri!("http://www.w3.org/2001/XMLSchema#NOTATION");
pub const XSD_NORMALIZED_STRING: &Iri = iri!("http://www.w3.org/2001/XMLSchema#normalizedString");
pub const XSD_TOKEN: &Iri = iri!("http://www.w3.org/2001/XMLSchema#token");
pub const XSD_LANGUAGE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#language");
pub const XSD_NAME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#Name");
pub const XSD_NMTOKEN: &Iri = iri!("http://www.w3.org/2001/XMLSchema#NMTOKEN");
pub const XSD_NC_NAME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#NCName");
pub const XSD_NMTOKENS: &Iri = iri!("http://www.w3.org/2001/XMLSchema#NMTOKENS");
pub const XSD_ID: &Iri = iri!("http://www.w3.org/2001/XMLSchema#ID");
pub const XSD_IDREF: &Iri = iri!("http://www.w3.org/2001/XMLSchema#IDREF");
pub const XSD_ENTITY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#ENTITY");
pub const XSD_IDREFS: &Iri = iri!("http://www.w3.org/2001/XMLSchema#IDREFS");
pub const XSD_ENTITIES: &Iri = iri!("http://www.w3.org/2001/XMLSchema#ENTITIES");
pub const XSD_INTEGER: &Iri = iri!("http://www.w3.org/2001/XMLSchema#integer");
pub const XSD_NON_POSITIVE_INTEGER: &Iri =
	iri!("http://www.w3.org/2001/XMLSchema#nonPositiveInteger");
pub const XSD_NEGATIVE_INTEGER: &Iri = iri!("http://www.w3.org/2001/XMLSchema#negativeInteger");
pub const XSD_LONG: &Iri = iri!("http://www.w3.org/2001/XMLSchema#long");
pub const XSD_INT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#int");
pub const XSD_SHORT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#short");
pub const XSD_BYTE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#byte");
pub const XSD_NON_NEGATIVE_INTEGER: &Iri =
	iri!("http://www.w3.org/2001/XMLSchema#nonNegativeInteger");
pub const XSD_UNSIGNED_LONG: &Iri = iri!("http://www.w3.org/2001/XMLSchema#unsignedLong");
pub const XSD_UNSIGNED_INT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#unsignedInt");
pub const XSD_UNSIGNED_SHORT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#unsignedShort");
pub const XSD_UNSIGNED_BYTE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#unsignedByte");
pub const XSD_POSITIVE_INTEGER: &Iri = iri!("http://www.w3.org/2001/XMLSchema#positiveInteger");

/// Parse a value directly from its RDF lexical form.
pub trait ParseRdf: Sized {
	type LexicalForm: LexicalFormOf<Self> + ?Sized;

	fn parse_rdf(lexical_value: &str) -> ParseRdfResult<Self, Self::LexicalForm> {
		Self::LexicalForm::parse(lexical_value)
			.map_err(ParseRdfError::InvalidLexicalForm)?
			.try_as_value()
			.map_err(ParseRdfError::InvalidValue)
	}
}

pub type ParseRdfResult<T, L> =
	Result<T, ParseRdfError<<L as Lexical>::Error, <L as LexicalFormOf<T>>::ValueError>>;

pub enum ParseRdfError<L, V> {
	InvalidLexicalForm(L),
	InvalidValue(V),
}
