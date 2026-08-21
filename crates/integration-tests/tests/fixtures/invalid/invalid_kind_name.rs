use oxide_generation_macros::impl_typed_generation_kinds;

impl_typed_generation_kinds! {
    kinds = {
        123Invalid = {},
        NonÅscii = {},
        // User and Custom are valid.
        User = {},
        Custom = { tag = "custom" },
        Hello = { tag = "Hellö" },
        Tag = "hi",
        Youre = [ "it" ],
        Bad = {
            tag = "this_is_fine",
            type_name = 42,
            alias = 13,
        },
        "" = {},
    }
}

fn main() {
    // UserGeneration and CustomGeneration should exist.
    let _user = UserGeneration::new();
    let _custom = CustomGeneration::new();
    let _user_kind: UserGenerationKind;
    let _custom_kind: CustomGenerationKind;
}
