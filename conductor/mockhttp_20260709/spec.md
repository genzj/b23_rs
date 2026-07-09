# Specification: Enforce Zero External Network Access in Unit Tests

## Overview
This track focuses on reviewing existing unit tests and implementing a mocking mechanism to ensure that no tests make external HTTP requests. This will improve test reliability, speed, and determinism.

## Functional Requirements
- Integrate the `wiremock` crate for mocking HTTP requests in the test suite.
- Modify the application to allow injecting the mocked `wiremock` server URL during testing, rather than hardcoding external URLs (like `b23.tv`).
- Ensure the mocking infrastructure is applied globally across all unit tests to enforce zero external network access.
- Configure `wiremock` to immediately fail any test that makes an unhandled or unexpected HTTP request.

## Non-Functional Requirements
- Test suite execution time should be reduced by eliminating network latency.
- Tests must be fully deterministic and unaffected by external network conditions or rate limits.

## Acceptance Criteria
- All unit tests run successfully without requiring an active internet connection.
- Any attempt to make an external HTTP request during a unit test results in an immediate test failure.
- `reqwest` usages are properly mocked using `wiremock`.

## Out of Scope
- Modifying the core business logic of the application outside of what is necessary for dependency injection of the HTTP client/URL.
- Adding new feature tests unrelated to the existing network-dependent tests.
