use my_custom_generation::TypedGenerationKind;
use oxide_typed_generation_macros::impl_typed_generation_kinds;

impl_typed_generation_kinds! {
    settings = {
        oxide_typed_generation_crate = my_custom_generation,
        schemars08 = {
            attrs = [#[cfg(feature = "internal-schemars08-tests")]],
            rust_type = {
                crate = "my-api-service",
                version = "2.1.0",
                path = "my_api_service::models",
            },
        },
    },
    kinds = {
        Account = {},
        Transaction = {},
        ApiKey = {},
    }
}

fn main() {
    // Test that the generated types exist and work with custom crate alias and schemars
    let _account_kind_tag = AccountKind::TAG;
    let _transaction_kind_tag = TransactionKind::TAG;
    let _api_key_kind_tag = ApiKeyKind::TAG;

    // Test type aliases exist
    let _account_generation: AccountGeneration;
    let _transaction_generation: TransactionGeneration;
    let _api_key_generation: ApiKeyGeneration;

    // Test that tags are properly snake_cased
    assert_eq!(AccountKind::TAG.as_str(), "account");
    assert_eq!(TransactionKind::TAG.as_str(), "transaction");
    assert_eq!(ApiKeyKind::TAG.as_str(), "api_key");
}
