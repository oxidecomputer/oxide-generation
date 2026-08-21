//! Basic usage example for the `impl_typed_generation_kinds` macro.
//!
//! This example demonstrates how to use the macro to generate typed generation kinds
//! and their corresponding type aliases.

use oxide_generation::TypedGenerationKind;
use oxide_generation_macros::impl_typed_generation_kinds;

// Generate typed generation kinds for different entities in our application.
impl_typed_generation_kinds! {
    kinds = {
        User = {},
        Organization = {},
        Project = {},
    }
}

// The above macro generates:
//
// * pub enum UserGenerationKind {}
// * pub type UserGeneration = TypedGeneration<UserGenerationKind>;
// * pub type OrganizationGenerationKind {}
// * pub type OrganizationGeneration = TypedGeneration<OrganizationGenerationKind>;
// * pub type ProjectGenerationKind {}
// * pub type ProjectGeneration = TypedGeneration<ProjectGenerationKind>;

fn main() {
    // Create some generation numbers of different types.
    let user_generation = UserGeneration::new();
    let org_generation = OrganizationGeneration::new();
    let project_generation = ProjectGeneration::new();

    // Print the generation numbers and their tags.
    println!(
        "User generation: {} (tag: {})",
        user_generation,
        UserGenerationKind::TAG
    );
    println!(
        "Organization generation: {} (tag: {})",
        org_generation,
        OrganizationGenerationKind::TAG
    );
    println!(
        "Project generation: {} (tag: {})",
        project_generation,
        ProjectGenerationKind::TAG
    );

    // The compiler ensures type safety -- you can't accidentally mix up types.
    // This would be a compile error:
    // let _error: UserGeneration = typed_org;  // Error: mismatched types
}
