use oxide_generation_macros::impl_typed_generation_kinds;

enum UserKind {}
struct UserGeneration;

impl_typed_generation_kinds! {
    kinds = {
        User = {},
    }
}

fn main() {}
