use crate::{Datatype, ParseXsd, XsdValue};

pub type AnyUri = iref::Iri;

impl XsdValue for AnyUri {
	fn datatype(&self) -> Datatype {
		Datatype::AnyUri
	}
}

pub type AnyUriBuf = iref::IriBuf;

impl XsdValue for AnyUriBuf {
	fn datatype(&self) -> Datatype {
		Datatype::AnyUri
	}
}

impl ParseXsd for AnyUriBuf {
	type LexicalForm = AnyUri;
}

#[cfg(test)]
mod tests {
	use super::*;

	// The XSD `anyURI` datatype is defined in terms of IRIs (RFC 3987), not
	// plain URIs (RFC 3986), so non-ASCII characters must be accepted.
	#[test]
	fn accepts_unicode_iri() {
		AnyUriBuf::parse_xsd("http://xn--exmple-cva.example/例").unwrap();
		AnyUriBuf::parse_xsd("http://例え.jp/").unwrap();
	}
}
