#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserGenerationKind {}
impl ::my_custom_generation::TypedGenerationKind for UserGenerationKind {
    const TAG: ::my_custom_generation::TypedGenerationTag = ::my_custom_generation::TypedGenerationTag::new(
        "user",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(UserGeneration));
}
const _: () = {
    let _ = <UserGenerationKind as ::my_custom_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type UserGeneration = ::my_custom_generation::TypedGeneration<UserGenerationKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrganizationGenerationKind {}
impl ::my_custom_generation::TypedGenerationKind for OrganizationGenerationKind {
    const TAG: ::my_custom_generation::TypedGenerationTag = ::my_custom_generation::TypedGenerationTag::new(
        "organization",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(OrganizationGeneration));
}
const _: () = {
    let _ = <OrganizationGenerationKind as ::my_custom_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type OrganizationGeneration = ::my_custom_generation::TypedGeneration<
    OrganizationGenerationKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductGenerationKind {}
impl ::my_custom_generation::TypedGenerationKind for ProductGenerationKind {
    const TAG: ::my_custom_generation::TypedGenerationTag = ::my_custom_generation::TypedGenerationTag::new(
        "product",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(ProductGeneration));
}
const _: () = {
    let _ = <ProductGenerationKind as ::my_custom_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type ProductGeneration = ::my_custom_generation::TypedGeneration<
    ProductGenerationKind,
>;
