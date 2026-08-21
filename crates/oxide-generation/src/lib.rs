//! Untyped and typed generation numbers.
//!
//! # Motivation
//!
//! Oxide's [Omicron](https://github.com/oxidecomputer/omicron) tracks many
//! different kinds of generation numbers: versions attached to individual
//! resources, configuration generations, and so on. However, a bare integer
//! does not carry information about the kind of counter it belongs to, which
//! can lead to comparing or assigning the wrong generation number at runtime.
//!
//! This crate provides a [`Generation`] type for untyped generation numbers,
//! along with a wrapper type around it that allows you to specify the kind of
//! counter a generation number belongs to.
//!
//! This crate is modeled after [newtype-uuid](https://docs.rs/newtype-uuid).
//! Rust doesn't have [higher-kinded
//! types](https://en.wikipedia.org/wiki/Kind_(type_theory)), so one must write
//! separate libraries for each kind of thing one might want to add sets of
//! newtypes around.
//!
//! # Example
//!
//! ```
//! use oxide_generation::{
//!     GenericGeneration, TypedGeneration, TypedGenerationKind, TypedGenerationTag,
//! };
//!
//! // First, define a type that represents the kind of generation number this is.
//! enum MyKind {}
//!
//! impl TypedGenerationKind for MyKind {
//!     // Tags are required to be ASCII identifiers, with underscores
//!     // and dashes also supported. Because the tag is an associated
//!     // constant, its validity is checked at compile time, at the point
//!     // where the tag is first used.
//!     const TAG: TypedGenerationTag = TypedGenerationTag::new("my_kind");
//! }
//!
//! // Now, a generation number can be created with this kind.
//! let generation: TypedGeneration<MyKind> = "5".parse().unwrap();
//!
//! // The Display (and therefore ToString) impls still show the same value.
//! assert_eq!(generation.to_string(), "5");
//!
//! // The Debug impl will show the tag as well.
//! assert_eq!(format!("{:?}", generation), "5 (my_kind)");
//! ```
//!
//! If you have a large number of generation kinds, consider using
//! [`oxide-generation-macros`] which comes with several convenience features.
//!
//! ```
//! use oxide_generation_macros::impl_typed_generation_kinds;
//!
//! // Invoke this macro with:
//! impl_typed_generation_kinds! {
//!     kinds = {
//!         User = {},
//!         Project = {},
//!         // ...
//!     },
//! }
//! ```
//!
//! See [`oxide-generation-macros`] for more information.
//!
//! [`oxide-generation-macros`]: https://docs.rs/oxide-generation-macros
//!
//! For simpler cases, you can also write your own declarative macro. Use this
//! template to get started:
//!
//! ```rust
//! # use oxide_generation::{TypedGenerationKind, TypedGenerationTag};
//! macro_rules! impl_kinds {
//!     ($($kind:ident => $tag:literal),* $(,)?) => {
//!         $(
//!             pub enum $kind {}
//!
//!             impl TypedGenerationKind for $kind {
//!                 const TAG: TypedGenerationTag = TypedGenerationTag::new($tag);
//!             }
//!         )*
//!     };
//! }
//!
//! // Invoke this macro with:
//! impl_kinds! {
//!     UserKind => "user",
//!     ProjectKind => "project",
//! }
//! ```
//!
//! # Implementations
//!
//! In general, [`TypedGeneration`] uses the same wire and serialization formats as [`Generation`].
//! This means that persistent representations of [`TypedGeneration`] are the same as
//! [`Generation`]; [`TypedGeneration`] is intended to be helpful within Rust code, not across
//! serialization boundaries.
//!
//! - The `Display` and `FromStr` impls are forwarded to the underlying [`Generation`].
//! - If the `serde` feature is enabled, `TypedGeneration` will serialize and deserialize using the
//!   same format as [`Generation`].
//! - If the `schemars08` feature is enabled, [`TypedGeneration`] will implement `JsonSchema` if the
//!   corresponding [`TypedGenerationKind`] implements `JsonSchema`.
//!
//! To abstract over typed and untyped generation numbers, the [`GenericGeneration`] trait is
//! provided. This trait also permits conversions between typed and untyped generation numbers.
//!
//! # Dependencies
//!
//! - This crate has no required dependencies. Optional features may add further dependencies.
//!
//! # Features
//!
//! - `default`: Enables default features in the oxide-generation crate.
//! - `std`: Enables the use of the standard library, and the `std` feature of any enabled
//!   optional dependencies. *Enabled by default.*
//! - `serde`: Enables serialization and deserialization support via Serde. *Not enabled by
//!   default.*
//! - `schemars08`: Enables support for generating JSON schemas via schemars 0.8. *Not enabled by
//!   default.* Note that the format of the generated schema is **not currently part** of the stable
//!   API, though we hope to stabilize it in the future. Enabling this feature also enables `std`.
//! - `proptest1`: Enables support for generating `proptest::Arbitrary` instances of generation
//!   numbers. *Not enabled by default.* Enabling this feature also enables `std`.
//! - `daft01`: Enables diffing support via [`daft`](https://docs.rs/daft) 0.1, treating generation
//!   numbers as leaf values. *Not enabled by default.*
//! - `slog2`: Enables logging support via [`slog`](https://docs.rs/slog) 2.x, emitting generation
//!   numbers as integers. *Not enabled by default.*
//!
//! # Minimum supported Rust version (MSRV)
//!
//! The MSRV of this crate is **Rust 1.85.** In general, this crate will follow the MSRV of its
//! dependencies, with an aim to be conservative.
//!
//! Within the 0.x series, MSRV updates will be accompanied by a minor version bump.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

/// Macro support for [`oxide-generation-macros`].
///
/// This module re-exports types needed for [`oxide-generation-macros`] to work.
///
/// [`oxide-generation-macros`]: https://docs.rs/oxide-generation-macros
#[doc(hidden)]
pub mod macro_support {
    #[cfg(feature = "schemars08")]
    pub use schemars as schemars08;
    #[cfg(feature = "schemars08")]
    pub use serde_json;
}

