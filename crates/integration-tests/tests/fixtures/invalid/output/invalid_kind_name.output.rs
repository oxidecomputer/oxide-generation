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
pub enum CustomGenerationKind {}
impl ::oxide_generation::TypedGenerationKind for CustomGenerationKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "custom",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(CustomGeneration));
}
const _: () = {
    let _ = <CustomGenerationKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type CustomGeneration = ::oxide_generation::TypedGeneration<CustomGenerationKind>;
