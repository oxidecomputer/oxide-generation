use oxide_generation_macros::impl_typed_generation_kinds;

impl_typed_generation_kinds! {
    settings = {
        oxide_generation_crate = "my_custom_generation",
    },
    kinds = {
        User = {},
    }
}

fn main() {}