use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    num::ParseIntError,
    str::FromStr,
};

/// Generation numbers stored in the database, used for optimistic concurrency control
///
/// Generation numbers are monotonic counters: each successive version of a resource is assigned a
/// generation number greater than the one before it.
//
// A generation is a value between 0 and 2**63-1, i.e. equivalent to a u63.
// The reason is that generations are commonly stored as an i64 in databases,
// and we want to disallow negative values. (We could potentially use two's
// complement to store values greater than that as negative values, but surely
// 2**63 is enough.)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Generation(
    // Generations are restricted to 2**63 - 1 as documented above.
    u64,
);

impl Generation {
    /// The generation number before any change has been applied, 0.
    ///
    /// Generation numbers conventionally start at 1 (see [`Self::new`]). Zero is
    /// reserved to represent an initial or empty state that precedes the first
    /// generation.
    pub const ZERO: Generation = Generation(0);

    /// The largest possible generation number, `2**63 - 1`.
    //
    // `as` is a little distasteful because it allows lossy conversion, but we
    // know converting `i64::MAX` to `u64` will always succeed losslessly.
    pub const MAX: Generation = Generation(i64::MAX as u64);

    /// Creates the first generation number, which is 1.
    ///
    /// There is deliberately no `Default` impl: whether a value should start at
    /// [`Self::ZERO`] or at the first generation is a choice callers must make
    /// explicitly.
    #[inline]
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Generation {
        Generation(1)
    }

    /// Creates a generation number from a `u32`.
    ///
    /// Every `u32` is a valid generation number, so this conversion is infallible.
    #[inline]
    #[must_use]
    pub const fn from_u32(value: u32) -> Generation {
        // `as` is a little distasteful because it allows lossy conversion, but
        // (a) we know converting `u32` to `u64` will always succeed
        // losslessly, and (b) it allows to make this function `const`, unlike
        // if we were to use `u64::from(value)`.
        Generation(value as u64)
    }

    /// Returns the next generation number.
    ///
    /// # Panics
    ///
    /// Panics if the next generation number would exceed [`Generation::MAX`]. Use
    /// [`Self::checked_next`] to handle overflow instead.
    #[inline]
    #[must_use]
    pub const fn next(&self) -> Generation {
        // It should technically be an operational error if this wraps or even
        // exceeds the value allowed by an i64.  But it seems unlikely enough to
        // happen in practice that we can probably feel safe with this.
        let next_gen = self.0 + 1;
        assert!(
            next_gen <= Generation::MAX.0,
            "attempt to overflow generation number"
        );
        Generation(next_gen)
    }

    /// Returns the next generation number, or `None` if it would exceed [`Generation::MAX`].
    #[inline]
    #[must_use]
    pub const fn checked_next(&self) -> Option<Generation> {
        let next_gen = self.0 + 1;
        if next_gen <= Generation::MAX.0 {
            Some(Generation(next_gen))
        } else {
            None
        }
    }

    /// Returns the previous generation number, or `None` if this is the first generation.
    ///
    /// Both 1 and [`Self::ZERO`] are treated as having no predecessor: `prev` never
    /// returns [`Self::ZERO`].
    #[inline]
    #[must_use]
    pub const fn prev(&self) -> Option<Generation> {
        if self.0 > 1 {
            Some(Generation(self.0 - 1))
        } else {
            None
        }
    }

    /// Returns the generation number as a `u64`.
    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// Returns the generation number as an `i64`.
    ///
    /// This conversion is infallible because generation numbers are restricted to `2**63 - 1`.
    #[inline]
    pub const fn as_i64(self) -> i64 {
        // `as` is a little distasteful because it allows lossy conversion, but
        // we know that generation numbers never exceed `i64::MAX`, so this
        // conversion always succeeds losslessly.
        self.0 as i64
    }
}

// ---
// Trait impls
// ---

impl fmt::Display for Generation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl FromStr for Generation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try to parse `s` as both an i64 and u64, returning the error from
        // either.
        let _ = i64::from_str(s)?;
        Ok(Generation(u64::from_str(s)?))
    }
}

impl From<u32> for Generation {
    #[inline]
    fn from(value: u32) -> Self {
        Generation::from_u32(value)
    }
}

impl From<Generation> for u64 {
    #[inline]
    fn from(g: Generation) -> Self {
        g.0
    }
}

impl From<Generation> for i64 {
    #[inline]
    fn from(g: Generation) -> Self {
        g.as_i64()
    }
}

impl From<&Generation> for i64 {
    #[inline]
    fn from(g: &Generation) -> Self {
        // We have already validated that the value is within range.
        g.as_i64()
    }
}

impl TryFrom<u64> for Generation {
    type Error = GenerationOverflowError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        i64::try_from(value).map_err(|_| GenerationOverflowError(()))?;
        Ok(Generation(value))
    }
}

impl TryFrom<i64> for Generation {
    type Error = GenerationNegativeError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Generation(
            u64::try_from(value).map_err(|_| GenerationNegativeError(()))?,
        ))
    }
}

/// An error that occurred while converting a `u64` into a [`Generation`].
///
/// The value was greater than [`Generation::MAX`].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct GenerationOverflowError(());

impl fmt::Display for GenerationOverflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("generation number too large")
    }
}

impl core::error::Error for GenerationOverflowError {}

/// An error that occurred while converting an `i64` into a [`Generation`].
///
/// The value was negative.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct GenerationNegativeError(());

impl fmt::Display for GenerationNegativeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("negative generation number")
    }
}

impl core::error::Error for GenerationNegativeError {}

/// A generation number with type-level information about what it's used for.
///
/// For more, see [the library documentation](crate).
#[repr(transparent)]
pub struct TypedGeneration<T: TypedGenerationKind> {
    generation: Generation,
    _phantom: PhantomData<T>,
}

