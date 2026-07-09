# Specification: Implement the Backend APIs

## Overview
This track implements the core backend RESTful APIs for the Bilibili short link cleaner using Rust and Axum. It focuses on the `/api/v1/clean` and `/api/v1/redirect` endpoints.

## Functional Requirements
1. **Endpoint: `GET /api/v1/clean`**
   - Query Parameters:
     - `url`: The Bilibili URL to clean (e.g., `https://b23.tv/rlUSCcz`).
     - `format` (optional): `json` (default) or `text`.
   - Behavior: Resolves redirects of the provided URL and strips tracking parameters (e.g., `spm_id_from`, `vd_source`).
   - Response:
     - If `format=json` (or omitted): Returns a JSON object containing:
       - `sanitized_url`: The cleaned URL.
       - `raw_url`: The expanded, unsanitized URL.
       - `stripped_params`: A map of the query parameters that were removed.
     - If `format=text`: Returns a pure text response with just the sanitized URL.
   
2. **Endpoint: `GET /api/v1/redirect`**
   - Query Parameter: `url` (the Bilibili URL to clean).
   - Behavior: Resolves and cleans the URL (same logic as `/clean`).
   - Response: Returns an HTTP 302 redirect directly to the cleaned URL.

3. **Core URL Resolution Logic Component**
   - The common URL resolution, redirect following, and tracking parameter stripping logic must be extracted into a reusable, independent component or module.
   - API handlers must remain extremely lightweight, delegating the core logic to this shared component.

## Non-Functional Requirements
- **Error Handling:** Invalid, malformed, or unreachable URLs must return an HTTP `400 Bad Request` with an informative JSON error message. This JSON error format applies regardless of the requested `format`.
- **Performance:** Use `reqwest` for fast asynchronous HTTP requests.

## Acceptance Criteria
- [ ] `GET /api/v1/clean?url=https://b23.tv/rlUSCcz` returns the expected JSON structure with sanitized URL (e.g. `https://www.bilibili.com/video/BV1BECcB3EG6?p=1`), raw URL, and stripped params.
- [ ] `GET /api/v1/clean?url=https://b23.tv/rlUSCcz&format=text` returns the pure text sanitized URL.
- [ ] `GET /api/v1/redirect?url=https://b23.tv/rlUSCcz` returns a 302 status code and the `Location` header is the sanitized URL.
- [ ] Shared logic is isolated from the HTTP handler layer.
- [ ] Failed URL resolution gracefully returns a 400 JSON error.
- [ ] Comprehensive unit tests cover the core URL cleaning logic using provided examples.

## Out of Scope
- Implementation of the `PUT /api/v1/settings/auto-redirect` endpoint.
- Web UI / Frontend integration.
