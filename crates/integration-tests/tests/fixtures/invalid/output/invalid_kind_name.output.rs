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
pub enum CustomKind {}
impl ::oxide_typed_generation::TypedGenerationKind for CustomKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "custom",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(CustomGeneration))
    }
}
#[allow(unused)]
pub type CustomGeneration = ::oxide_typed_generation::TypedGeneration<CustomKind>;
