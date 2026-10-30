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
pub type XsdString = std::string::String;

/// Unsized counterpart of [`XsdString`], aliased to [`str`].
///
/// The `string` datatype is its own lexical domain, so this is also the
/// lexical form of [`XsdString`].
pub type XsdStr = str;

impl XsdTyped for XsdString {
	fn datatype(&self) -> Datatype {
		Datatype::String(crate::StringDatatype::String)
	}
}

impl XsdValue for XsdString {
	type LexicalForm = XsdStr;

	fn to_lexical_form(&self) -> XsdString {
		self.clone()
	}
}

impl<'a> XsdFromStr<'a> for XsdString {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<XsdStr, Self>(lexical_value)
	}
}

#[cfg(test)]
mod tests {
	use crate::{XsdFromStr, XsdValue};

	#[test]
	fn lexical_form_round_trip() {
		let input = "hello \t world\n";
		let value = <super::XsdString as XsdFromStr>::xsd_from_str(input).unwrap();
		let lexical = value.to_lexical_form();
		assert_eq!(lexical, input);
		assert_eq!(
			<super::XsdString as XsdFromStr>::xsd_from_str(&lexical).unwrap(),
			value
		);
	}
}
