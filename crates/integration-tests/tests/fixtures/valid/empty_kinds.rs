use oxide_generation_macros::impl_typed_generation_kinds;

impl_typed_generation_kinds! {
    kinds = {}
}

fn main() {
    // Test that the macro compiles successfully with no kinds defined
    // This should generate no code but not error
}
