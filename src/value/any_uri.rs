use crate::{Datatype, ParseXsd, XsdValue};

pub type AnyUri = iref::Uri;

impl XsdValue for AnyUri {
	fn datatype(&self) -> Datatype {
		Datatype::AnyUri
	}
}

pub type AnyUriBuf = iref::UriBuf;

impl XsdValue for AnyUriBuf {
	fn datatype(&self) -> Datatype {
		Datatype::AnyUri
	}
}

impl ParseXsd for AnyUriBuf {
	type LexicalForm = AnyUri;
}
