#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccountGenerationKind {}
impl ::my_custom_generation::TypedGenerationKind for AccountGenerationKind {
    const TAG: ::my_custom_generation::TypedGenerationTag = ::my_custom_generation::TypedGenerationTag::new(
        "account",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(AccountGeneration));
}
const _: () = {
    let _ = <AccountGenerationKind as ::my_custom_generation::TypedGenerationKind>::TAG;
};
#[cfg(feature = "internal-schemars08-tests")]
impl ::my_custom_generation::macro_support::schemars08::JsonSchema
for AccountGenerationKind {
    fn schema_name() -> ::std::string::String {
        "AccountGenerationKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_api_service::models::AccountGenerationKind")
    }
    fn json_schema(
        _gen: &mut ::my_custom_generation::macro_support::schemars08::r#gen::SchemaGenerator,
    ) -> ::my_custom_generation::macro_support::schemars08::schema::Schema {
        use ::my_custom_generation::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::my_custom_generation::macro_support::schemars08::Map::new();
        let rust_type = ::my_custom_generation::macro_support::serde_json::json!(
            { "crate" : "my-api-service", "version" : "2.1.0", "path" :
            "my_api_service::models::AccountGenerationKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type AccountGeneration = ::my_custom_generation::TypedGeneration<
    AccountGenerationKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionGenerationKind {}
impl ::my_custom_generation::TypedGenerationKind for TransactionGenerationKind {
    const TAG: ::my_custom_generation::TypedGenerationTag = ::my_custom_generation::TypedGenerationTag::new(
        "transaction",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(TransactionGeneration));
}
const _: () = {
    let _ = <TransactionGenerationKind as ::my_custom_generation::TypedGenerationKind>::TAG;
};
#[cfg(feature = "internal-schemars08-tests")]
impl ::my_custom_generation::macro_support::schemars08::JsonSchema
for TransactionGenerationKind {
    fn schema_name() -> ::std::string::String {
        "TransactionGenerationKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_api_service::models::TransactionGenerationKind")
    }
    fn json_schema(
        _gen: &mut ::my_custom_generation::macro_support::schemars08::r#gen::SchemaGenerator,
    ) -> ::my_custom_generation::macro_support::schemars08::schema::Schema {
        use ::my_custom_generation::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::my_custom_generation::macro_support::schemars08::Map::new();
        let rust_type = ::my_custom_generation::macro_support::serde_json::json!(
            { "crate" : "my-api-service", "version" : "2.1.0", "path" :
            "my_api_service::models::TransactionGenerationKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type TransactionGeneration = ::my_custom_generation::TypedGeneration<
    TransactionGenerationKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiKeyGenerationKind {}
impl ::my_custom_generation::TypedGenerationKind for ApiKeyGenerationKind {
    const TAG: ::my_custom_generation::TypedGenerationTag = ::my_custom_generation::TypedGenerationTag::new(
        "api_key",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(ApiKeyGeneration));
}
const _: () = {
    let _ = <ApiKeyGenerationKind as ::my_custom_generation::TypedGenerationKind>::TAG;
};
#[cfg(feature = "internal-schemars08-tests")]
impl ::my_custom_generation::macro_support::schemars08::JsonSchema
for ApiKeyGenerationKind {
    fn schema_name() -> ::std::string::String {
        "ApiKeyGenerationKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_api_service::models::ApiKeyGenerationKind")
    }
    fn json_schema(
        _gen: &mut ::my_custom_generation::macro_support::schemars08::r#gen::SchemaGenerator,
    ) -> ::my_custom_generation::macro_support::schemars08::schema::Schema {
        use ::my_custom_generation::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::my_custom_generation::macro_support::schemars08::Map::new();
        let rust_type = ::my_custom_generation::macro_support::serde_json::json!(
            { "crate" : "my-api-service", "version" : "2.1.0", "path" :
            "my_api_service::models::ApiKeyGenerationKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type ApiKeyGeneration = ::my_custom_generation::TypedGeneration<
    ApiKeyGenerationKind,
>;