impl<T: TypedGenerationKind> TypedGeneration<T> {
    /// The generation number before any change has been applied, 0.
    ///
    /// Generation numbers conventionally start at 1 (see [`Self::new`]). Zero is
    /// reserved to represent an initial or empty state that precedes the first
    /// generation.
    pub const ZERO: Self = Self {
        generation: Generation::ZERO,
        _phantom: PhantomData,
    };

    /// The largest possible generation number, `2**63 - 1`.
    pub const MAX: Self = Self {
        generation: Generation::MAX,
        _phantom: PhantomData,
    };

    /// Creates the first generation number of this type, which is 1.
    ///
    /// There is deliberately no `Default` impl: whether a value should start at
    /// [`Self::ZERO`] or at the first generation is a choice callers must make
    /// explicitly.
    #[inline]
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            generation: Generation::new(),
            _phantom: PhantomData,
        }
    }

    /// Creates a generation number of this type from a `u32`.
    ///
    /// Every `u32` is a valid generation number, so this conversion is infallible.
    #[inline]
    #[must_use]
    pub const fn from_u32(value: u32) -> Self {
        Self {
            generation: Generation::from_u32(value),
            _phantom: PhantomData,
        }
    }

    /// Returns the next generation number.
    ///
    /// # Panics
    ///
    /// Panics if the next generation number would exceed [`Self::MAX`]. Use
    /// [`Self::checked_next`] to handle overflow instead.
    #[inline]
    #[must_use]
    pub const fn next(&self) -> Self {
        Self {
            generation: self.generation.next(),
            _phantom: PhantomData,
        }
    }

    /// Returns the next generation number, or `None` if it would exceed [`Self::MAX`].
    #[inline]
    #[must_use]
    pub const fn checked_next(&self) -> Option<Self> {
        match self.generation.checked_next() {
            Some(generation) => Some(Self {
                generation,
                _phantom: PhantomData,
            }),
            None => None,
        }
    }

    /// Returns the previous generation number, or `None` if this is the first generation.
    ///
    /// Both 1 and [`Self::ZERO`] are treated as having no predecessor: `prev` never
    /// returns [`Self::ZERO`].
    #[inline]
    #[must_use]
    pub const fn prev(&self) -> Option<Self> {
        match self.generation.prev() {
            Some(generation) => Some(Self {
                generation,
                _phantom: PhantomData,
            }),
            None => None,
        }
    }

    /// Returns the generation number as a `u64`.
    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.generation.as_u64()
    }

    /// Returns the generation number as an `i64`.
    ///
    /// This conversion is infallible because generation numbers are restricted to `2**63 - 1`.
    #[inline]
    pub const fn as_i64(self) -> i64 {
        self.generation.as_i64()
    }

    /// Converts the generation number to one with looser semantics.
    ///
    /// By default, generation kinds are considered independent, and conversions
    /// between them must happen via the [`GenericGeneration`] interface. But in
    /// some cases, there may be a relationship between two different generation
    /// kinds, and you may wish to easily convert generation numbers from one
    /// kind to another.
    ///
    /// Typically, a conversion from `TypedGeneration<T>` to `TypedGeneration<U>`
    /// is most useful when `T`'s semantics are a superset of `U`'s, or in other
    /// words, when every `TypedGeneration<T>` is logically also a
    /// `TypedGeneration<U>`.
    ///
    /// For instance:
    ///
    /// * Imagine you have [`TypedGenerationKind`]s for different types of
    ///   database connections, where `DbConnKind` is the general type
    ///   and `PgConnKind` is a specific kind for Postgres.
    /// * Since every Postgres connection is also a database connection,
    ///   a cast from `TypedGeneration<PgConnKind>` to `TypedGeneration<DbConnKind>`
    ///   makes sense.
    /// * The inverse cast would not make sense, as a database connection may not
    ///   necessarily be a Postgres connection.
    ///
    /// This interface provides an alternative, safer way to perform this
    /// conversion. Indicate your intention to allow a conversion between kinds
    /// by implementing `From<T> for U`, as shown in the example below.
    ///
    /// # Examples
    ///
    /// ```
    /// use oxide_generation::{TypedGeneration, TypedGenerationKind, TypedGenerationTag};
    ///
    /// // Let's say that these generation numbers track repositories for
    /// // different version control systems, such that you have a generic
    /// // RepoKind:
    /// pub enum RepoKind {}
    /// impl TypedGenerationKind for RepoKind {
    ///     const TAG: TypedGenerationTag = TypedGenerationTag::new("repo");
    /// }
    ///
    /// // You also have more specific kinds:
    /// pub enum GitRepoKind {}
    /// impl TypedGenerationKind for GitRepoKind {
    ///     const TAG: TypedGenerationTag = TypedGenerationTag::new("git_repo");
    /// }
    /// // (and HgRepoKind, JujutsuRepoKind, etc...)
    ///
    /// // First, define a `From` impl. This impl indicates your desire
    /// // to convert from one kind to another.
    /// impl From<GitRepoKind> for RepoKind {
    ///     fn from(value: GitRepoKind) -> Self {
    ///         match value {}
    ///     }
    /// }
    ///
    /// // Now you can convert between them:
    /// let git_generation: TypedGeneration<GitRepoKind> = TypedGeneration::from_u32(5);
    /// let repo_generation: TypedGeneration<RepoKind> = git_generation.upcast();
    /// ```
    #[inline]
    #[must_use]
    pub const fn upcast<U: TypedGenerationKind>(self) -> TypedGeneration<U>
    where
        T: Into<U>,
    {
        TypedGeneration {
            generation: self.generation,
            _phantom: PhantomData,
        }
    }
}

// ---
// Trait impls
// ---

impl<T: TypedGenerationKind> PartialEq for TypedGeneration<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.generation.eq(&other.generation)
    }
}

impl<T: TypedGenerationKind> Eq for TypedGeneration<T> {}

