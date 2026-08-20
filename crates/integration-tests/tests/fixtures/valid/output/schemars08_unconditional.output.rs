#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserKind {}
impl ::oxide_typed_generation::TypedGenerationKind for UserKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "user",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(UserGeneration))
    }
}
impl ::oxide_typed_generation::macro_support::schemars08::JsonSchema for UserKind {
    fn schema_name() -> ::std::string::String {
        "UserKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_service::types::UserKind")
    }
    fn json_schema(
        _gen: &mut ::oxide_typed_generation::macro_support::schemars08::r#gen::SchemaGenerator,
    ) -> ::oxide_typed_generation::macro_support::schemars08::schema::Schema {
        use ::oxide_typed_generation::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::oxide_typed_generation::macro_support::schemars08::Map::new();
        let rust_type = ::oxide_typed_generation::macro_support::serde_json::json!(
            { "crate" : "my-service", "version" : "1.0.0", "path" :
            "my_service::types::UserKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type UserGeneration = ::oxide_typed_generation::TypedGeneration<UserKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrganizationKind {}
impl ::oxide_typed_generation::TypedGenerationKind for OrganizationKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "organization",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(OrganizationGeneration))
    }
}
impl ::oxide_typed_generation::macro_support::schemars08::JsonSchema
for OrganizationKind {
    fn schema_name() -> ::std::string::String {
        "OrganizationKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_service::types::OrganizationKind")
    }
    fn json_schema(
        _gen: &mut ::oxide_typed_generation::macro_support::schemars08::r#gen::SchemaGenerator,
    ) -> ::oxide_typed_generation::macro_support::schemars08::schema::Schema {
        use ::oxide_typed_generation::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::oxide_typed_generation::macro_support::schemars08::Map::new();
        let rust_type = ::oxide_typed_generation::macro_support::serde_json::json!(
            { "crate" : "my-service", "version" : "1.0.0", "path" :
            "my_service::types::OrganizationKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type OrganizationGeneration = ::oxide_typed_generation::TypedGeneration<
    OrganizationKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectKind {}
impl ::oxide_typed_generation::TypedGenerationKind for ProjectKind {
    #[inline]
    fn tag() -> ::oxide_typed_generation::TypedGenerationTag {
        const TAG: ::oxide_typed_generation::TypedGenerationTag = ::oxide_typed_generation::TypedGenerationTag::new(
            "project",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(ProjectGeneration))
    }
}
impl ::oxide_typed_generation::macro_support::schemars08::JsonSchema for ProjectKind {
    fn schema_name() -> ::std::string::String {
        "ProjectKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_service::types::ProjectKind")
    }
    fn json_schema(
        _gen: &mut ::oxide_typed_generation::macro_support::schemars08::r#gen::SchemaGenerator,
    ) -> ::oxide_typed_generation::macro_support::schemars08::schema::Schema {
        use ::oxide_typed_generation::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::oxide_typed_generation::macro_support::schemars08::Map::new();
        let rust_type = ::oxide_typed_generation::macro_support::serde_json::json!(
            { "crate" : "my-service", "version" : "1.0.0", "path" :
            "my_service::types::ProjectKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type ProjectGeneration = ::oxide_typed_generation::TypedGeneration<ProjectKind>;
