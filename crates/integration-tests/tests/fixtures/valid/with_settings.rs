use oxide_typed_generation::TypedGenerationKind;
use oxide_typed_generation_macros::impl_typed_generation_kinds;

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
    let _user_kind_tag = UserKind::tag();
    let _org_kind_tag = OrganizationKind::tag();
    let _project_kind_tag = ProjectKind::tag();

    // Test type aliases exist
    let _user_generation: UserGeneration;
    let _org_generation: OrganizationGeneration;
    let _project_generation: ProjectGeneration;

    // Test that tags are properly snake_cased
    assert_eq!(UserKind::tag().as_str(), "user");
    assert_eq!(OrganizationKind::tag().as_str(), "organization");
    assert_eq!(ProjectKind::tag().as_str(), "project");
}
