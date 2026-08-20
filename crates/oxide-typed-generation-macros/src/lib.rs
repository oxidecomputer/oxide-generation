//! Procedural macro for [`oxide-typed-generation`](https://docs.rs/oxide-typed-generation).
//!
//! This crate provides a procedural macro to help with creating
//! [`oxide-typed-generation`](https://docs.rs/oxide-typed-generation) instances.
//!
//! For more information, see the documentation for [`impl_typed_generation_kinds!`].
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! use oxide_typed_generation::TypedGenerationKind;
//! use oxide_typed_generation_macros::impl_typed_generation_kinds;
//!
//! impl_typed_generation_kinds! {
//!     kinds = {
//!         User = {},
//!         BusinessUnit = {},
//!     },
//! }
//!
//! // This generates empty UserKind and BusinessUnitKind enums implementing
//! // TypedGenerationKind, with the tags "user" and "business_unit" respectively.
//! // Tags are snake_case versions of type names.
//! assert_eq!(UserKind::TAG.as_str(), "user");
//! assert_eq!(BusinessUnitKind::TAG.as_str(), "business_unit");
//!
//! // The macro also generates UserGeneration and BusinessUnitGeneration type aliases.
//! let user_generation = UserGeneration::new();
//! let business_unit_generation = BusinessUnitGeneration::new();
//! ```
//!
//! For more details and examples, see the documentation for
//! [`impl_typed_generation_kinds!`].

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod internals;

use proc_macro::TokenStream;
use quote::ToTokens;