impl<T: TypedGenerationKind> PartialOrd for TypedGeneration<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: TypedGenerationKind> Ord for TypedGeneration<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.generation.cmp(&other.generation)
    }
}

impl<T: TypedGenerationKind> Hash for TypedGeneration<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.generation.hash(state);
    }
}

impl<T: TypedGenerationKind> fmt::Debug for TypedGeneration<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.generation, T::TAG)
    }
}

impl<T: TypedGenerationKind> fmt::Display for TypedGeneration<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.generation.fmt(f)
    }
}

impl<T: TypedGenerationKind> Clone for TypedGeneration<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: TypedGenerationKind> Copy for TypedGeneration<T> {}

impl<T: TypedGenerationKind> FromStr for TypedGeneration<T> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let generation =
            Generation::from_str(s).map_err(|error| ParseError { error, tag: T::TAG })?;
        Ok(Self::from_untyped_generation(generation))
    }
}

impl<T: TypedGenerationKind> From<u32> for TypedGeneration<T> {
    #[inline]
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl<T: TypedGenerationKind> From<TypedGeneration<T>> for u64 {
    #[inline]
    fn from(g: TypedGeneration<T>) -> Self {
        g.as_u64()
    }
}

impl<T: TypedGenerationKind> From<TypedGeneration<T>> for i64 {
    #[inline]
    fn from(g: TypedGeneration<T>) -> Self {
        g.as_i64()
    }
}

impl<T: TypedGenerationKind> From<&TypedGeneration<T>> for i64 {
    #[inline]
    fn from(g: &TypedGeneration<T>) -> Self {
        g.as_i64()
    }
}

impl<T: TypedGenerationKind> TryFrom<u64> for TypedGeneration<T> {
    type Error = GenerationOverflowError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self::from_untyped_generation(Generation::try_from(value)?))
    }
}

impl<T: TypedGenerationKind> TryFrom<i64> for TypedGeneration<T> {
    type Error = GenerationNegativeError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self::from_untyped_generation(Generation::try_from(value)?))
    }
}

#[cfg(feature = "serde")]
mod serde_imp {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<'de> Deserialize<'de> for Generation {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = u64::deserialize(deserializer)?;
            Generation::try_from(value).map_err(|GenerationOverflowError(_)| {
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Unsigned(value),
                    &"an integer between 0 and 9223372036854775807",
                )
            })
        }
    }

    // This is the equivalent of applying `#[serde(transparent)]`, written out by
    // hand to match the manual `Deserialize` impl above.
    impl Serialize for Generation {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    /// Deserializes a `TypedGeneration<T>` using the same format as [`Generation`].
    ///
    /// This impl does not require `T` to implement `Deserialize`.
    impl<'de, T: TypedGenerationKind> Deserialize<'de> for TypedGeneration<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(Self::from_untyped_generation(Generation::deserialize(
                deserializer,
            )?))
        }
    }

    /// Serializes a `TypedGeneration<T>` using the same format as [`Generation`].
    ///
    /// This impl does not require `T` to implement `Serialize`.
    impl<T: TypedGenerationKind> Serialize for TypedGeneration<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.generation.serialize(serializer)
        }
    }
}

#[cfg(feature = "schemars08")]
mod schemars08_imp {
    use super::*;
    use schemars::{
        JsonSchema, SchemaGenerator,
        schema::{InstanceType, Metadata, NumberValidation, Schema, SchemaObject},
        schema_for,
    };

    // This must be the first line of the doc comment on `Generation`.
    const GENERATION_DESCRIPTION: &str =
        "Generation numbers stored in the database, used for optimistic concurrency control";

    const CRATE_NAME: &str = "oxide-generation";
    // 0.x crates are semver-compatible only within a minor version, so this is
    // "0.1" rather than "0".
    const CRATE_VERSION: &str = "0.1";
    const CRATE_PATH: &str = "oxide_generation::TypedGeneration";

    /// Implements `JsonSchema` for `Generation`.
    ///
    /// The schema is that of a `u64` with a minimum of 0; the upper bound is not expressed in the
    /// schema. The description matches the first line of the doc comment on [`Generation`].
    impl JsonSchema for Generation {
        #[inline]
        fn schema_name() -> String {
            "Generation".to_owned()
        }

        #[inline]
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("oxide_generation::Generation")
        }

