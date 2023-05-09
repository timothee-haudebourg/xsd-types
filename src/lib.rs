//! This crate aims at providing safe representations
//! of [XSD built-in data types](https://www.w3.org/TR/xmlschema-2/#built-in-datatypes).
//! For now, only numeric types are implemented.

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

/// XSD datatype.
pub enum Datatype {
	String(Option<StringDatatype>),
	Boolean,
	Decimal(Option<DecimalDatatype>),
	Float,
	Double,
	Duration(Option<DurationDatatype>),
	DateTime(Option<DateTimeDatatype>),
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
	ty: UnsignedShortDatatype => Self::Decimal(ty.into()),
	ty: DurationDatatype => Self::Duration(ty),
	ty: DateTimeDatatype => Self::DateTime(ty)
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

pub enum NormalizedStringDatatype {
	Token(Option<TokenDatatype>),
}

pub enum TokenDatatype {
	Language,
	NMToken,
	Name(Option<NameDatatype>),
}

pub enum NameDatatype {
	NCName(Option<NCNameDatatype>),
}

pub enum NCNameDatatype {
	Id,
	IdRef,
	Entity,
}

/// Datatype derived from `xsd:decimal`.
pub enum DecimalDatatype {
	Integer(Option<IntegerDatatype>),
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

pub enum LongDatatype {
	Int(Option<IntDatatype>),
}

impl_from!(LongDatatype {
	ty: IntDatatype => Self::Int(ty),
	ty: ShortDatatype => Self::Int(ty.into())
});

pub enum IntDatatype {
	Short(Option<ShortDatatype>),
}

impl_from!(IntDatatype {
	ty: ShortDatatype => Self::Short(ty)
});

pub enum ShortDatatype {
	Byte,
}

pub enum NonNegativeIntegerDatatype {
	UnsignedLong(Option<UnsignedLongDatatype>),
	PositiveInteger,
}

impl_from!(NonNegativeIntegerDatatype {
	ty: UnsignedLongDatatype => Self::UnsignedLong(ty),
	ty: UnsignedIntDatatype => Self::UnsignedLong(ty.into()),
	ty: UnsignedShortDatatype => Self::UnsignedLong(ty.into())
});

pub enum UnsignedLongDatatype {
	UnsignedInt(Option<UnsignedIntDatatype>),
}

impl_from!(UnsignedLongDatatype {
	ty: UnsignedIntDatatype => Self::UnsignedInt(ty),
	ty: UnsignedShortDatatype => Self::UnsignedInt(ty.into())
});

pub enum UnsignedIntDatatype {
	UnsignedShort(Option<UnsignedShortDatatype>),
}

impl_from!(UnsignedIntDatatype {
	ty: UnsignedShortDatatype => Self::UnsignedShort(ty)
});

pub enum UnsignedShortDatatype {
	UnsignedByte,
}

/// Datatype derived from `xsd:duration`.
pub enum DurationDatatype {
	YearMonth,
	DayTime,
}

/// Datatype derived from `xsd:dateTime`.
pub enum DateTimeDatatype {
	DateTimeStamp,
}
