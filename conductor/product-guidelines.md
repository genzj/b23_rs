# Product Guidelines

## Core Principles
- **Privacy-First:** User privacy is paramount. Stripping tracking parameters reliably and never logging user URLs or PII (Personally Identifiable Information) in production.
- **Speed & Efficiency:** The service must provide near-instantaneous responses. Leverage Rust's performance and ensure low overhead in Docker containers.
- **Simplicity:** The API should be intuitive, RESTful, and easy to integrate for developers.

## Design & UX
- **Minimalist Interface:** Any web UI provided should be ultra-minimalist, fast-loading, and responsive, focusing solely on the core functionality (entering a URL and getting a clean one back).
- **Clear Feedback:** If a URL is invalid or cannot be resolved, return clear, standard HTTP error codes and descriptive JSON error messages.

## Technical Architecture & Code Quality
- **Idiomatic Rust:** Write safe, idiomatic Rust code following standard formatting (`rustfmt`) and linting (`clippy`) guidelines.
- **Comprehensive Testing:** Implement unit and integration tests to verify parameter stripping, edge cases, and API contract correctness.
- **Containerization First:** The application must be designed to run statelessly in Docker, configurable via environment variables.

## Deployment & CI/CD
- **Automated Workflows:** Every push should trigger linting, testing, and Docker image building.
- **Immutable Artifacts:** Images published to GHCR and Docker Hub should be tagged accurately (e.g., semver and `latest`).
