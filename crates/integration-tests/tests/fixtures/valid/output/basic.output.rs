#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserGenerationKind {}
impl ::oxide_generation::TypedGenerationKind for UserGenerationKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "user",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(UserGeneration));
}
const _: () = {
    let _ = <UserGenerationKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type UserGeneration = ::oxide_generation::TypedGeneration<UserGenerationKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrganizationGenerationKind {}
impl ::oxide_generation::TypedGenerationKind for OrganizationGenerationKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "organization",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(OrganizationGeneration));
}
const _: () = {
    let _ = <OrganizationGenerationKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type OrganizationGeneration = ::oxide_generation::TypedGeneration<
    OrganizationGenerationKind,
>;
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
