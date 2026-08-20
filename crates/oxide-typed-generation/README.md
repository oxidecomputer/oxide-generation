<!-- cargo-sync-rdme title [[ -->
# oxide-typed-generation
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme badge [[ -->
![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/oxide-typed-generation.svg?)
[![crates.io](https://img.shields.io/crates/v/oxide-typed-generation.svg?logo=rust)](https://crates.io/crates/oxide-typed-generation)
[![docs.rs](https://img.shields.io/docsrs/oxide-typed-generation.svg?logo=docs.rs)](https://docs.rs/oxide-typed-generation)
[![Rust: ^1.85.0](https://img.shields.io/badge/rust-^1.85.0-93450a.svg?logo=rust)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme rustdoc [[ -->
A newtype wrapper around [`Generation`].

## Motivation

Many large systems track a number of independent generation numbers: versions attached to
individual resources, configuration generations, and so on. However, a bare integer does not
carry information about the kind of counter it belongs to, which can lead to comparing or
assigning the wrong generation number at runtime.

This crate provides a [`Generation`] type for untyped generation numbers, along with a wrapper
type around it that allows you to specify the kind of counter a generation number belongs to.

## Example

````rust
use oxide_typed_generation::{
    GenericGeneration, TypedGeneration, TypedGenerationKind, TypedGenerationTag,
};

// First, define a type that represents the kind of generation number this is.
enum MyKind {}

impl TypedGenerationKind for MyKind {
    fn tag() -> TypedGenerationTag {
        // Tags are required to be ASCII identifiers, with underscores
        // and dashes also supported. The validity of a tag can be checked
        // at compile time by assigning it to a const, like so:
        const TAG: TypedGenerationTag = TypedGenerationTag::new("my_kind");
        TAG
    }
}

// Now, a generation number can be created with this kind.
let generation: TypedGeneration<MyKind> = "5".parse().unwrap();

// The Display (and therefore ToString) impls still show the same value.
assert_eq!(generation.to_string(), "5");

// The Debug impl will show the tag as well.
assert_eq!(format!("{:?}", generation), "5 (my_kind)");
````

If you have a large number of generation kinds, consider using
[`oxide-typed-generation-macros`] which comes with several convenience features.

````rust
use oxide_typed_generation_macros::impl_typed_generation_kinds;

// Invoke this macro with:
impl_typed_generation_kinds! {
    kinds = {
        User = {},
        Project = {},
        // ...
    },
}
````

See [`oxide-typed-generation-macros`] for more information.

For simpler cases, you can also write your own declarative macro. Use this
template to get started:

````rust
macro_rules! impl_kinds {
    ($($kind:ident => $tag:literal),* $(,)?) => {
        $(
            pub enum $kind {}

            impl TypedGenerationKind for $kind {
                #[inline]
                fn tag() -> TypedGenerationTag {
                    const TAG: TypedGenerationTag = TypedGenerationTag::new($tag);
                    TAG
                }
            }
        )*
    };
}

// Invoke this macro with:
impl_kinds! {
    UserKind => "user",
    ProjectKind => "project",
}
````

## Implementations

In general, [`TypedGeneration`] uses the same wire and serialization formats as [`Generation`].
This means that persistent representations of [`TypedGeneration`] are the same as
[`Generation`]; [`TypedGeneration`] is intended to be helpful within Rust code, not across
serialization boundaries.

* The `Display` and `FromStr` impls are forwarded to the underlying [`Generation`].
* If the `serde` feature is enabled, `TypedGeneration` will serialize and deserialize using the
  same format as [`Generation`].
* If the `schemars08` feature is enabled, [`TypedGeneration`] will implement `JsonSchema` if the
  corresponding [`TypedGenerationKind`] implements `JsonSchema`.

To abstract over typed and untyped generation numbers, the [`GenericGeneration`] trait is
provided. This trait also permits conversions between typed and untyped generation numbers.

## Dependencies

* This crate has no required dependencies. Optional features may add further dependencies.

## Features

* `default`: Enables default features in the oxide-typed-generation crate.
* `std`: Enables the use of the standard library. *Enabled by default.*
* `serde`: Enables serialization and deserialization support via Serde. *Not enabled by
  default.*
* `schemars08`: Enables support for generating JSON schemas via schemars 0.8. *Not enabled by
  default.* Note that the format of the generated schema is **not currently part** of the stable
  API, though we hope to stabilize it in the future.
* `proptest1`: Enables support for generating `proptest::Arbitrary` instances of generation
  numbers. *Not enabled by default.*

## Minimum supported Rust version (MSRV)

The MSRV of this crate is **Rust 1.85.** In general, this crate will follow the MSRV of its
dependencies, with an aim to be conservative.

Within the 0.x series, MSRV updates will be accompanied by a minor version bump.

[`Generation`]: https://docs.rs/oxide-typed-generation/0.1.0/oxide_typed_generation/struct.Generation.html "struct oxide_typed_generation::Generation"
[`oxide-typed-generation-macros`]: https://docs.rs/oxide-typed-generation-macros
[`TypedGeneration`]: https://docs.rs/oxide-typed-generation/0.1.0/oxide_typed_generation/struct.TypedGeneration.html "struct oxide_typed_generation::TypedGeneration"
[`TypedGenerationKind`]: https://docs.rs/oxide-typed-generation/0.1.0/oxide_typed_generation/trait.TypedGenerationKind.html "trait oxide_typed_generation::TypedGenerationKind"
[`GenericGeneration`]: https://docs.rs/oxide-typed-generation/0.1.0/oxide_typed_generation/trait.GenericGeneration.html "trait oxide_typed_generation::GenericGeneration"
<!-- cargo-sync-rdme ]] -->

## License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the [MIT
license](LICENSE-MIT).
