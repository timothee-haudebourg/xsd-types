use static_regular_grammar::RegularGrammar;

use super::{Lexical, LexicalFormOf};

/// Date and time.
///
/// ```abnf
/// date-time = year "-" month "-" day %s"T" time [timezone]
///
/// time = hour ":" minute ":" second ["." fraction]
///      / "24:00:00" ["." 1*"0"]
///
/// year = [ "-" ] year-number
///
/// year-number = *3DIGIT NZDIGIT
///             / *2DIGIT NZDIGIT DIGIT
///             / *1DIGIT NZDIGIT 2DIGIT
///             / NZDIGIT 3*DIGIT
///
/// month = "0" NZDIGIT
///       / "1" ( "0" / "1" / "2" )
///
/// day = "0" NZDIGIT
///     / ("1" / "2") DIGIT
///     / "3" ("0" / "1")
///
/// hour = "0" NZDIGIT
///      / "1" DIGIT
///      / "2" ("0" / "1" / "2" / "3")
///
/// minute = "0" NZDIGIT
///        / ("1" / "2" / "3" / "4" / "5") DIGIT
///
/// second = "0" NZDIGIT
///        / ("1" / "2" / "3" / "4" / "5") DIGIT
///
/// fraction = 1*DIGIT
///
/// timezone = ("+" / "-") hour ":" minute
///          / %s"Z"
///
/// NZDIGIT = "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9"
/// ```
///
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(sized(DateTimeBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
pub struct DateTime(str);

impl Lexical for DateTime {
	type Error = InvalidDateTime<String>;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Self::new(value).map_err(|_| InvalidDateTime(value.to_owned()))
	}
}

impl LexicalFormOf<crate::DateTime> for DateTime {
	type ValueError = chrono::ParseError;

	fn try_as_value(&self) -> Result<crate::DateTime, Self::ValueError> {
		self.as_str().parse()
	}
}
