# Changelog

<!-- next-header -->
## Unreleased - ReleaseDate

## [0.1.3] - 2026-08-21

### Fixed

- `TypedGeneration<T>`'s JSON schema now carries the same description as `Generation`'s, including when the `x-rust-type` extension is lifted from a kind's schema.

## [0.1.2] - 2026-08-21

Fixed README links.

## [0.1.1] - 2026-08-21

### Changed

Fix up feature set:

- The `std` feature now also enables the `std` feature of `daft`, `serde`, and `slog` when the corresponding integration is enabled.
- The `serde` and `slog2` features no longer require `std`, and are usable in `no_std` builds.
- The `proptest1` feature now explicitly enables `std`, which proptest requires.
- The `schemars08` feature no longer enables schemars' `derive` feature.

## [0.1.0] - 2026-08-21

Initial release.

<!-- next-url -->
[0.1.3]: https://github.com/oxidecomputer/oxide-generation/releases/oxide-generation-0.1.3
[0.1.2]: https://github.com/oxidecomputer/oxide-generation/releases/oxide-generation-0.1.2
[0.1.1]: https://github.com/oxidecomputer/oxide-generation/releases/oxide-generation-0.1.1
[0.1.0]: https://github.com/oxidecomputer/oxide-generation/releases/oxide-generation-0.1.0
