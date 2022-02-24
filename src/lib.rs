//! This crate aims at providing safe representations
//! of [XSD built-in data types](https://www.w3.org/TR/xmlschema-2/#built-in-datatypes).
//! For now, only numeric types are implemented.
mod decimal;
mod double;
mod integer;

pub use decimal::*;
pub use double::*;
pub use integer::*;

/// Error thrown when a conversion function overflowed.
pub struct Overflow;
