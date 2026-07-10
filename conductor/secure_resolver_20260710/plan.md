# Implementation Plan

## Phase 1: Setup and Configuration
- [ ] Task: Update dependencies
    - [ ] Add `hickory-resolver` crate to `Cargo.toml`.
- [ ] Task: Configuration update
    - [ ] Add `UPSTREAM_DOMAINS` parsing to the application's configuration logic (defaulting to `b23.tv,d.bilibili.com`).

## Phase 2: Custom DNS Resolver Implementation
- [ ] Task: Implement custom resolver
    - [ ] Create a struct that implements the `reqwest::dns::Resolve` trait.
    - [ ] Initialize `hickory_resolver::TokioAsyncResolver` within this struct to utilize its built-in DNS caching.
    - [ ] Implement the `resolve` method to ignore the requested hostname and instead sequentially resolve the domains configured in `UPSTREAM_DOMAINS`, returning the IPs of the first successfully resolved domain.

## Phase 3: Client Reuse via Axum State
- [ ] Task: Refactor HTTP Client initialization
    - [ ] Instantiate the custom resolver using the parsed `UPSTREAM_DOMAINS`.
    - [ ] Build a single `reqwest::Client` using the custom resolver at application startup.
- [ ] Task: Integrate with Axum State
    - [ ] Add the `reqwest::Client` to the Axum application state (e.g., `AppState`).
    - [ ] Update route handlers to extract the `reqwest::Client` from the `State` rather than creating a new one per request.

## Phase 4: Testing and Verification
- [ ] Task: Unit tests
    - [ ] Add/update tests to verify that the custom resolver returns IPs for the configured upstream domains.
    - [ ] Verify that request handlers successfully utilize the shared client from state.
- [ ] Task: Local validation (Pre-commit checks)
    - [ ] Run `cargo fmt --check`.
    - [ ] Run `cargo clippy -- -D warnings`.
    - [ ] Run `cargo test`.
