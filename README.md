# b23_rs

A tracing-free bilibili short URL service clone written in Rust.

## Web Interface

Open the service root at `http://localhost:3000/` to expand a Bilibili short link and remove tracking parameters in the browser. The page calls the existing `GET /api/v1/clean` endpoint and displays a clean, shareable destination URL.

The project is inspired by [b23.wtf](https://github.com/nicholascw/b23.wtf).

## API

- `GET /api/v1/clean?url={url}` returns a JSON object containing `sanitized_url`.
- `GET /api/v1/clean?url={url}&format=text` returns only the cleaned URL.
- `GET /api/v1/redirect?url={url}` redirects to the cleaned URL.

## Development

This project uses Git hooks to enforce code quality before commits.

**To set up the pre-commit hook:**

Run the setup script once after cloning the repository:

```bash
./setup-hooks.sh
```

This will configure Git to automatically run `cargo fmt`, `cargo clippy`, and `cargo test` before allowing a commit.
