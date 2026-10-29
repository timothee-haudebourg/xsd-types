/// Qualified name value.
///
/// `QName` has no distinct value-space encoding, so the lexical
/// [`QName`](crate::lexical::QName)/[`QNameBuf`](crate::lexical::QNameBuf)
/// representation is reused directly as the value type.
/// See: <https://www.w3.org/TR/xmlschema11-2/#QName>.
pub use crate::lexical::{QName, QNameBuf};

impl<'a> crate::XsdFromStr<'a> for QNameBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<QName, Self>(lexical_value)
	}
}

impl crate::XsdTyped for QNameBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::Datatype::QName
	}
}

impl crate::XsdValue for QNameBuf {
	type LexicalForm = QName;

	fn to_lexical_form(&self) -> QNameBuf {
		// `QName` has no distinct value space, so the lexical form is the
		// value itself.
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{XsdFromStr, XsdValue};

	#[test]
	fn to_lexical_form_round_trips() {
		for input in ["a", "xs:string", "_p-1.x:Local_Name"] {
			let value = QNameBuf::xsd_from_str(input).unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), input);
			assert_eq!(QNameBuf::xsd_from_str(lexical.as_str()).unwrap(), value);
		}
	}
}
