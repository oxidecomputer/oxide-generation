//! Tests for the property-based infrastructure.

use oxide_typed_generation::{
    Generation, GenericGeneration, TypedGeneration, TypedGenerationKind, TypedGenerationTag,
};
use test_strategy::proptest;

#[derive(Debug)]
enum MyKind {}

impl TypedGenerationKind for MyKind {
    const TAG: TypedGenerationTag = TypedGenerationTag::new("my_kind");
}

/// Ensure that generated generation numbers are always within range.
#[proptest]
fn prop_within_range(generation: TypedGeneration<MyKind>) {
    assert!(generation <= TypedGeneration::<MyKind>::MAX);
}

/// Ensure that `checked_next` returns `Some` exactly when the value is below
/// the maximum, and that the result is strictly greater.
#[proptest]
fn prop_checked_next(generation: TypedGeneration<MyKind>) {
    match generation.checked_next() {
        Some(next) => {
            assert!(generation < TypedGeneration::<MyKind>::MAX);
            assert!(next > generation);
            assert_eq!(next.as_u64(), generation.as_u64() + 1);
        }
        None => {
            assert_eq!(generation, TypedGeneration::<MyKind>::MAX);
        }
    }
}

/// Ensure that `prev` followed by `next` round-trips, and that `prev` is `None`
/// exactly for the first generation and below.
#[proptest]
fn prop_prev_next_roundtrip(generation: TypedGeneration<MyKind>) {
    match generation.prev() {
        Some(prev) => {
            assert!(generation.as_u64() > 1);
            assert_eq!(prev.next(), generation);
        }
        None => {
            assert!(generation.as_u64() <= 1);
        }
    }
}

/// Ensure that typed and untyped conversions agree.
#[proptest]
fn prop_untyped_conversions(typed_generation: TypedGeneration<MyKind>) {
    let untyped_generation: &Generation = typed_generation.as_untyped_generation();
    assert_eq!(typed_generation.as_u64(), untyped_generation.as_u64());
    assert_eq!(typed_generation.as_i64(), untyped_generation.as_i64());

    let typed_u64: u64 = typed_generation.into();
    let untyped_u64: u64 = typed_generation.into_untyped_generation().into();
    assert_eq!(typed_u64, untyped_u64);

    let typed_i64: i64 = typed_generation.into();
    let untyped_i64: i64 = typed_generation.into_untyped_generation().into();
    assert_eq!(typed_i64, untyped_i64);
}

/// Ensure that generation numbers round-trip through JSON.
#[cfg(feature = "internal-schemars08-tests")]
#[proptest]
fn prop_serde_roundtrip(typed_generation: TypedGeneration<MyKind>) {
    let json = serde_json::to_string(&typed_generation).unwrap();
    let deserialized: TypedGeneration<MyKind> = serde_json::from_str(&json).unwrap();
    assert_eq!(typed_generation, deserialized);

    // The typed representation is the same as the untyped one.
    let untyped_generation = typed_generation.into_untyped_generation();
    let untyped_json = serde_json::to_string(&untyped_generation).unwrap();
    assert_eq!(json, untyped_json);

    let deserialized: Generation = serde_json::from_str(&untyped_json).unwrap();
    assert_eq!(untyped_generation, deserialized);
}
