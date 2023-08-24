//! This crate aims at providing safe representations
//! of [XSD built-in data types](https://www.w3.org/TR/xmlschema-2/#built-in-datatypes).
//! For now, only numeric types are implemented.
use iref::Iri;
use static_iref::iri;

pub mod lexical;
pub mod value;

use lexical::{Lexical, LexicalFormOf};
pub use value::*;

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

macro_rules! impl_from {
	{
		$ty:ty {
			$($input:ident : $from_ty:ty => Self::$variant:ident($output:expr)),*
		}
	} => {
		$(
			impl From<$from_ty> for $ty {
				fn from($input: $from_ty) -> Self {
					Self::$variant(Some($output))
				}
			}

			impl From<Option<$from_ty>> for $ty {
				fn from(input_opt: Option<$from_ty>) -> Self {
					Self::$variant(input_opt.map(|$input| $output))
				}
			}
		)*
	};
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

/// XSD datatype.
pub enum Datatype {
	String(Option<StringDatatype>),
	Boolean,
	Decimal(Option<DecimalDatatype>),
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

impl Datatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::String(None) => XSD_STRING,
			Self::String(Some(t)) => t.iri(),
			Self::Boolean => XSD_BOOLEAN,
			Self::Decimal(None) => XSD_DECIMAL,
			Self::Decimal(Some(t)) => t.iri(),
			Self::Float => XSD_FLOAT,
			Self::Double => XSD_FLOAT,
			Self::Duration => XSD_DURATION,
			Self::DateTime => XSD_DATE_TIME,
			Self::Time => XSD_TIME,
			Self::Date => XSD_DATE,
			Self::GYearMonth => XSD_G_YEAR_MONTH,
			Self::GYear => XSD_G_YEAR,
			Self::GMonthDay => XSD_G_MONTH_DAY,
			Self::GDay => XSD_G_DAY,
			Self::GMonth => XSD_G_MONTH,
			Self::HexBinary => XSD_HEX_BINARY,
			Self::Base64Binary => XSD_BASE64_BINARY,
			Self::AnyUri => XSD_ANY_URI,
			Self::QName => XSD_Q_NAME,
			Self::Notation => XSD_NOTATION,
		}
	}
}

impl AsRef<Iri> for Datatype {
	fn as_ref(&self) -> &Iri {
		self.iri()
	}
}

impl_from!(Datatype {
	ty: StringDatatype => Self::String(ty),
	ty: DecimalDatatype => Self::Decimal(ty),
	ty: IntegerDatatype => Self::Decimal(ty.into()),
	ty: NonPositiveIntegerDatatype => Self::Decimal(ty.into()),
	ty: LongDatatype => Self::Decimal(ty.into()),
	ty: IntDatatype => Self::Decimal(ty.into()),
	ty: ShortDatatype => Self::Decimal(ty.into()),
	ty: NonNegativeIntegerDatatype => Self::Decimal(ty.into()),
	ty: UnsignedLongDatatype => Self::Decimal(ty.into()),
	ty: UnsignedIntDatatype => Self::Decimal(ty.into()),
	ty: UnsignedShortDatatype => Self::Decimal(ty.into())
});

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

/// Datatype derived from `xsd:string`.
pub enum StringDatatype {
	NormalizedString(Option<NormalizedStringDatatype>),
}

impl StringDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::NormalizedString(None) => XSD_NORMALIZED_STRING,
			Self::NormalizedString(Some(t)) => t.iri(),
		}
	}
}

pub enum NormalizedStringDatatype {
	Token(Option<TokenDatatype>),
}

impl NormalizedStringDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Token(None) => XSD_TOKEN,
			Self::Token(Some(t)) => t.iri(),
		}
	}
}

pub enum TokenDatatype {
	Language,
	NMToken,
	Name(Option<NameDatatype>),
}

impl TokenDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Language => XSD_LANGUAGE,
			Self::NMToken => XSD_NMTOKEN,
			Self::Name(None) => XSD_NAME,
			Self::Name(Some(t)) => t.iri(),
		}
	}
}

pub enum NameDatatype {
	NCName(Option<NCNameDatatype>),
}

impl NameDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::NCName(None) => XSD_NC_NAME,
			Self::NCName(Some(t)) => t.iri(),
		}
	}
}

pub enum NCNameDatatype {
	Id,
	IdRef,
	Entity,
}

impl NCNameDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Id => XSD_ID,
			Self::IdRef => XSD_IDREF,
			Self::Entity => XSD_ENTITY,
		}
	}
}

/// Datatype derived from `xsd:decimal`.
pub enum DecimalDatatype {
	Integer(Option<IntegerDatatype>),
}

impl DecimalDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Integer(None) => XSD_INTEGER,
			Self::Integer(Some(t)) => t.iri(),
		}
	}
}

