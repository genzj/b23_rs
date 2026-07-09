# Implementation Plan: Enforce Zero External Network Access in Unit Tests

## Phase 1: Setup and Configuration
- [ ] Task: Add `wiremock` as a dev-dependency in `Cargo.toml`.
- [ ] Task: Refactor the application code to allow dependency injection of the target URL instead of hardcoding `b23.tv`.

## Phase 2: Mocking Infrastructure Implementation
- [ ] Task: Set up a global test helper to initialize a `wiremock` server for tests.
- [ ] Task: Configure the mock server to reject unhandled requests to enforce zero external network access.

## Phase 3: Unit Test Refactoring
- [ ] Task: Identify all existing unit tests making external HTTP requests.
- [ ] Task: Update identified tests to use the `wiremock` server.
- [ ] Task: Ensure the application client in tests points to the mock server URL.

## Phase 4: Validation
- [ ] Task: Run `cargo clippy -- -D warnings` and `cargo fmt --check`.
- [ ] Task: Disconnect from the network and run `cargo test` to verify zero external dependencies.
- [ ] Task: Run full `cargo test` locally to ensure all tests pass.
