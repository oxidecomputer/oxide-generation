use oxide_generation_macros::impl_typed_generation_kinds;

impl_typed_generation_kinds! {
    // this is not valid syntax at all
    User => "user",
    Organization => "org",
}

fn main() {}