impl_from!(DecimalDatatype {
	ty: IntegerDatatype => Self::Integer(ty),
	ty: NonPositiveIntegerDatatype => Self::Integer(ty.into()),
	ty: LongDatatype => Self::Integer(ty.into()),
	ty: IntDatatype => Self::Integer(ty.into()),
	ty: ShortDatatype => Self::Integer(ty.into()),
	ty: NonNegativeIntegerDatatype => Self::Integer(ty.into()),
	ty: UnsignedLongDatatype => Self::Integer(ty.into()),
	ty: UnsignedIntDatatype => Self::Integer(ty.into()),
	ty: UnsignedShortDatatype => Self::Integer(ty.into())
});

pub enum IntegerDatatype {
	NonPositiveInteger(Option<NonPositiveIntegerDatatype>),
	Long(Option<LongDatatype>),
	NonNegativeInteger(Option<NonNegativeIntegerDatatype>),
}

impl IntegerDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::NonPositiveInteger(None) => XSD_NON_POSITIVE_INTEGER,
			Self::NonPositiveInteger(Some(t)) => t.iri(),
			Self::Long(None) => XSD_LONG,
			Self::Long(Some(t)) => t.iri(),
			Self::NonNegativeInteger(None) => XSD_NON_NEGATIVE_INTEGER,
			Self::NonNegativeInteger(Some(t)) => t.iri(),
		}
	}
}

impl_from!(IntegerDatatype {
	ty: NonPositiveIntegerDatatype => Self::NonPositiveInteger(ty),
	ty: LongDatatype => Self::Long(ty),
	ty: IntDatatype => Self::Long(ty.into()),
	ty: ShortDatatype => Self::Long(ty.into()),
	ty: NonNegativeIntegerDatatype => Self::NonNegativeInteger(ty),
	ty: UnsignedLongDatatype => Self::NonNegativeInteger(ty.into()),
	ty: UnsignedIntDatatype => Self::NonNegativeInteger(ty.into()),
	ty: UnsignedShortDatatype => Self::NonNegativeInteger(ty.into())
});

pub enum NonPositiveIntegerDatatype {
	NegativeInteger,
}

impl NonPositiveIntegerDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::NegativeInteger => XSD_NEGATIVE_INTEGER,
		}
	}
}

pub enum LongDatatype {
	Int(Option<IntDatatype>),
}

impl LongDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Int(None) => XSD_INT,
			Self::Int(Some(t)) => t.iri(),
		}
	}
}

impl_from!(LongDatatype {
	ty: IntDatatype => Self::Int(ty),
	ty: ShortDatatype => Self::Int(ty.into())
});

pub enum IntDatatype {
	Short(Option<ShortDatatype>),
}

impl IntDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Short(None) => XSD_SHORT,
			Self::Short(Some(t)) => t.iri(),
		}
	}
}

impl_from!(IntDatatype {
	ty: ShortDatatype => Self::Short(ty)
});

pub enum ShortDatatype {
	Byte,
}

impl ShortDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Byte => XSD_BYTE,
		}
	}
}

pub enum NonNegativeIntegerDatatype {
	UnsignedLong(Option<UnsignedLongDatatype>),
	PositiveInteger,
}

impl NonNegativeIntegerDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::UnsignedLong(None) => XSD_UNSIGNED_LONG,
			Self::UnsignedLong(Some(t)) => t.iri(),
			Self::PositiveInteger => XSD_POSITIVE_INTEGER,
		}
	}
}

impl_from!(NonNegativeIntegerDatatype {
	ty: UnsignedLongDatatype => Self::UnsignedLong(ty),
	ty: UnsignedIntDatatype => Self::UnsignedLong(ty.into()),
	ty: UnsignedShortDatatype => Self::UnsignedLong(ty.into())
});

pub enum UnsignedLongDatatype {
	UnsignedInt(Option<UnsignedIntDatatype>),
}

impl UnsignedLongDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::UnsignedInt(None) => XSD_UNSIGNED_INT,
			Self::UnsignedInt(Some(t)) => t.iri(),
		}
	}
}

impl_from!(UnsignedLongDatatype {
	ty: UnsignedIntDatatype => Self::UnsignedInt(ty),
	ty: UnsignedShortDatatype => Self::UnsignedInt(ty.into())
});

pub enum UnsignedIntDatatype {
	UnsignedShort(Option<UnsignedShortDatatype>),
}

impl UnsignedIntDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::UnsignedShort(None) => XSD_UNSIGNED_SHORT,
			Self::UnsignedShort(Some(t)) => t.iri(),
		}
	}
}

impl_from!(UnsignedIntDatatype {
	ty: UnsignedShortDatatype => Self::UnsignedShort(ty)
});

pub enum UnsignedShortDatatype {
	UnsignedByte,
}

impl UnsignedShortDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::UnsignedByte => XSD_UNSIGNED_BYTE,
		}
	}
}