        #[inline]
        fn json_schema(_: &mut SchemaGenerator) -> Schema {
            SchemaObject {
                metadata: Some(Box::new(Metadata {
                    description: Some(GENERATION_DESCRIPTION.to_owned()),
                    ..Default::default()
                })),
                ..generation_schema_object()
            }
            .into()
        }
    }

    /// Implements `JsonSchema` for `TypedGeneration<T>`, if `T` implements `JsonSchema`.
    ///
    /// * `schema_name` is set to `"TypedGenerationFor"`, concatenated by the schema name of `T`.
    /// * `schema_id` is set to `format!("oxide_generation::TypedGeneration<{}>",
    ///   T::schema_id())`.
    /// * `json_schema` is the same as the one for `Generation`, with the `x-rust-type` extension
    ///   to allow automatic replacement in typify and progenitor.
    impl<T> JsonSchema for TypedGeneration<T>
    where
        T: TypedGenerationKind + JsonSchema,
    {
        #[inline]
        fn schema_name() -> String {
            // Use the alias if available, otherwise generate our own schema name.
            if let Some(alias) = T::ALIAS {
                alias.to_owned()
            } else {
                format!("TypedGenerationFor{}", T::schema_name())
            }
        }

        #[inline]
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Owned(format!(
                "oxide_generation::TypedGeneration<{}>",
                T::schema_id()
            ))
        }

        #[inline]
        fn json_schema(generator: &mut SchemaGenerator) -> Schema {
            // Look at the schema for `T`. If it has `x-rust-type`, *and* if an
            // alias is available, we can lift up the `x-rust-type` into our own schema.
            //
            // We use a new schema generator for `T` to avoid T's schema being
            // added to the list of schemas in `generator` in case the lifting
            // is successful.
            let t_schema = schema_for!(T);
            if let Some(schema) = lift_json_schema(&t_schema.schema, T::ALIAS) {
                return schema.into();
            }

            SchemaObject {
                extensions: [(
                    "x-rust-type".to_string(),
                    serde_json::json!({
                        "crate": CRATE_NAME,
                        "version": CRATE_VERSION,
                        "path": CRATE_PATH,
                        "parameters": [generator.subschema_for::<T>()]
                    }),
                )]
                .into_iter()
                .collect(),
                ..generation_schema_object()
            }
            .into()
        }
    }

    // The schema shared by `Generation` and `TypedGeneration<T>`: a `u64` with a
    // minimum of 0.
    fn generation_schema_object() -> SchemaObject {
        SchemaObject {
            instance_type: Some(InstanceType::Integer.into()),
            format: Some("uint64".to_string()),
            number: Some(Box::new(NumberValidation {
                minimum: Some(0.0),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    // ? on Option is too easy to make mistakes with, so we use `let Some(..) =
    // .. else` instead.
    #[allow(clippy::question_mark)]
    fn lift_json_schema(schema: &SchemaObject, alias: Option<&str>) -> Option<SchemaObject> {
        let Some(alias) = alias else {
            return None;
        };

        let Some(v) = schema.extensions.get("x-rust-type") else {
            return None;
        };

        // The crate, version and path must all be present.
        let Some(crate_) = v.get("crate") else {
            return None;
        };
        let Some(version) = v.get("version") else {
            return None;
        };
        let Some(path) = v.get("path").and_then(|p| p.as_str()) else {
            return None;
        };
        let Some((module_path, _)) = path.rsplit_once("::") else {
            return None;
        };

        // The preconditions are all met. We can lift the schema by appending
        // the alias to the module path.
        let alias_path = format!("{module_path}::{alias}");

        Some(SchemaObject {
            extensions: [(
                "x-rust-type".to_string(),
                serde_json::json!({
                    "crate": crate_,
                    "version": version,
                    "path": alias_path,
                }),
            )]
            .into_iter()
            .collect(),
            ..generation_schema_object()
        })
    }
}

#[cfg(feature = "proptest1")]
mod proptest1_imp {
    use super::*;
    use proptest::{
        arbitrary::Arbitrary,
        strategy::{BoxedStrategy, Strategy},
    };

    /// Parameters for use with `proptest` instances.
    ///
    /// This is currently not exported as a type because it has no options. But
    /// it's left in as an extension point for the future.
    #[derive(Clone, Debug, Default)]
    pub struct GenerationParams(());

    /// Generates random `Generation` instances.
    ///
    /// Values are drawn uniformly from the range `0..=`[`Generation::MAX`].
    impl Arbitrary for Generation {
        type Parameters = GenerationParams;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (0..=Generation::MAX.as_u64()).prop_map(Generation).boxed()
        }
    }

    /// Parameters for use with `proptest` instances.
    ///
    /// This is currently not exported as a type because it has no options. But
    /// it's left in as an extension point for the future.
    #[derive(Clone, Debug, Default)]
    pub struct TypedGenerationParams(());

    /// Generates random `TypedGeneration<T>` instances.
    ///
    /// Values are drawn uniformly from the range `0..=`[`TypedGeneration::MAX`].
    impl<T> Arbitrary for TypedGeneration<T>
    where
        T: TypedGenerationKind,
    {
        type Parameters = TypedGenerationParams;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (0..=Generation::MAX.as_u64())
                .prop_map(|value| TypedGeneration::<T>::from_untyped_generation(Generation(value)))
                .boxed()
        }
    }
}

#[cfg(feature = "daft01")]
mod daft01_imp {
    use super::*;

    /// Diffs a `Generation` as a leaf value.
    ///
    /// Generation numbers are scalars, so diffing stops at them.
    impl daft::Diffable for Generation {
        type Diff<'daft> = daft::Leaf<&'daft Self>;

        fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
            daft::Leaf {
                before: self,
                after: other,
            }
        }
    }

    /// Diffs a `TypedGeneration<T>` as a leaf value.
    ///
    /// This impl does not require `T` to implement `Diffable`.
    impl<T: TypedGenerationKind> daft::Diffable for TypedGeneration<T> {
        type Diff<'daft> = daft::Leaf<&'daft Self>;

        fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
            daft::Leaf {
                before: self,
                after: other,
            }
        }
    }
}

#[cfg(feature = "slog2")]
mod slog2_imp {
    use super::*;

    /// Logs a `Generation` as an integer.
    impl slog::Value for Generation {
        fn serialize(
            &self,
            _rec: &slog::Record,
            key: slog::Key,
            serializer: &mut dyn slog::Serializer,
        ) -> slog::Result {
            serializer.emit_u64(key, self.as_u64())
        }
    }

    /// Logs a `TypedGeneration<T>` as an integer, using the same format as [`Generation`].
    ///
    /// This impl does not require `T` to implement `slog::Value`.
    impl<T: TypedGenerationKind> slog::Value for TypedGeneration<T> {
        fn serialize(
            &self,
            _rec: &slog::Record,
            key: slog::Key,
            serializer: &mut dyn slog::Serializer,
        ) -> slog::Result {
            serializer.emit_u64(key, self.as_u64())
        }
    }
}

