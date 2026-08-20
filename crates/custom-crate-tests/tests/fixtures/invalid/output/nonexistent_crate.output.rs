#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserKind {}
impl ::nonexistent_crate::TypedGenerationKind for UserKind {
    const TAG: ::nonexistent_crate::TypedGenerationTag = ::nonexistent_crate::TypedGenerationTag::new(
        "user",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(UserGeneration));
}
const _: () = {
    let _ = <UserKind as ::nonexistent_crate::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type UserGeneration = ::nonexistent_crate::TypedGeneration<UserKind>;
