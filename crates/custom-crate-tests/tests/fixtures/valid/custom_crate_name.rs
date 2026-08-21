use my_custom_generation::TypedGenerationKind;
use oxide_generation_macros::impl_typed_generation_kinds;

impl_typed_generation_kinds! {
    settings = {
        oxide_generation_crate = my_custom_generation,
    },
    kinds = {
        User = {},
        Organization = {},
        Product = {},
    }
}

fn main() {
    // Test that the generated types exist and work with custom crate alias
    let _user_kind_tag = UserGenerationKind::TAG;
    let _org_kind_tag = OrganizationGenerationKind::TAG;
    let _product_kind_tag = ProductGenerationKind::TAG;

    // Test type aliases exist
    let _user_generation: UserGeneration;
    let _org_generation: OrganizationGeneration;
    let _product_generation: ProductGeneration;

    // Test that tags are properly snake_cased
    assert_eq!(UserGenerationKind::TAG.as_str(), "user");
    assert_eq!(OrganizationGenerationKind::TAG.as_str(), "organization");
    assert_eq!(ProductGenerationKind::TAG.as_str(), "product");
}
