# Track Spec: Implement a simple web UI to introduce the project and accept user input for expanding and sanitizing Bilibili URLs

## Overview

Add a lightweight, embedded web interface that explains B23 Rust's privacy-preserving Bilibili link resolver and lets visitors submit a short or long Bilibili URL for expansion and allow-list-based sanitization. The UI will call the existing REST API rather than duplicate resolver behavior in the browser, keeping deployment as a single Rust binary.

## Functional Requirements

1. Serve a single HTML page from the Rust application at `/` using a compile-time embedded asset.
2. Present the service purpose, privacy benefit, and attribution to the original `b23.wtf` project with a link.
3. Provide a labeled URL input and submit action that calls `GET /api/v1/clean` with the user-provided URL.
4. Show the cleaned destination URL after a successful request, with a direct link and a copy action.
5. Present clear validation, loading, and API error states without navigating away from the page.
6. Preserve the existing API routes and their JSON/text response behavior.

## Non-Functional Requirements

1. Use a single static HTML asset with vanilla JavaScript and a CDN-hosted CSS framework; do not add a Node.js frontend build step.
2. Keep user input out of HTML interpolation; render results through safe DOM APIs and encode the API query value with `URLSearchParams`.
3. Ensure the page is usable by keyboard and includes accessible labels, status messaging, and sufficient responsive layout behavior.
4. Add automated Rust route tests for the root page and retain the project's `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` quality gate.

## Acceptance Criteria

1. `GET /` returns an HTML document containing the product introduction, attributed `b23.wtf` link, and URL-cleaning form.
2. Submitting a URL sends an encoded request to `/api/v1/clean` and displays the API's `sanitized_url` response as a clickable destination.
3. Empty input, failed requests, and malformed API responses show understandable inline error feedback without an unhandled browser exception.
4. The result can be copied through the browser Clipboard API, with feedback when copying succeeds or is unavailable.
5. The existing API endpoint tests remain passing, and a test verifies the root route returns the embedded HTML with `text/html` content type.
6. `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` pass.

## Out of Scope

- Changing resolver allow-list rules, redirect behavior, DNS handling, or API response schemas.
- User accounts, persisted settings, analytics, localization, or a JavaScript build pipeline.
- Implementing automatic browser redirects from the UI.
