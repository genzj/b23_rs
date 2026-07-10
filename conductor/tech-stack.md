# Technology Stack

## Core Language
- **Rust:** Chosen for its performance, memory safety, and robust concurrency model, making it ideal for a high-throughput, low-latency URL resolver.

## Backend Framework & Libraries
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum) - A highly ergonomic and fast web framework built on top of Tokio. Chosen for excellent routing, simple handlers, and strong ecosystem integration.
- **HTTP Client:** [Reqwest](https://github.com/seanmonstar/reqwest) - Used to perform HTTP requests to `b23.tv` links and follow redirects to discover the actual destination URL.
- **DNS Resolution:** [hickory-resolver](https://github.com/hickory-dns/hickory-dns) - Injected into the HTTP client to implement DNS caching and enforce connections only to certified upstream domains.
- **Serialization:** [Serde](https://serde.rs/) & [serde_json](https://github.com/serde-rs/json) - The standard Rust framework for serializing and deserializing data structures, required for the RESTful JSON API.
- **Error Handling:** [anyhow](https://github.com/dtolnay/anyhow) / [thiserror](https://github.com/dtolnay/thiserror) - For clean and descriptive error propagation.
- **Testing (Mocking):** [wiremock](https://github.com/LukeMathWalker/wiremock-rs) - HTTP mocking to enforce zero external network access in unit tests.

## Frontend Framework & Libraries
- **Architecture:** Single HTML file (`index.html`).
- **Styling:** Simple, CDN-based CSS framework (e.g., Bootstrap 5 or Tailwind CSS via CDN) to ensure rapid development without requiring a complex frontend build process (Node.js/npm).
- **JavaScript:** Vanilla JS to handle API requests and DOM updates, keeping the frontend extremely lightweight.
- **Integration:** The `index.html` file is embedded directly into the Rust backend binary at compile time (e.g., using Rust's `include_str!` macro). This generates a single self-contained executable for incredibly simple deployment.

## Infrastructure & Deployment
- **Containerization:** Docker. Utilizing a multi-stage Dockerfile to compile the Rust binary and package it in a lightweight base image (e.g., `distroless` or `alpine`) for minimal attack surface and image size.
- **CI/CD Pipeline:** GitHub Actions. Automated workflows to lint, test, and build the Docker image, followed by pushing the final image to both GitHub Container Registry (GHCR) and Docker Hub.
