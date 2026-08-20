#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserKind {}
impl ::my_custom_generation::TypedGenerationKind for UserKind {
    #[inline]
    fn tag() -> ::my_custom_generation::TypedGenerationTag {
        const TAG: ::my_custom_generation::TypedGenerationTag = ::my_custom_generation::TypedGenerationTag::new(
            "user",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(UserGeneration))
    }
}
#[allow(unused)]
pub type UserGeneration = ::my_custom_generation::TypedGeneration<UserKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrganizationKind {}
impl ::my_custom_generation::TypedGenerationKind for OrganizationKind {
    #[inline]
    fn tag() -> ::my_custom_generation::TypedGenerationTag {
        const TAG: ::my_custom_generation::TypedGenerationTag = ::my_custom_generation::TypedGenerationTag::new(
            "organization",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(OrganizationGeneration))
    }
}
#[allow(unused)]
pub type OrganizationGeneration = ::my_custom_generation::TypedGeneration<
    OrganizationKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductKind {}
impl ::my_custom_generation::TypedGenerationKind for ProductKind {
    #[inline]
    fn tag() -> ::my_custom_generation::TypedGenerationTag {
        const TAG: ::my_custom_generation::TypedGenerationTag = ::my_custom_generation::TypedGenerationTag::new(
            "product",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(ProductGeneration))
    }
}
#[allow(unused)]
pub type ProductGeneration = ::my_custom_generation::TypedGeneration<ProductKind>;
