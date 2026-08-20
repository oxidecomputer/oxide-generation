use my_custom_generation::TypedGenerationKind;
use oxide_typed_generation_macros::impl_typed_generation_kinds;

impl_typed_generation_kinds! {
    settings = {
        oxide_typed_generation_crate = my_custom_generation,
    },
    kinds = {
        User = {},
        Organization = {},
        Product = {},
    }
}

fn main() {
    // Test that the generated types exist and work with custom crate alias
    let _user_kind_tag = UserKind::tag();
    let _org_kind_tag = OrganizationKind::tag();
    let _product_kind_tag = ProductKind::tag();

    // Test type aliases exist
    let _user_generation: UserGeneration;
    let _org_generation: OrganizationGeneration;
    let _product_generation: ProductGeneration;

    // Test that tags are properly snake_cased
    assert_eq!(UserKind::tag().as_str(), "user");
    assert_eq!(OrganizationKind::tag().as_str(), "organization");
    assert_eq!(ProductKind::tag().as_str(), "product");
}
