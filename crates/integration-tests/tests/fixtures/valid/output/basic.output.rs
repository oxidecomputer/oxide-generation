#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserKind {}
impl ::oxide_typed_generation::TypedGenerationKind for UserKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "user",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(UserGeneration))
    }
}
#[allow(unused)]
pub type UserGeneration = ::oxide_typed_generation::TypedGeneration<UserKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrganizationKind {}
impl ::oxide_typed_generation::TypedGenerationKind for OrganizationKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "organization",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(OrganizationGeneration))
    }
}
#[allow(unused)]
pub type OrganizationGeneration = ::oxide_typed_generation::TypedGeneration<
    OrganizationKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectKind {}
impl ::oxide_typed_generation::TypedGenerationKind for ProjectKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "project",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(ProjectGeneration))
    }
}
#[allow(unused)]
pub type ProjectGeneration = ::oxide_typed_generation::TypedGeneration<ProjectKind>;
