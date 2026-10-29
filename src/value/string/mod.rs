use crate::{Datatype, XsdFromStr, XsdTyped, XsdValue};

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

/// String value.
///
/// Value space representation of the XSD `string` datatype, aliased to
/// [`std::string::String`].
/// See: <https://www.w3.org/TR/xmlschema11-2/#string>.
pub type String = std::string::String;

impl XsdTyped for String {
	fn datatype(&self) -> Datatype {
		Datatype::String(crate::StringDatatype::String)
	}
}

impl XsdValue for String {
	type LexicalForm = str;

	fn to_lexical_form(&self) -> String {
		self.clone()
	}
}

impl<'a> XsdFromStr<'a> for String {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<str, Self>(lexical_value)
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	#[test]
	fn lexical_form_round_trip() {
		let input = "hello \t world\n";
		let value = <super::String as XsdFromStr>::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical, input);
		assert_eq!(
			<super::String as XsdFromStr>::xsd_from_str(&lexical).unwrap(),
			value
		);
	}
}
