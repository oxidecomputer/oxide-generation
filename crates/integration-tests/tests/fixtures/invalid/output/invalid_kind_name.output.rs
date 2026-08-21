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
pub enum CustomKind {}
impl ::oxide_generation::TypedGenerationKind for CustomKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "custom",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(CustomGeneration));
}
const _: () = {
    let _ = <CustomKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type CustomGeneration = ::oxide_generation::TypedGeneration<CustomKind>;
