use oxide_typed_generation::TypedGenerationKind;
use oxide_typed_generation_macros::impl_typed_generation_kinds;

impl_typed_generation_kinds! {
    kinds = {
        User = {},
        Organization = {},
        Pröject = {
            tag = "project",
            type_name = ProjectKind,
            alias = ProjectGeneration,
        },
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
}
