# b23_rs

A tracing-free bilibili short URL service clone written in Rust.

## Development

This project uses Git hooks to enforce code quality before commits.

**To set up the pre-commit hook:**

Run the setup script once after cloning the repository:

```bash
./setup-hooks.sh
```

This will configure Git to automatically run `cargo fmt`, `cargo clippy`, and `cargo test` before allowing a commit.
