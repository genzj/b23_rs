# Implementation Plan

## Phase 1: Setup Translation Resources
- [ ] Task: Create a directory for locale files (e.g., `src/locales`).
- [ ] Task: Create `en.json` containing all English UI text strings (excluding brand names).
- [ ] Task: Create `zh-cn.json` containing the corresponding Simplified Chinese translations.

## Phase 2: Backend Integration
- [ ] Task: Add necessary dependencies for JSON parsing (`serde`, `serde_json`) or templating in `Cargo.toml`.
- [ ] Task: Modify the main route handler in Rust to parse incoming `Cookie` and `Accept-Language` headers.
- [ ] Task: Implement language detection logic (Cookie preference takes priority over `Accept-Language`, fallback to English).
- [ ] Task: Embed the locale JSON files into the Rust binary at compile time.
- [ ] Task: Update the `index.html` serving logic to dynamically inject the resolved language dictionary into the HTML response (e.g., as a JavaScript global variable `window.i18n`).

## Phase 3: Frontend Implementation
- [ ] Task: Modify `src/index.html` logic to populate all static text elements, placeholders, and the page title using the injected `window.i18n` dictionary.
- [ ] Task: Add the language switcher UI (dropdown with flag emoji and short names: "🇺🇸 EN", "🇨🇳 中文").
- [ ] Task: Add JavaScript logic to handle language changes (set the language cookie and reload the page).

## Phase 4: Validation
- [ ] Task: Verify language fallback and cookie logic in the browser.
- [ ] Task: Ensure the UI correctly updates without affecting brand names.
- [ ] Task: Run `cargo fmt`, `cargo clippy`, and `cargo test` to ensure code quality and prevent CI failures.

## Phase: Review Fixes
- [x] Task: Apply review suggestions 45c835a