/// A function-like procedural macro for implementing typed generation kinds.
///
/// This macro generates types that implement `TypedGenerationKind` and corresponding
/// type aliases for `TypedGeneration<T>`. The macro provides an easy way to generate
/// typed generation kinds in bulk, and also to implement `JsonSchema` support with
/// schemars 0.8.
///
/// # Basic usage
///
/// Invoke the `impl_typed_generation_kinds!` macro within a path that's publicly
/// visible. A dedicated crate for generation kinds is recommended.
///
/// By default, for a kind `Foo`, this macro generates:
///
/// * A `FooKind` type that implements `TypedGenerationKind`: `pub enum FooKind {}`.
/// * A `FooGeneration` type alias: `pub type FooGeneration = TypedGeneration<FooKind>;`.
///
/// ## Examples
///
/// ```
/// use oxide_typed_generation::TypedGenerationKind;
/// use oxide_typed_generation_macros::impl_typed_generation_kinds;
///
/// impl_typed_generation_kinds! {
///     kinds = {
///         User = {},
///         BusinessUnit = {},
///     },
/// }
///
/// // This generates empty UserKind and BusinessUnitKind enums implementing
/// // TypedGenerationKind, with the tags "user" and "business_unit" respectively.
/// // Tags are snake_case versions of type names.
/// assert_eq!(UserKind::TAG.as_str(), "user");
/// assert_eq!(BusinessUnitKind::TAG.as_str(), "business_unit");
///
/// // The macro also generates UserGeneration and BusinessUnitGeneration type aliases.
/// let user_generation = UserGeneration::new();
/// let business_unit_generation = BusinessUnitGeneration::new();
/// ```
///
/// * The generated `Kind` types always implement `Clone`, `Copy`, `Debug`,
///   `Eq`, and `PartialEq`.
/// * The `Kind` types are all empty enums, also known as *marker* or
///   *uninhabited* enums. This means that values for these types cannot be
///   created. (Using empty enums is the recommended approach for
///   `oxide-typed-generation`).
///
/// # Per-kind settings
///
/// Kinds can be customized with the following settings:
///
/// - `attrs`: Attributes to apply to the kind enum, such as
///   `#[derive(SomeTrait)]` or `#[cfg(feature = "some-feature")]`. *Optional,
///   defaults to the global `attrs`.*
/// - `tag`: The tag to use for the kind (a string literal). *Optional, defaults
///   to the snake_case version of the type name.*
/// - `type_name`: The name of the type to use for the kind (a Rust identifier).
///   *Optional, defaults to `{Name}Kind`*.
/// - `alias`: The name of the type alias to use for the kind (a Rust
///   identifier). *Optional, defaults to `{Name}Generation`*.
///
/// Per-kind customizations should generally be unnecessary; the conventionally
/// generated type names should be sufficient for most use cases.
///
/// ## Examples
///
/// In this example, we derive `PartialOrd` and `Ord` for `MyUserKind`.
///
/// ```
/// use oxide_typed_generation::TypedGenerationKind;
/// use oxide_typed_generation_macros::impl_typed_generation_kinds;
///
/// impl_typed_generation_kinds! {
///     kinds = {
///         User = {
///             // This is a list of attributes surrounded by square brackets.
///             attrs = [#[derive(PartialOrd, Ord)]],
///             tag = "user",
///             type_name = MyUserKind,
///         },
///         Organization = { tag = "org", alias = OrgGeneration },
///     },
/// }
///
/// // This generates types with the specified names:
/// assert_eq!(MyUserKind::TAG.as_str(), "user");
/// assert_eq!(OrganizationKind::TAG.as_str(), "org");
///
/// let user_generation = UserGeneration::new();
/// let org_generation = OrgGeneration::new();
///
/// // MyUserKind also implements `Ord`.
/// static_assertions::assert_impl_all!(MyUserKind: Ord);
/// ```
///
/// # Global settings
///
/// This macro accepts global settings under a top-level `settings` map:
///
/// - `attrs`: A list of attributes to apply to all generated `Kind` types.
///   Per-kind attributes, if provided, will override these. *Optional, defaults
///   to the empty list.*
/// - `oxide_typed_generation_crate`: The name of the `oxide-typed-generation` crate (a Rust
///   identifier). *Optional, defaults to `oxide_typed_generation`.*
/// - `schemars08`: If defined, generates JSON Schema support for the given
///   types using [`schemars` 0.8]. *Optional.*
///
/// ## JSON Schema support
///
/// If the `schemars08` global setting is defined, the macro generates JSON
/// Schema support for the `Kind` instances using [schemars 0.8].
///
/// **To enable JSON Schema support, you'll need to enable `oxide-typed-generation`'s
/// `schemars08` feature.**
///
/// Within `settings.schemars08`, the options are:
///
/// - `attrs`: A list of attributes to apply to all generated `JsonSchema`
///   implementations. For example, if `schemars` is an optional dependency
///   for the crate where the macro is being invoked, you might specify something
///   like `[#[cfg(feature = "schemars")]]`.
/// - `rust_type`: If defined, adds the `x-rust-type` extension to the schema,
///   enabling automatic replacement with [`typify`] and other tools that
///   support it. *Optional, defaults to not adding the extension.*
///
///   Automatic replacement enables an end-to-end workflow where the same generation
///   kinds can be shared between servers and clients.
///
///   `rust_type` is a map of the following options:
///
///   - `crate`: The crate name consumers will use to access these types.
///     *Required.*
///   - `version`: The versions of the crate that automatic replacement is
///     supported for. *Required.*
///   - `path`: The path to the module these types can be accessed from,
///     including the crate name. *Required.*
///
///   For more about `x-rust-type`, see the [`typify` documentation].
///
/// [`schemars` 0.8]: https://docs.rs/schemars/0.8/schemars/
/// [`typify`]: https://docs.rs/typify
/// [`typify` documentation]:
///     https://github.com/oxidecomputer/typify#rust---schema---rust
///
/// ## Examples
///
/// An example with all global settings defined:
///
/// ```
/// use oxide_typed_generation::TypedGenerationKind;
/// use oxide_typed_generation_macros::impl_typed_generation_kinds;
///
/// impl_typed_generation_kinds! {
///     settings = {
///         attrs = [#[derive(PartialOrd, Ord)]],
///         oxide_typed_generation_crate = oxide_typed_generation,
///         schemars08 = {
///             attrs = [#[cfg(feature = "schemars")]],
///             rust_type = {
///                 crate = "my-crate",
///                 version = "0.1.0",
///                 path = "my_crate::types",
///             },
///         },
///     },
///     kinds = {
///         User = {},
///         Organization = {},
///         Project = {},
///     },
/// }
///
/// let user_generation = UserGeneration::new();
/// let org_generation = OrganizationGeneration::new();
/// let project_generation = ProjectGeneration::new();
/// ```
///
/// For a working end-to-end example, see the
/// [`e2e-example`](https://github.com/oxidecomputer/oxide-typed-generation/tree/main/e2e-example)
/// directory in the oxide-typed-generation repository.
#[proc_macro]
pub fn impl_typed_generation_kinds(input: TokenStream) -> TokenStream {
    internals::impl_typed_generation_kinds(input.into())
        .into_token_stream()
        .into()
}
