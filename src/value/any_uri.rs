use crate::{Datatype, XsdFromStr, XsdTyped, XsdValue};

/// IRI, used as the value-space representation of the XSD 1.1 `anyURI`
/// datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#anyURI>.
pub type AnyUri = iref::Iri;

impl XsdTyped for AnyUri {
	fn datatype(&self) -> Datatype {
		Datatype::AnyUri
	}
}

/// Owned IRI buffer, used as the value-space representation of the XSD 1.1
/// `anyURI` datatype.
/// See: <https://www.w3.org/TR/xmlschema11-2/#anyURI>.
pub type AnyUriBuf = iref::IriBuf;

impl XsdTyped for AnyUriBuf {
	fn datatype(&self) -> Datatype {
		Datatype::AnyUri
	}
}

impl XsdValue for AnyUriBuf {
	type LexicalForm = AnyUri;

	fn to_lexical_form(&self) -> AnyUriBuf {
		// The value space of `anyURI` is the lexical space, so the lexical
		// form is the IRI itself.
		self.clone()
	}
}

impl<'a> XsdFromStr<'a> for AnyUriBuf {
	fn xsd_from_str(lexical_value: &'a str) -> Result<Self, crate::XsdError> {
		<&'a AnyUri as XsdFromStr<'a>>::xsd_from_str(lexical_value).map(AnyUri::to_owned)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// The XSD `anyURI` datatype is defined in terms of IRIs (RFC 3987), not
	// plain URIs (RFC 3986), so non-ASCII characters must be accepted.
	#[test]
	fn accepts_unicode_iri() {
		AnyUriBuf::xsd_from_str("http://xn--exmple-cva.example/例").unwrap();
		AnyUriBuf::xsd_from_str("http://例え.jp/").unwrap();
	}

	#[test]
	fn to_lexical_form_round_trips() {
		for input in [
			"http://example.org/a/b?c=d#e",
			"urn:isbn:0451450523",
			"http://例え.jp/",
		] {
			let value = AnyUriBuf::xsd_from_str(input).unwrap();
			let lexical = value.to_lexical_form();
			assert_eq!(lexical.as_str(), input);
			assert_eq!(AnyUriBuf::xsd_from_str(lexical.as_str()).unwrap(), value);
		}
	}
}
