# Implementation Plan: Implement a simple web UI to introduce the project and accept user input for expanding and sanitizing Bilibili URLs

## Phase 1: Page and Server Integration

- [ ] Task: Define the embedded page contract and server route
    - [ ] Document the root-page response and its relationship to the existing `/api/v1/clean` API.
    - [ ] Write a Rust route test asserting that `GET /` returns the page with a `text/html` content type.
    - [ ] Implement a compile-time embedded HTML asset and register the root route without changing existing API routes.
- [ ] Task: Build the responsive introduction and URL form
    - [ ] Add concise product, privacy, and `b23.wtf` attribution content.
    - [ ] Add an accessible labeled URL input, submit control, and stable regions for status, errors, and results.
    - [ ] Use the selected CDN CSS framework without adding a frontend build dependency.

## Phase 2: Sanitization Interaction

- [ ] Task: Implement the client-side clean request workflow
    - [ ] Add vanilla JavaScript validation for empty input and a loading state during requests.
    - [ ] Request `/api/v1/clean` with a URL encoded using `URLSearchParams`.
    - [ ] Safely render a successful `sanitized_url` result as text and as a direct destination link.
- [ ] Task: Implement result and failure handling
    - [ ] Add a copy action with Clipboard API feedback and a graceful unavailable/failure state.
    - [ ] Display network, non-success API, and malformed-response failures in the inline error region.
    - [ ] Verify keyboard operation and responsive behavior manually in a browser.

## Phase 3: Verification and Documentation

- [ ] Task: Expand server-side regression coverage
    - [ ] Verify the root page test runs alongside the existing clean and redirect endpoint tests.
    - [ ] Confirm existing JSON and text API behaviors remain unchanged.
- [ ] Task: Run project quality checks
    - [ ] Run `cargo fmt --check`.
    - [ ] Run `cargo clippy -- -D warnings`.
    - [ ] Run `cargo test`.
- [ ] Task: Update user-facing documentation as needed
    - [ ] Document the root web UI and its relationship to the API in the README.
