# Implementation Plan: Backend APIs

## Phase 1: Setup and Shared Component Implementation
- [ ] Task: Set up the core module for URL resolution
    - [ ] Create `src/resolver.rs` (or similar) to encapsulate the URL processing logic.
    - [ ] Add `reqwest` client initialization (ideally reusing a single client instance).
- [ ] Task: Implement the URL resolution and cleaning logic
    - [ ] Write a function to take a URL, resolve HTTP redirects (e.g., using `reqwest`), and return the final destination URL.
    - [ ] Write a function to parse the resolved URL and strip tracking parameters (e.g., `spm_id_from`, `vd_source`), keeping relevant ones like `p`.
    - [ ] Define the response data structure (struct) representing the cleaning result (sanitized URL, raw URL, stripped params map).
- [ ] Task: Write unit tests for the core logic
    - [ ] Add tests in `src/resolver.rs` (or `src/resolver/tests.rs`) to verify URL redirect resolution.
    - [ ] Add a specific test for the `https://b23.tv/rlUSCcz` case to ensure it resolves and cleans to `https://www.bilibili.com/video/BV1BECcB3EG6?p=1`.

## Phase 2: RESTful API Implementation
- [ ] Task: Implement the `/api/v1/clean` endpoint
    - [ ] Define Axum handler for `GET /api/v1/clean`.
    - [ ] Define query parameter struct (for `url` and optional `format`).
    - [ ] Integrate the core logic component from Phase 1.
    - [ ] Implement response formatting: return JSON by default or pure text if `format=text`.
    - [ ] Implement error handling to return `400 Bad Request` with a JSON error payload for invalid URLs.
- [ ] Task: Implement the `/api/v1/redirect` endpoint
    - [ ] Define Axum handler for `GET /api/v1/redirect`.
    - [ ] Reuse the core logic component to get the sanitized URL.
    - [ ] Return an HTTP 302 Redirect response using Axum's `Redirect` utility.
- [ ] Task: Integrate handlers into the Axum Router
    - [ ] Add the routes `/api/v1/clean` and `/api/v1/redirect` to the main application router in `src/main.rs`.

## Phase 3: Testing and Quality Assurance
- [ ] Task: Write integration tests for API endpoints
    - [ ] Test `/api/v1/clean` with both `format=json` and `format=text`.
    - [ ] Test `/api/v1/redirect` for proper 302 response and `Location` header.
    - [ ] Test error scenarios (e.g., missing URL, unresolvable URL) to ensure a 400 JSON response.
- [ ] Task: Ensure code meets workflow quality standards
    - [ ] Run `cargo fmt`.
    - [ ] Run `cargo clippy -- -D warnings`.
    - [ ] Run `cargo test` to verify all tests pass.
