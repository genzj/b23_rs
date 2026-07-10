# Specification: Secure Resolver with DNS Caching

## Overview
Improve the security of the application by ensuring the URL resolver only connects to allowed upstream domains, rather than blindly following user-submitted hostnames. This prevents Server-Side Request Forgery (SSRF) and related vulnerabilities. A custom DNS resolver will be injected into `reqwest`, forcing resolution to allowed domains, implementing DNS caching for performance, and the HTTP client will be reused across requests via Axum state.

## Functional Requirements
1. **Custom DNS Resolution**: Integrate `hickory-dns` (specifically `hickory-resolver`) to perform DNS lookups.
2. **Domain Override/Whitelisting**: Configure allowed upstream domains via the `UPSTREAM_DOMAINS` environment variable (comma-separated). Default value: `b23.tv,d.bilibili.com`. The custom resolver will ignore the hostname in the requested URL and instead resolve the domains in `UPSTREAM_DOMAINS`, returning the IP of the first successfully resolvable domain. This routes the traffic safely to the certified upstream servers while maintaining the user-requested URL/Host.
3. **DNS Caching**: The DNS client must respect TTLs in DNS responses and cache the IPs to reduce DNS queries and latency (leveraging `hickory-resolver`'s built-in caching).
4. **Client Reuse**: Create a single `reqwest::Client` instance (configured with the custom resolver) at startup and share it across all handlers using Axum's `State`.

## Non-Functional Requirements
- **Performance**: Caching DNS responses ensures minimal latency impact. Sharing the `reqwest::Client` utilizes connection pooling and eliminates per-request setup overhead.
- **Security**: Prevents the server from connecting to arbitrary IPs.

## Acceptance Criteria
- `reqwest::Client` is initialized once and shared via Axum state.
- `hickory-resolver` is implemented as the custom DNS resolver for the `reqwest` client.
- Outgoing requests to any URL are physically routed to the IP of a domain in `UPSTREAM_DOMAINS` (defaulting to `b23.tv` or `d.bilibili.com`).
- DNS responses are cached according to their TTL.

## Out of Scope
- Frontend UI modifications.