/// Represents marker types that can be used as a type parameter for [`TypedGeneration`].
///
/// Generally, an implementation of this will be a zero-sized type that can never be constructed. An
/// empty struct or enum works well for this.
///
/// # Implementations
///
/// If the `schemars08` feature is enabled, and [`JsonSchema`] is implemented for a kind `T`, then
/// [`TypedGeneration`]`<T>` will also implement [`JsonSchema`].
///
/// If you have a large number of generation kinds, consider using
/// [`oxide-generation-macros`] which comes with several convenience features.
///
/// ```
/// use oxide_generation_macros::impl_typed_generation_kinds;
///
/// // Invoke this macro with:
/// impl_typed_generation_kinds! {
///     kinds = {
///         User = {},
///         Project = {},
///         // ...
///     },
/// }
/// ```
///
/// See [`oxide-generation-macros`] for more information.
///
/// [`oxide-generation-macros`]: https://docs.rs/oxide-generation-macros
/// [`JsonSchema`]: schemars::JsonSchema
pub trait TypedGenerationKind: Send + Sync + 'static {
    /// The corresponding tag for this kind.
    ///
    /// The tag forms a runtime representation of this type.
    ///
    /// The tag is required to be a static string.
    const TAG: TypedGenerationTag;

    /// A string that corresponds to a type alias for `TypedGeneration<Self>`,
    /// if one is defined.
    ///
    /// The type alias must be defined in the same module as `Self`. This
    /// constant is used by the schemars integration to refer to embed a
    /// reference to that alias in the schema, if available.
    ///
    /// This is usually defined by the [`oxide-generation-macros`] crate.
    ///
    /// [`oxide-generation-macros`]: https://docs.rs/oxide-generation-macros
    const ALIAS: Option<&'static str> = None;
}

/// Describes what kind of [`TypedGeneration`] something is.
///
/// This is the runtime equivalent of [`TypedGenerationKind`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedGenerationTag(&'static str);

impl TypedGenerationTag {
    /// Creates a new `TypedGenerationTag` from a static string.
    ///
    /// The string must be non-empty, and consist of:
    /// - ASCII letters
    /// - digits (only after the first character)
    /// - underscores
    /// - hyphens (only after the first character)
    ///
    /// # Panics
    ///
    /// Panics if the above conditions aren't met. Use [`Self::try_new`] to handle errors instead.
    #[must_use]
    pub const fn new(tag: &'static str) -> Self {
        match Self::try_new_impl(tag) {
            Ok(tag) => tag,
            Err(message) => panic!("{}", message),
        }
    }

    /// Attempts to create a new `TypedGenerationTag` from a static string.
    ///
    /// The string must be non-empty, and consist of:
    /// - ASCII letters
    /// - digits (only after the first character)
    /// - underscores
    /// - hyphens (only after the first character)
    ///
    /// # Errors
    ///
    /// Returns a [`TagError`] if the above conditions aren't met.
    pub const fn try_new(tag: &'static str) -> Result<Self, TagError> {
        match Self::try_new_impl(tag) {
            Ok(tag) => Ok(tag),
            Err(message) => Err(TagError {
                input: tag,
                message,
            }),
        }
    }

    const fn try_new_impl(tag: &'static str) -> Result<Self, &'static str> {
        if tag.is_empty() {
            return Err("tag must not be empty");
        }

        let bytes = tag.as_bytes();
        if !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_') {
            return Err("first character of tag must be an ASCII letter or underscore");
        }

        let mut bytes = match bytes {
            [_, rest @ ..] => rest,
            [] => panic!("already checked that it's non-empty"),
        };
        while let [rest @ .., last] = &bytes {
            if !(last.is_ascii_alphanumeric() || *last == b'_' || *last == b'-') {
                break;
            }
            bytes = rest;
        }

        if !bytes.is_empty() {
            return Err("tag must only contain ASCII letters, digits, underscores, or hyphens");
        }

        Ok(Self(tag))
    }

    /// Returns the tag as a string.
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl fmt::Display for TypedGenerationTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl AsRef<str> for TypedGenerationTag {
    fn as_ref(&self) -> &str {
        self.0
    }
}

/// An error that occurred while creating a [`TypedGenerationTag`].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct TagError {
    /// The input string.
    pub input: &'static str,

    /// The error message.
    pub message: &'static str,
}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error creating tag from '{}': {}",
            self.input, self.message
        )
    }
}

impl core::error::Error for TagError {}

/// An error that occurred while parsing a [`TypedGeneration`].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ParseError {
    /// The underlying error.
    pub error: ParseIntError,

    /// The tag of the generation number that failed to parse.
    pub tag: TypedGenerationTag,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error parsing generation number ({})", self.tag)
    }
}

impl core::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// A trait abstracting over typed and untyped generation numbers.
///
/// This can be used to write code that's generic over [`TypedGeneration`], [`Generation`], and
/// other types that may wrap [`TypedGeneration`] (due to e.g. orphan rules).
///
/// This trait is similar to `From`, but a bit harder to get wrong -- in general, the conversion
/// from and to untyped generation numbers should be careful and explicit.
pub trait GenericGeneration {
    /// Creates a new instance of `Self` from an untyped [`Generation`].
    #[must_use]
    fn from_untyped_generation(generation: Generation) -> Self
    where
        Self: Sized;

    /// Converts `self` into an untyped [`Generation`].
    #[must_use]
    fn into_untyped_generation(self) -> Generation
    where
        Self: Sized;

    /// Returns the inner [`Generation`].
    ///
    /// Generally, [`into_untyped_generation`](Self::into_untyped_generation) should be preferred.
    /// However, in some cases it may be necessary to use this method to satisfy lifetime
    /// constraints.
    fn as_untyped_generation(&self) -> &Generation;
}

impl GenericGeneration for Generation {
    #[inline]
    fn from_untyped_generation(generation: Generation) -> Self {
        generation
    }

    #[inline]
    fn into_untyped_generation(self) -> Generation {
        self
    }

    #[inline]
    fn as_untyped_generation(&self) -> &Generation {
        self
    }
}

