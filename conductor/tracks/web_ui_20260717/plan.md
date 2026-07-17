# Implementation Plan: Implement a simple web UI to introduce the project and accept user input for expanding and sanitizing Bilibili URLs

## Phase 1: Page and Server Integration

- [x] Task: Define the embedded page contract and server route 1f7fa88
    - [x] Document the root-page response and its relationship to the existing `/api/v1/clean` API.
    - [x] Write a Rust route test asserting that `GET /` returns the page with a `text/html` content type.
    - [x] Implement a compile-time embedded HTML asset and register the root route without changing existing API routes.
- [x] Task: Build the responsive introduction and URL form 1f7fa88
    - [x] Add concise product, privacy, and `b23.wtf` attribution content.
    - [x] Add an accessible labeled URL input, submit control, and stable regions for status, errors, and results.
    - [x] Use the selected CDN CSS framework without adding a frontend build dependency.

## Phase 2: Sanitization Interaction

- [x] Task: Implement the client-side clean request workflow 1f7fa88
    - [x] Add vanilla JavaScript validation for empty input and a loading state during requests.
    - [x] Request `/api/v1/clean` with a URL encoded using `URLSearchParams`.
    - [x] Safely render a successful `sanitized_url` result as text and as a direct destination link.
- [x] Task: Implement result and failure handling 1f7fa88
    - [x] Add a copy action with Clipboard API feedback and a graceful unavailable/failure state.
    - [x] Display network, non-success API, and malformed-response failures in the inline error region.
    - [x] Verify keyboard operation and responsive behavior manually in a browser.
- [x] Task: Add a collapsed parameter review to clean results 966bc66
    - [x] Use the API's `raw_url` and `stripped_params` fields instead of duplicating sanitization rules in the browser.
    - [x] Safely display each expanded-URL parameter, its value, and whether B23 Rust kept or removed it.
    - [x] Keep the review collapsed until a user chooses to inspect it.

## Phase 3: Verification and Documentation

- [x] Task: Expand server-side regression coverage 1f7fa88
    - [x] Verify the root page test runs alongside the existing clean and redirect endpoint tests.
    - [x] Confirm existing JSON and text API behaviors remain unchanged.
- [x] Task: Run project quality checks 1f7fa88
    - [x] Run `cargo fmt --check`.
    - [x] Run `cargo clippy -- -D warnings`.
    - [x] Run `cargo test`.
- [x] Task: Update user-facing documentation as needed 1f7fa88
    - [x] Document the root web UI and its relationship to the API in the README.
