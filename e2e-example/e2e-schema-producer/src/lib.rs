//! End-to-end schema producer crate.
//!
//! This crate is part of the end-to-end JSON Schema generation and replacment
//! example in this repository. See the parent README for more.

pub use e2e_kinds::{OrganizationGeneration, UserGeneration};
use serde::{Deserialize, Serialize};

/// A simple type which uses the `UserGeneration` and `OrganizationGeneration`
/// types defined in this crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars08", derive(schemars::JsonSchema))]
pub struct ConfigSnapshot {
    pub user_generation: UserGeneration,
    pub organization_generation: OrganizationGeneration,
}

#[cfg(test)]
mod tests {
    /// This test generates the JSON schema checked in at
    /// ../tests/output/config-snapshot-schema.json.
    #[cfg(feature = "schemars08")]
    #[test]
    fn test_generate_config_snapshot_schema() {
        let schema = schemars::schema_for!(super::ConfigSnapshot);
        let schema_json = serde_json::to_string_pretty(&schema).unwrap();
        expectorate::assert_contents("tests/output/config-snapshot-schema.json", &schema_json);
    }
}
