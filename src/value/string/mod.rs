use crate::{Datatype, ParseXsd, XsdValue};

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
pub use normalized::*;
pub use token::*;

pub type String = std::string::String;

impl XsdValue for String {
	fn datatype(&self) -> Datatype {
		Datatype::String(crate::StringDatatype::String)
	}
}

impl ParseXsd for String {
	type LexicalForm = str;
}
