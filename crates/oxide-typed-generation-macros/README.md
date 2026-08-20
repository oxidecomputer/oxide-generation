<!-- cargo-sync-rdme title [[ -->
# oxide-typed-generation-macros
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme badge [[ -->
![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/oxide-typed-generation-macros.svg?)
[![crates.io](https://img.shields.io/crates/v/oxide-typed-generation-macros.svg?logo=rust)](https://crates.io/crates/oxide-typed-generation-macros)
[![docs.rs](https://img.shields.io/docsrs/oxide-typed-generation-macros.svg?logo=docs.rs)](https://docs.rs/oxide-typed-generation-macros)
[![Rust: ^1.85.0](https://img.shields.io/badge/rust-^1.85.0-93450a.svg?logo=rust)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme rustdoc [[ -->
Procedural macro for [`oxide-typed-generation`](https://docs.rs/oxide-typed-generation).

This crate provides a procedural macro to help with creating
[`oxide-typed-generation`](https://docs.rs/oxide-typed-generation) instances.

For more information, see the documentation for [`impl_typed_generation_kinds!`].

## Examples

Basic usage:

````rust
use oxide_typed_generation::TypedGenerationKind;
use oxide_typed_generation_macros::impl_typed_generation_kinds;

impl_typed_generation_kinds! {
    kinds = {
        User = {},
        BusinessUnit = {},
    },
}

// This generates empty UserKind and BusinessUnitKind enums implementing
// TypedGenerationKind, with the tags "user" and "business_unit" respectively.
// Tags are snake_case versions of type names.
assert_eq!(UserKind::tag().as_str(), "user");
assert_eq!(BusinessUnitKind::tag().as_str(), "business_unit");

// The macro also generates UserGeneration and BusinessUnitGeneration type aliases.
let user_generation = UserGeneration::new();
let business_unit_generation = BusinessUnitGeneration::new();
````

For more details and examples, see the documentation for
[`impl_typed_generation_kinds!`].

[`impl_typed_generation_kinds!`]: https://docs.rs/oxide-typed-generation-macros/0.1.0/oxide_typed_generation_macros/macro.impl_typed_generation_kinds.html "macro oxide_typed_generation_macros::impl_typed_generation_kinds"
<!-- cargo-sync-rdme ]] -->

## License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the [MIT
license](LICENSE-MIT).
