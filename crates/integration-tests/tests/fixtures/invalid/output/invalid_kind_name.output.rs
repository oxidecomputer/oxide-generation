#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserKind {}
impl ::oxide_typed_generation::TypedGenerationKind for UserKind {
    const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
        "user",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(UserGeneration));
}
const _: () = {
    let _ = <UserKind as ::oxide_typed_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type UserGeneration = ::oxide_typed_generation::TypedGeneration<UserKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CustomKind {}
impl ::oxide_typed_generation::TypedGenerationKind for CustomKind {
    const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
        "custom",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(CustomGeneration));
}
const _: () = {
    let _ = <CustomKind as ::oxide_typed_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type CustomGeneration = ::oxide_typed_generation::TypedGeneration<CustomKind>;
