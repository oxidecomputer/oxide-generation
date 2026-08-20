# Custom crate tests

This crate contains integration tests for the `oxide_typed_generation_crate`  parameter of the `impl_typed_generation_kinds!` macro.

When `oxide_typed_generation_crate = my_custom_generation` is specified, the generated code should use `::my_custom_generation::TypedGenerationKind`, `::my_custom_generation::TypedGenerationTag`, etc. instead of the default `::oxide_typed_generation::` prefixes.
