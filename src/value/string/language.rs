use static_regular_grammar::RegularGrammar;

use crate::ParseRdf;

/// Language.
///
/// ```abnf
/// language = 1*8ALPHA *("-" 1*8(ALPHA / DIGIT))
/// ```
///
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(sized(
	LanguageBuf,
	derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)
))]
pub struct Language(str);

impl ParseRdf for LanguageBuf {
	type LexicalForm = crate::lexical::Language;
}
