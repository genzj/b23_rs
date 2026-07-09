# Initial Concept
Explore the project https://github.com/nicholascw/b23.wtf and create a clone of it with Rust. The project should support Docker deployment. The Docker image shall be built by GitHub workflow and hosted on GitHub Container Registry and Docker Hub.

# Product Guide

## Vision
To provide a fast, memory-safe, and self-hostable service implemented in Rust that resolves Bilibili short links (`b23.tv`) and automatically strips all privacy-invasive tracking parameters. 

## Goals
1. **Tracing-Free Resolution:** Automatically resolve Bilibili short links to their full destinations and strip away tracking query parameters (e.g., `spm_id_from`, `vd_source`).
2. **RESTful API:** Provide modern, well-structured RESTful APIs for programmatic access, departing from the legacy API design for better developer experience.
3. **High Performance:** Ensure minimal latency and low resource footprint by utilizing Rust.
4. **DevOps & Deployment:** Streamline deployment via Docker. Use GitHub Actions for continuous integration, automatically building and publishing Docker images to GitHub Container Registry (GHCR) and Docker Hub.
5. **Attribution:** Clearly state credits to the original `b23.wtf` repository in the `README.md` as "inspired by" with a link.

## Target Users
- Users who want to share clean Bilibili links without embedded tracking identifiers.
- Developers seeking a fast, reliable, RESTful, and containerized link resolver to integrate into their own applications or bots.

## Key Features
- **Link Cleaning:** Takes a provided Bilibili URL (short or long), resolves any redirects, and sanitizes it by removing tracking parameters.
- **RESTful API:**
  - `GET /api/v1/clean?url={url}&format={json|text}`: Returns a JSON response (default) or plain text (`format=text`) containing the cleaned URL.
  - `GET /api/v1/redirect?url={url}`: HTTP 302 redirect directly to the cleaned URL.
  - `PUT /api/v1/settings/auto-redirect`: Updates client preference (e.g., via cookie) to toggle automatic redirection on the web UI.
- **Dockerized:** Fully containerized for easy scaling and deployment in any environment.
- **CI/CD Pipeline:** Automated GitHub workflow for building and publishing production-ready Docker images.
