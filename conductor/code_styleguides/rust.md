# Rust Code Style Guide

## Formatting
- Use `rustfmt` with default settings for all code formatting.
- Ensure `cargo fmt --check` passes in CI.

## Linting
- Use `clippy` for linting.
- Address all `clippy` warnings. Consider `#![deny(clippy::all)]` for stricter enforcement if appropriate.
- Ensure `cargo clippy -- -D warnings` passes in CI.

## Naming Conventions
- Follow standard Rust naming conventions:
  - `CamelCase` for types, traits, and enums.
  - `snake_case` for functions, methods, variables, and modules.
  - `SCREAMING_SNAKE_CASE` for constants and statics.

## Error Handling
- Use `Result` for recoverable errors.
- Use `anyhow` for application-level errors where the exact type doesn't matter.
- Use `thiserror` for library-level errors or specific error types that need to be matched against.
- Avoid `unwrap()` and `expect()` in production code unless you can prove the invariant holds. Prefer `?` for propagation.

## Documentation
- Document all public items (functions, structs, traits) using `///`.
- Include examples in documentation for complex APIs.
- Provide module-level documentation using `//!`.
