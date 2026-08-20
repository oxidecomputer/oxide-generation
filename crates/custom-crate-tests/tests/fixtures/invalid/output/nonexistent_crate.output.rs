#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserKind {}
impl ::nonexistent_crate::TypedGenerationKind for UserKind {
    #[inline]
    fn tag() -> ::nonexistent_crate::TypedGenerationTag {
        const TAG: ::nonexistent_crate::TypedGenerationTag = ::nonexistent_crate::TypedGenerationTag::new(
            "user",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(UserGeneration))
    }
}
#[allow(unused)]
pub type UserGeneration = ::nonexistent_crate::TypedGeneration<UserKind>;
