#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTTPClientKind {}
impl ::oxide_generation::TypedGenerationKind for HTTPClientKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "http_client",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(HTTPClientGeneration));
}
const _: () = {
    let _ = <HTTPClientKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type HTTPClientGeneration = ::oxide_generation::TypedGeneration<HTTPClientKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XMLParserKind {}
impl ::oxide_generation::TypedGenerationKind for XMLParserKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "xml_parser",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(XMLParserGeneration));
}
const _: () = {
    let _ = <XMLParserKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type XMLParserGeneration = ::oxide_generation::TypedGeneration<XMLParserKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APIKeyKind {}
impl ::oxide_generation::TypedGenerationKind for APIKeyKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "api_key",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(APIKeyGeneration));
}
const _: () = {
    let _ = <APIKeyKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type APIKeyGeneration = ::oxide_generation::TypedGeneration<APIKeyKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOHandlerKind {}
impl ::oxide_generation::TypedGenerationKind for IOHandlerKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "io_handler",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(IOHandlerGeneration));
}
const _: () = {
    let _ = <IOHandlerKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type IOHandlerGeneration = ::oxide_generation::TypedGeneration<IOHandlerKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserAccountKind {}
impl ::oxide_generation::TypedGenerationKind for UserAccountKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "user_account",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(UserAccountGeneration));
}
const _: () = {
    let _ = <UserAccountKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type UserAccountGeneration = ::oxide_generation::TypedGeneration<UserAccountKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectTaskKind {}
impl ::oxide_generation::TypedGenerationKind for ProjectTaskKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "project_task",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(ProjectTaskGeneration));
}
const _: () = {
    let _ = <ProjectTaskKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[allow(unused)]
pub type ProjectTaskGeneration = ::oxide_generation::TypedGeneration<ProjectTaskKind>;
