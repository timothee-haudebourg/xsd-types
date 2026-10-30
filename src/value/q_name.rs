use crate::lexical::{XsdQName, XsdQNameBuf};

impl<'a> crate::XsdFromStr<'a> for XsdQNameBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		crate::xsd_parse_via::<XsdQName, Self>(lexical_value)
	}
}

impl crate::XsdTyped for XsdQNameBuf {
	fn datatype(&self) -> crate::Datatype {
		crate::Datatype::QName
	}
}

impl crate::XsdValueDomain for XsdQNameBuf {
	type LexicalForm = XsdQName;

	fn to_lexical_form(&self) -> XsdQNameBuf {
		// `XsdQName` has no distinct value space, so the lexical form is the
		// value itself.
		self.clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{XsdFromStr, XsdValueDomain};

	#[test]
	fn to_lexical_form_round_trips() {
		for input in ["a", "xs:string", "_p-1.x:Local_Name"] {
			let value = XsdQNameBuf::xsd_from_str(input).unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), input);
			assert_eq!(XsdQNameBuf::xsd_from_str(lexical.as_str()).unwrap(), value);
		}
	}
}
