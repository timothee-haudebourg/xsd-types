//! This crate aims at providing safe representations
//! of [XSD built-in data types](https://www.w3.org/TR/xmlschema-2/#built-in-datatypes).
//! For now, only numeric types are implemented.
mod decimal;
mod integer;
mod float;

pub use decimal::*;
pub use integer::*;
pub use float::*;

/// Error thrown when a conversion function overflowed.
pub struct Overflow;
