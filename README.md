# XSD Types for Rust

[![CI](https://github.com/timothee-haudebourg/xsd-types/workflows/CI/badge.svg)](https://github.com/timothee-haudebourg/xsd-types/actions)
[![Crate informations](https://img.shields.io/crates/v/xsd-types.svg?style=flat-square)](https://crates.io/crates/xsd-types)
[![License](https://img.shields.io/crates/l/xsd-types.svg?style=flat-square)](https://github.com/timothee-haudebourg/xsd-types#license)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/xsd-types)

<!-- cargo-rdme start -->

This crate aims at providing safe representations
of [XSD built-in data types][xsd].

[xsd]: <https://www.w3.org/TR/xmlschema-2/#built-in-datatypes>

## Usage

For each XSD datatype, this library provides two families of types:
one representing the *lexical space* of the datatype (see the [`lexical`]
module), and one representing the *value space* of the datatype (see the
[`value`] module).

For instance, assume we wish to store the lexical representation of an
[XSD decimal][xsd-decimal] datatype value. We can use the
`lexical::Decimal` type (or its owned variant, `lexical::DecimalBuf`)

[`lexical`]: https://docs.rs/xsd-types/latest/xsd_types/lexical/
[`value`]: https://docs.rs/xsd-types/latest/xsd_types/value/
[xsd-decimal]: <https://www.w3.org/TR/xmlschema11-2/#decimal>

```rust
let string = "3.141592653589793";

// Parse the lexical representation (lexical domain).
let lexical_repr = xsd_types::lexical::Decimal::new(string).unwrap();

// Interprets the lexical representation (value domain).
use xsd_types::lexical::LexicalFormOf;
let value_repr: xsd_types::Decimal = lexical_repr.try_as_value().unwrap();
```

Of course it is possible to parse the value directly into the value domain
using `FromStr`:
```rust
let value_repr: xsd_types::Decimal = "3.141592653589793".parse().unwrap();
```

### Any value

The [`Value`] type provides a simple way to represent *any* XSD value.

```rust
use xsd_types::{XSD_DATE, Datatype, Value};
let dt = Datatype::from_iri(XSD_DATE).unwrap();
let value: Value = dt.parse("1758-12-25").unwrap(); // Halley is back!
```

[`Value`]: crate::Value

<!-- cargo-rdme end -->

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
