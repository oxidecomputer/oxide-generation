use oxide_generation::TypedGenerationKind;
use oxide_generation_macros::impl_typed_generation_kinds;

impl_typed_generation_kinds! {
    settings = {
        schemars08 = {
            attrs = [#[cfg(feature = "internal-schemars08-tests")]],
            rust_type = {
                crate = "my-service",
                version = "1.0.0",
                path = "my_service::types",
            },
        },
    },
    kinds = {
        User = {},
        Organization = {},
        Project = {},
    }
}

fn main() {
    // Test that the generated types exist and work
    let _user_kind_tag = UserKind::TAG;
    let _org_kind_tag = OrganizationKind::TAG;
    let _project_kind_tag = ProjectKind::TAG;

    // Test type aliases exist
    let _user_generation: UserGeneration;
    let _org_generation: OrganizationGeneration;
    let _project_generation: ProjectGeneration;

    // Test that tags are properly snake_cased
    assert_eq!(UserKind::TAG.as_str(), "user");
    assert_eq!(OrganizationKind::TAG.as_str(), "organization");
    assert_eq!(ProjectKind::TAG.as_str(), "project");
}
