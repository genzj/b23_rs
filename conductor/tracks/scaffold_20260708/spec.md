# Track Specification: Scaffold the project (Axum, Docker, CI/CD)

## Overview
This track focuses on the initial scaffolding of a tracing-free Bilibili short URL resolution service. It sets up the foundational project structure, containerization, and continuous integration pipeline according to the defined Product Guide and Tech Stack.

## Functional Requirements
- **Rust Initialization:** Initialize a standard flat Cargo project (`src/main.rs`).
- **Web Framework Setup:** Add `axum` and `tokio` dependencies and implement a basic `/api/health` endpoint that returns a 200 OK status.
- **Dockerization:** Create a multi-stage `Dockerfile` that:
  1. Uses a Rust base image to compile the application.
  2. Copies the compiled binary into a minimal `distroless` image for execution.
- **Git Hooks:** Create a setup script (`setup-hooks.sh`) that installs a pre-commit git hook to automatically run `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test`.
- **CI/CD:** Define a GitHub Actions workflow (`.github/workflows/ci.yml`) to:
  1. Run formatting checks, linting, and tests on every push and pull request.
  2. Build the Docker image and push it to GitHub Container Registry (GHCR) and Docker Hub upon merging to the `main` branch.

## Non-Functional Requirements
- Minimal footprint and high security (Distroless base image).
- Easy onboarding for developers (automated git hook setup).

## Out of Scope
- Implementing the actual URL resolution logic and parameter stripping.
- Building the frontend HTML interface.
