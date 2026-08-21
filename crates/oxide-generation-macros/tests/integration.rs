use oxide_generation::TypedGenerationKind;
use oxide_generation_macros::impl_typed_generation_kinds;
use static_assertions::{assert_impl_all, assert_not_impl_all};
use std::{fmt, hash::Hash};

impl_typed_generation_kinds! {
    kinds = {
        User = {},
        Organization = {},
        Project = {},
    }
}

#[test]
fn test_generated_kinds() {
    // Test that the generated kinds have the correct tags.
    assert_eq!(UserGenerationKind::TAG.as_str(), "user");
    assert_eq!(OrganizationGenerationKind::TAG.as_str(), "organization");
    assert_eq!(ProjectGenerationKind::TAG.as_str(), "project");
}

#[test]
fn test_single_entry() {
    impl_typed_generation_kinds! {
        kinds = {
            Single = {},
        }
    }

    assert_eq!(SingleGenerationKind::TAG.as_str(), "single");
    let single_generation = SingleGeneration::new();
    assert_eq!(single_generation.as_u64(), 1);
}

#[test]
fn test_snake_case_conversion() {
    impl_typed_generation_kinds! {
        kinds = {
            // The Rust convention is to use `HttpClient`, `XmlParser`, etc, but
            // ensure that these all-caps variants also have the correct tags.
            HTTPClient = {},
            XMLParser = {},
            UserAccount = {},
            IOHandler = {},
        }
    }

    assert_eq!(HTTPClientGenerationKind::TAG.as_str(), "http_client");
    assert_eq!(XMLParserGenerationKind::TAG.as_str(), "xml_parser");
    assert_eq!(UserAccountGenerationKind::TAG.as_str(), "user_account");
    assert_eq!(IOHandlerGenerationKind::TAG.as_str(), "io_handler");
}

#[test]
fn test_empty_kinds() {
    // Test that we can handle an empty kinds map.
    impl_typed_generation_kinds! {
        kinds = {}
    }
}

/// Test that global `attrs` are applied to all generated kinds.
#[test]
fn test_global_attrs_param() {
    impl_typed_generation_kinds! {
        settings = {
            attrs = [#[derive(Hash)]],
        },
        kinds = {
            GlobalA = {},
            GlobalB = {},
        }
    }

    // The GlobalA and GlobalB kinds should impl Clone + Copy + fmt::Debug + Eq
    // by default, and also implement Hash.
    assert_impl_all!(GlobalAGenerationKind: Clone, Copy, fmt::Debug, Eq, Hash);
    assert_impl_all!(GlobalBGenerationKind: Clone, Copy, fmt::Debug, Eq, Hash);
}

/// Test that per-kind `attrs` override global ones and are applied only to the
/// correct kind.
#[test]
fn test_per_kind_attrs_param() {
    impl_typed_generation_kinds! {
        settings = {
            attrs = [#[derive(Hash)]],
        },
        kinds = {
            A = {},
            B = {
                attrs = [],
            },
            C = {
                attrs = [#[derive(Ord, PartialOrd)]],
            },
        }
    }

    // AGenerationKind should implement Hash.
    assert_impl_all!(AGenerationKind: Hash);

    // BGenerationKind should not implement Hash.
    assert_not_impl_all!(BGenerationKind: Hash);

    // CGenerationKind should implement Ord and PartialOrd, but not Hash.
    assert_impl_all!(CGenerationKind: Ord, PartialOrd);
    assert_not_impl_all!(CGenerationKind: Hash);
}
