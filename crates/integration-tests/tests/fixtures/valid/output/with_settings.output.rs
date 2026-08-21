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
#[cfg(feature = "internal-schemars08-tests")]
impl ::oxide_generation::macro_support::schemars08::JsonSchema for UserGenerationKind {
    fn schema_name() -> ::std::string::String {
        "UserGenerationKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_service::types::UserGenerationKind")
    }
    fn json_schema(
        _gen: &mut ::oxide_generation::macro_support::schemars08::r#gen::SchemaGenerator,
    ) -> ::oxide_generation::macro_support::schemars08::schema::Schema {
        use ::oxide_generation::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::oxide_generation::macro_support::schemars08::Map::new();
        let rust_type = ::oxide_generation::macro_support::serde_json::json!(
            { "crate" : "my-service", "version" : "1.0.0", "path" :
            "my_service::types::UserGenerationKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type UserGeneration = ::oxide_generation::TypedGeneration<UserGenerationKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrganizationGenerationKind {}
impl ::oxide_generation::TypedGenerationKind for OrganizationGenerationKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "organization",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(OrganizationGeneration));
}
const _: () = {
    let _ = <OrganizationGenerationKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[cfg(feature = "internal-schemars08-tests")]
impl ::oxide_generation::macro_support::schemars08::JsonSchema
for OrganizationGenerationKind {
    fn schema_name() -> ::std::string::String {
        "OrganizationGenerationKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_service::types::OrganizationGenerationKind")
    }
    fn json_schema(
        _gen: &mut ::oxide_generation::macro_support::schemars08::r#gen::SchemaGenerator,
    ) -> ::oxide_generation::macro_support::schemars08::schema::Schema {
        use ::oxide_generation::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::oxide_generation::macro_support::schemars08::Map::new();
        let rust_type = ::oxide_generation::macro_support::serde_json::json!(
            { "crate" : "my-service", "version" : "1.0.0", "path" :
            "my_service::types::OrganizationGenerationKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type OrganizationGeneration = ::oxide_generation::TypedGeneration<
    OrganizationGenerationKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectGenerationKind {}
impl ::oxide_generation::TypedGenerationKind for ProjectGenerationKind {
    const TAG: ::oxide_generation::TypedGenerationTag = ::oxide_generation::TypedGenerationTag::new(
        "project",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(ProjectGeneration));
}
const _: () = {
    let _ = <ProjectGenerationKind as ::oxide_generation::TypedGenerationKind>::TAG;
};
#[cfg(feature = "internal-schemars08-tests")]
impl ::oxide_generation::macro_support::schemars08::JsonSchema
for ProjectGenerationKind {
    fn schema_name() -> ::std::string::String {
        "ProjectGenerationKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_service::types::ProjectGenerationKind")
    }
    fn json_schema(
        _gen: &mut ::oxide_generation::macro_support::schemars08::r#gen::SchemaGenerator,
    ) -> ::oxide_generation::macro_support::schemars08::schema::Schema {
        use ::oxide_generation::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::oxide_generation::macro_support::schemars08::Map::new();
        let rust_type = ::oxide_generation::macro_support::serde_json::json!(
            { "crate" : "my-service", "version" : "1.0.0", "path" :
            "my_service::types::ProjectGenerationKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type ProjectGeneration = ::oxide_generation::TypedGeneration<ProjectGenerationKind>;
