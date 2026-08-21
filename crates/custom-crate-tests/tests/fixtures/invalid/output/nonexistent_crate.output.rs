#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserGenerationKind {}
impl ::nonexistent_crate::TypedGenerationKind for UserGenerationKind {
    const TAG: ::nonexistent_crate::TypedGenerationTag = ::nonexistent_crate::TypedGenerationTag::new(
        "user",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(UserGeneration));
}
const _: () = {
    let _ = <UserGenerationKind as ::nonexistent_crate::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type UserGeneration = ::nonexistent_crate::TypedGeneration<UserGenerationKind>;
