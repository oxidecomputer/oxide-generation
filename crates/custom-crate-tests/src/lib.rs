//! Tests for custom crate name functionality.

#[cfg(test)]
mod ui;

// my-custom-generation must be a normal dep so internal-schemars08-tests can
// enable my-custom-generation/schemars08 for trybuild's generated project.
// This marks it as used to silence the cargo::unused_dependencies lint.
use my_custom_generation as _;
