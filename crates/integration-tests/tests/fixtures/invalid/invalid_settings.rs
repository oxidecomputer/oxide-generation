use oxide_typed_generation_macros::impl_typed_generation_kinds;

impl_typed_generation_kinds! {
    settings = {
        schemars08 = {
            attrs = [#[cfg(feature = "schemars08")]],
            rust_type = {
                crate = "my-service",
                version = "1.0.0",
                // Missing required 'path' field
            },
        },
    },
    kinds = {
        User = {},
    }
}

fn main() {}
