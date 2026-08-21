#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTTPClientGenerationKind {}
impl ::oxide_generation::TypedGenerationKind for HTTPClientGenerationKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "http_client",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(HTTPClientGeneration));
}
const _: () = {
    let _ = <HTTPClientGenerationKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type HTTPClientGeneration = ::oxide_generation::TypedGeneration<
    HTTPClientGenerationKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XMLParserGenerationKind {}
impl ::oxide_generation::TypedGenerationKind for XMLParserGenerationKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "xml_parser",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(XMLParserGeneration));
}
const _: () = {
    let _ = <XMLParserGenerationKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type XMLParserGeneration = ::oxide_generation::TypedGeneration<
    XMLParserGenerationKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APIKeyGenerationKind {}
impl ::oxide_generation::TypedGenerationKind for APIKeyGenerationKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "api_key",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(APIKeyGeneration));
}
const _: () = {
    let _ = <APIKeyGenerationKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type APIKeyGeneration = ::oxide_generation::TypedGeneration<APIKeyGenerationKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOHandlerGenerationKind {}
impl ::oxide_generation::TypedGenerationKind for IOHandlerGenerationKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "io_handler",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(IOHandlerGeneration));
}
const _: () = {
    let _ = <IOHandlerGenerationKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type IOHandlerGeneration = ::oxide_generation::TypedGeneration<
    IOHandlerGenerationKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserAccountGenerationKind {}
impl ::oxide_generation::TypedGenerationKind for UserAccountGenerationKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "user_account",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(UserAccountGeneration));
}
const _: () = {
    let _ = <UserAccountGenerationKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type UserAccountGeneration = ::oxide_generation::TypedGeneration<
    UserAccountGenerationKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectTaskGenerationKind {}
impl ::oxide_generation::TypedGenerationKind for ProjectTaskGenerationKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "project_task",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(ProjectTaskGeneration));
}
const _: () = {
    let _ = <ProjectTaskGenerationKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type ProjectTaskGeneration = ::oxide_generation::TypedGeneration<
    ProjectTaskGenerationKind,
>;
