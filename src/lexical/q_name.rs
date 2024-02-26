use static_regular_grammar::RegularGrammar;

use super::{Lexical, LexicalFormOf};

/// Qualified Name.
///
/// ```abnf
/// QName = PrefixedName / UnprefixedName
///
/// PrefixedName = Prefix ":" LocalPart
///
/// UnprefixedName = LocalPart
///
/// Prefix = NCName
///
/// LocalPart = NCName
///
/// NCName = NCNameStartChar *NCNameChar
///
/// NCNameStartChar = ALPHA / "_" / %xC0-D6 / %xD8-F6 / %xF8-2FF / %x370-37D / %x37F-1FFF / %x200C-200D / %x2070-218F / %x2C00-2FEF / %x3001-D7FF / %xF900-FDCF / %xFDF0-FFFD / %x10000-EFFFF
///
/// NCNameChar = NCNameStartChar / "-" / "." / DIGIT / %xB7 / %x0300-036F / %x203F-2040
/// ```
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(sized(QNameBuf, derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct QName(str);

impl Lexical for QName {
	type Error = InvalidQName<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidQName(value.to_owned()))
	}
}

impl LexicalFormOf<QNameBuf> for QName {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<QNameBuf, Self::ValueError> {
		Ok(self.to_owned())
	}
}