impl<T: TypedGenerationKind> GenericGeneration for TypedGeneration<T> {
    #[inline]
    fn from_untyped_generation(generation: Generation) -> Self {
        Self {
            generation,
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn into_untyped_generation(self) -> Generation {
        self.generation
    }

    #[inline]
    fn as_untyped_generation(&self) -> &Generation {
        &self.generation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum MyKind {}

    impl TypedGenerationKind for MyKind {
        const TAG: TypedGenerationTag = TypedGenerationTag::new("my_kind");
    }

    #[test]
    fn test_validate_tags() {
        for &valid_tag in &[
            "a", "a-", "a_", "a-b", "a_b", "a1", "a1-", "a1_", "a1-b", "a1_b", "_a",
        ] {
            TypedGenerationTag::try_new(valid_tag).expect("tag is valid");
            // Should not panic
            _ = TypedGenerationTag::new(valid_tag);
        }

        for invalid_tag in &["", "1", "-", "a1b!", "a1-b!", "a1_b:", "\u{1f4a9}"] {
            TypedGenerationTag::try_new(invalid_tag).unwrap_err();
        }
    }

    // This test just ensures that `GenericGeneration` is object-safe.
    #[test]
    #[cfg(feature = "std")]
    fn test_generic_generation_object_safe() {
        let generation = Generation::new();
        let box_generation = Box::new(generation) as Box<dyn GenericGeneration>;
        assert_eq!(box_generation.as_untyped_generation(), &generation);
    }

    #[test]
    fn test_next_and_prev() {
        // The first generation is 1, and it has no predecessor.
        let first = Generation::new();
        assert_eq!(first.as_u64(), 1);
        assert_eq!(first.prev(), None);
        assert_eq!(first.next(), Generation::from_u32(2));
        assert_eq!(first.checked_next(), Some(Generation::from_u32(2)));

        // Generation 0 also has no predecessor.
        let zero = Generation::ZERO;
        assert_eq!(zero.as_u64(), 0);
        assert_eq!(zero.prev(), None);
        assert_eq!(zero.next(), first);

        // One below the maximum still has a successor.
        let almost_max = Generation::try_from(Generation::MAX.as_u64() - 1).unwrap();
        assert_eq!(almost_max.next(), Generation::MAX);
        assert_eq!(almost_max.checked_next(), Some(Generation::MAX));
        assert_eq!(almost_max.prev().unwrap().as_u64(), i64::MAX as u64 - 2);

        // The maximum has no successor.
        assert_eq!(Generation::MAX.checked_next(), None);
        assert_eq!(Generation::MAX.prev(), Some(almost_max));

        // The same holds for typed generation numbers.
        let typed_max = TypedGeneration::<MyKind>::MAX;
        assert_eq!(typed_max.as_u64(), Generation::MAX.as_u64());
        assert_eq!(typed_max.checked_next(), None);
        assert_eq!(
            typed_max.prev().map(|g| g.as_u64()),
            Some(Generation::MAX.as_u64() - 1)
        );
        assert_eq!(TypedGeneration::<MyKind>::new().as_u64(), 1);
        assert_eq!(TypedGeneration::<MyKind>::ZERO.as_u64(), 0);
        assert_eq!(TypedGeneration::<MyKind>::ZERO.next().as_u64(), 1);
    }

    #[test]
    #[should_panic(expected = "attempt to overflow generation number")]
    fn test_next_at_max_panics() {
        _ = Generation::MAX.next();
    }

    #[test]
    #[should_panic(expected = "attempt to overflow generation number")]
    fn test_typed_next_at_max_panics() {
        _ = TypedGeneration::<MyKind>::MAX.next();
    }

    #[test]
    fn test_try_from() {
        // Zero and the maximum are accepted, but one past the maximum is not.
        assert_eq!(Generation::try_from(0_u64).unwrap(), Generation::ZERO);
        assert_eq!(
            Generation::try_from(i64::MAX as u64).unwrap(),
            Generation::MAX
        );
        Generation::try_from(i64::MAX as u64 + 1).unwrap_err();
        Generation::try_from(u64::MAX).unwrap_err();

        // Non-negative i64s are accepted, negative ones are not.
        assert_eq!(Generation::try_from(0_i64).unwrap().as_u64(), 0);
        assert_eq!(Generation::try_from(i64::MAX).unwrap(), Generation::MAX);
        Generation::try_from(-1_i64).unwrap_err();
        Generation::try_from(i64::MIN).unwrap_err();

        // The same holds for typed generation numbers.
        assert_eq!(
            TypedGeneration::<MyKind>::try_from(i64::MAX as u64).unwrap(),
            TypedGeneration::<MyKind>::MAX
        );
        TypedGeneration::<MyKind>::try_from(i64::MAX as u64 + 1).unwrap_err();
        TypedGeneration::<MyKind>::try_from(-1_i64).unwrap_err();
    }

    #[test]
    fn test_conversions() {
        let generation = Generation::from_u32(5);
        assert_eq!(u64::from(generation), 5);
        assert_eq!(i64::from(generation), 5);
        assert_eq!(i64::from(&generation), 5);
        assert_eq!(Generation::from(5_u32), generation);

        let typed = TypedGeneration::<MyKind>::from_u32(5);
        assert_eq!(u64::from(typed), 5);
        assert_eq!(i64::from(typed), 5);
        assert_eq!(i64::from(&typed), 5);
        assert_eq!(TypedGeneration::<MyKind>::from(5_u32), typed);
        assert_eq!(typed.into_untyped_generation(), generation);
        assert_eq!(typed.as_untyped_generation(), &generation);
        assert_eq!(typed.as_i64(), 5);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Generation::from_str("0").unwrap().as_u64(), 0);
        assert_eq!(Generation::from_str("1").unwrap(), Generation::new());
        assert_eq!(
            Generation::from_str("9223372036854775807").unwrap(),
            Generation::MAX
        );

        // Negative numbers and numbers greater than the maximum are both
        // rejected.
        Generation::from_str("-1").unwrap_err();
        Generation::from_str("9223372036854775808").unwrap_err();

        // The same holds for typed generation numbers.
        assert_eq!(
            "9223372036854775807"
                .parse::<TypedGeneration<MyKind>>()
                .unwrap(),
            TypedGeneration::<MyKind>::MAX
        );
        "-1".parse::<TypedGeneration<MyKind>>().unwrap_err();
        "9223372036854775808"
            .parse::<TypedGeneration<MyKind>>()
            .unwrap_err();
    }

    // The remaining tests format values, which requires an allocator.

    #[test]
    #[cfg(feature = "std")]
    fn test_display_and_debug() {
        assert_eq!(Generation::new().to_string(), "1");

        let generation = Generation::from_u32(5);
        assert_eq!(generation.to_string(), "5");

        let typed = TypedGeneration::<MyKind>::from_u32(5);
        assert_eq!(typed.to_string(), "5");
        assert_eq!(format!("{typed:?}"), "5 (my_kind)");
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_error_displays() {
        assert_eq!(
            Generation::try_from(i64::MAX as u64 + 1)
                .unwrap_err()
                .to_string(),
            "generation number too large"
        );
        assert_eq!(
            Generation::try_from(-1_i64).unwrap_err().to_string(),
            "negative generation number"
        );
        assert_eq!(
            "-1".parse::<TypedGeneration<MyKind>>()
                .unwrap_err()
                .to_string(),
            "error parsing generation number (my_kind)"
        );
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "std"))]
    fn test_serde() {
        let generation = Generation::from_u32(5);
        assert_eq!(serde_json::to_string(&generation).unwrap(), "5");
        assert_eq!(serde_json::from_str::<Generation>("5").unwrap(), generation);

        // The first generation round-trips as the bare integer 1.
        assert_eq!(serde_json::to_string(&Generation::new()).unwrap(), "1");
        assert_eq!(
            serde_json::from_str::<Generation>("1").unwrap(),
            Generation::new()
        );

        // Both ends of the valid range deserialize, and values outside it --
        // whether above the maximum or negative -- do not.
        assert_eq!(
            serde_json::from_str::<Generation>("0").unwrap(),
            Generation::ZERO
        );
        assert_eq!(
            serde_json::from_str::<Generation>(&Generation::MAX.as_u64().to_string()).unwrap(),
            Generation::MAX
        );
        for bad_value in [Generation::MAX.as_u64() + 1, u64::MAX] {
            serde_json::from_str::<Generation>(&bad_value.to_string()).unwrap_err();
        }
        for bad_value in [-1_i64, i64::MIN] {
            serde_json::from_str::<Generation>(&bad_value.to_string()).unwrap_err();
        }

        let typed = TypedGeneration::<MyKind>::from_u32(5);
        assert_eq!(serde_json::to_string(&typed).unwrap(), "5");
        assert_eq!(
            serde_json::from_str::<TypedGeneration<MyKind>>("5").unwrap(),
            typed
        );

        // One past the maximum is rejected with a message that describes the
        // valid range.
        let error = serde_json::from_str::<Generation>("9223372036854775808").unwrap_err();
        assert_eq!(
            error.to_string(),
            "invalid value: integer `9223372036854775808`, \
             expected an integer between 0 and 9223372036854775807"
        );
        serde_json::from_str::<TypedGeneration<MyKind>>("9223372036854775808").unwrap_err();
    }

    #[test]
    #[cfg(feature = "daft01")]
    fn test_daft() {
        use daft::Diffable;

        let before = Generation::from_u32(5);
        let after = Generation::from_u32(6);
        let diff = before.diff(&after);
        assert_eq!(diff.before, &before);
        assert_eq!(diff.after, &after);

        // The same holds for typed generation numbers.
        let typed_before = TypedGeneration::<MyKind>::from_u32(5);
        let typed_after = TypedGeneration::<MyKind>::from_u32(6);
        let typed_diff = typed_before.diff(&typed_after);
        assert_eq!(typed_diff.before, &typed_before);
        assert_eq!(typed_diff.after, &typed_after);
    }

    #[test]
    #[cfg(feature = "slog2")]
    fn test_slog() {
        // Records the value passed to `emit_u64`, so that the test can check
        // that generation numbers are logged as integers rather than strings.
        struct RecordingSerializer(Option<u64>);

        impl slog::Serializer for RecordingSerializer {
            fn emit_u64(&mut self, _key: slog::Key, value: u64) -> slog::Result {
                self.0 = Some(value);
                Ok(())
            }

            fn emit_arguments(&mut self, _key: slog::Key, _value: &fmt::Arguments) -> slog::Result {
                panic!("generation numbers are emitted via emit_u64")
            }
        }

        // The record is built inline because `format_args!` produces a
        // temporary that only lives until the end of the statement.
        //
        // slog's `Key` is a plain `&'static str` unless the `dynamic-keys`
        // feature is enabled, so the conversion below is sometimes a no-op.
        #[allow(clippy::useless_conversion)]
        fn emitted_u64(value: &dyn slog::Value) -> Option<u64> {
            let mut serializer = RecordingSerializer(None);
            value
                .serialize(
                    &slog::record!(
                        slog::Level::Info,
                        "",
                        &format_args!(""),
                        slog::BorrowedKV(&())
                    ),
                    "generation".into(),
                    &mut serializer,
                )
                .expect("generation number was serialized");
            serializer.0
        }

        assert_eq!(emitted_u64(&Generation::from_u32(5)), Some(5));

        // The same holds for typed generation numbers.
        assert_eq!(
            emitted_u64(&TypedGeneration::<MyKind>::from_u32(6)),
            Some(6)
        );
    }

    #[test]
    #[cfg(feature = "schemars08")]
    fn test_generation_schema() {
        // This schema must match the one omicron generates today, so that wire
        // schemas are unchanged.
        let schema = <Generation as schemars::JsonSchema>::json_schema(
            &mut schemars::SchemaGenerator::default(),
        );
        assert_eq!(
            serde_json::to_value(&schema).unwrap(),
            serde_json::json!({
                "description": "Generation numbers stored in the database, \
                                used for optimistic concurrency control",
                "type": "integer",
                "format": "uint64",
                "minimum": 0.0,
            })
        );
    }
}
