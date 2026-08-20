#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccountKind {}
impl ::my_custom_generation::TypedGenerationKind for AccountKind {
    const TAG: ::my_custom_generation::TypedGenerationTag = ::my_custom_generation::TypedGenerationTag::new(
        "account",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(AccountGeneration));
}
const _: () = {
    let _ = <AccountKind as ::my_custom_generation::TypedGenerationKind>::TAG;
};
#[cfg(feature = "internal-schemars08-tests")]
impl ::my_custom_generation::macro_support::schemars08::JsonSchema for AccountKind {
    fn schema_name() -> ::std::string::String {
        "AccountKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_api_service::models::AccountKind")
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
            "my_api_service::models::AccountKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type AccountGeneration = ::my_custom_generation::TypedGeneration<AccountKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionKind {}
impl ::my_custom_generation::TypedGenerationKind for TransactionKind {
    const TAG: ::my_custom_generation::TypedGenerationTag = ::my_custom_generation::TypedGenerationTag::new(
        "transaction",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(TransactionGeneration));
}
const _: () = {
    let _ = <TransactionKind as ::my_custom_generation::TypedGenerationKind>::TAG;
};
#[cfg(feature = "internal-schemars08-tests")]
impl ::my_custom_generation::macro_support::schemars08::JsonSchema for TransactionKind {
    fn schema_name() -> ::std::string::String {
        "TransactionKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_api_service::models::TransactionKind")
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
            "my_api_service::models::TransactionKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type TransactionGeneration = ::my_custom_generation::TypedGeneration<
    TransactionKind,
>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiKeyKind {}
impl ::my_custom_generation::TypedGenerationKind for ApiKeyKind {
    const TAG: ::my_custom_generation::TypedGenerationTag = ::my_custom_generation::TypedGenerationTag::new(
        "api_key",
    );
    const ALIAS: Option<&'static str> = Some(stringify!(ApiKeyGeneration));
}
const _: () = {
    let _ = <ApiKeyKind as ::my_custom_generation::TypedGenerationKind>::TAG;
};
#[cfg(feature = "internal-schemars08-tests")]
impl ::my_custom_generation::macro_support::schemars08::JsonSchema for ApiKeyKind {
    fn schema_name() -> ::std::string::String {
        "ApiKeyKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_api_service::models::ApiKeyKind")
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
            "my_api_service::models::ApiKeyKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type ApiKeyGeneration = ::my_custom_generation::TypedGeneration<ApiKeyKind>;
