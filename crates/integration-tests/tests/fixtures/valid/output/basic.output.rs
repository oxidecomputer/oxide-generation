#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserKind {}
impl ::oxide_generation::TypedGenerationKind for UserKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "user",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(UserGeneration));
}
const _: () = {
    let _ = <UserKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type UserGeneration = ::oxide_generation::TypedGeneration<UserKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrganizationKind {}
impl ::oxide_generation::TypedGenerationKind for OrganizationKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "organization",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(OrganizationGeneration));
}
const _: () = {
    let _ = <OrganizationKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type OrganizationGeneration = ::oxide_generation::TypedGeneration<OrganizationKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectKind {}
impl ::oxide_generation::TypedGenerationKind for ProjectKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "project",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(ProjectGeneration));
}
const _: () = {
    let _ = <ProjectKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type ProjectGeneration = ::oxide_generation::TypedGeneration<ProjectKind>;
