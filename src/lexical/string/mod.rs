use static_automata::grammar;

#[grammar(
	file = "grammar.abnf",
	export(
		"NCName" as Id,
		"NCName" as IdRef,
		"NCName" as NCName,
		"Name" as Name,
		"Nmtoken" as NMToken,
		"language" as Language
	)
)]
mod grammar {}

mod id;
mod idref;
mod language;
mod name;
mod ncname;
mod nmtoken;
mod normalized;
mod token;

pub use id::*;
pub use idref::*;
pub use language::*;
pub use name::*;
pub use ncname::*;
pub use nmtoken::*;

use super::XsdLexicalForm;
use crate::XsdError;

impl XsdLexicalForm for str {
	type Value = String;
}

impl<'a> crate::XsdFromStr<'a> for &'a str {
	fn xsd_from_str(value: &'a str) -> Result<Self, XsdError> {
		Ok(value)
	}
}
