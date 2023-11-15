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

use super::LexicalFormOf;

impl LexicalFormOf<String> for str {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<String, Self::ValueError> {
		Ok(self.to_string())
	}
}
