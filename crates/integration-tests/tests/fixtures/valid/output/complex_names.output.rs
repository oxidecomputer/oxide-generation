#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTTPClientKind {}
impl ::oxide_typed_generation::TypedGenerationKind for HTTPClientKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "http_client",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(HTTPClientGeneration))
    }
}
#[allow(unused)]
pub type HTTPClientGeneration = ::oxide_typed_generation::TypedGeneration<
    HTTPClientKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XMLParserKind {}
impl ::oxide_typed_generation::TypedGenerationKind for XMLParserKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "xml_parser",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(XMLParserGeneration))
    }
}
#[allow(unused)]
pub type XMLParserGeneration = ::oxide_typed_generation::TypedGeneration<XMLParserKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APIKeyKind {}
impl ::oxide_typed_generation::TypedGenerationKind for APIKeyKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "api_key",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(APIKeyGeneration))
    }
}
#[allow(unused)]
pub type APIKeyGeneration = ::oxide_typed_generation::TypedGeneration<APIKeyKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOHandlerKind {}
impl ::oxide_typed_generation::TypedGenerationKind for IOHandlerKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "io_handler",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(IOHandlerGeneration))
    }
}
#[allow(unused)]
pub type IOHandlerGeneration = ::oxide_typed_generation::TypedGeneration<IOHandlerKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserAccountKind {}
impl ::oxide_typed_generation::TypedGenerationKind for UserAccountKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "user_account",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(UserAccountGeneration))
    }
}
#[allow(unused)]
pub type UserAccountGeneration = ::oxide_typed_generation::TypedGeneration<
    UserAccountKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectTaskKind {}
impl ::oxide_typed_generation::TypedGenerationKind for ProjectTaskKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "project_task",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(ProjectTaskGeneration))
    }
}
#[allow(unused)]
pub type ProjectTaskGeneration = ::oxide_typed_generation::TypedGeneration<
    ProjectTaskKind,
>;
