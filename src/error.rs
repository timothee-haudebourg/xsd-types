use crate::{
	InvalidXsdDateTimeStampValue, InvalidXsdDateTimeValue, InvalidXsdDateValue, InvalidXsdId,
	InvalidXsdIdRef, InvalidXsdLanguage, InvalidXsdNCName, InvalidXsdNMToken, InvalidXsdName,
	InvalidXsdNormalizedStr, InvalidXsdTimeValue, InvalidXsdToken, lexical,
};

#[derive(Debug, thiserror::Error)]
pub enum XsdError {
	#[error("invalid lexical form")]
	LexicalForm(String),

	#[error("invalid value")]
	Value(String),
}

impl From<std::convert::Infallible> for XsdError {
	fn from(value: std::convert::Infallible) -> Self {
		match value {}
	}
}

impl From<lexical::XsdOverflow> for XsdError {
	fn from(_: lexical::XsdOverflow) -> Self {
		Self::value("integer out of target bounds")
	}
}

macro_rules! value_error_from {
	{ $( $ty:ty ),* $(,)? } => {
		$(
			impl From<$ty> for XsdError {
				fn from(value: $ty) -> Self {
					Self::value(value)
				}
			}
		)*
	};
}

value_error_from! {
	InvalidXsdDateValue,
	InvalidXsdDateTimeValue,
	InvalidXsdDateTimeStampValue,
	InvalidXsdTimeValue,
	InvalidXsdToken<String>,
	InvalidXsdNormalizedStr<String>,
	InvalidXsdId<String>,
	InvalidXsdIdRef<String>,
	InvalidXsdLanguage<String>,
	InvalidXsdName<String>,
	InvalidXsdNCName<String>,
	InvalidXsdNMToken<String>,
}

impl XsdError {
	pub fn lexical_form(e: impl ToString) -> Self {
		Self::LexicalForm(e.to_string())
	}

	pub fn value(e: impl ToString) -> Self {
		Self::Value(e.to_string())
	}
}
