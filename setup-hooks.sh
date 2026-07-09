#!/usr/bin/env bash

# Create git pre-commit hook
cat << 'EOF' > .git/hooks/pre-commit
#!/usr/bin/env bash
set -e

echo "Running pre-commit checks..."

echo "Checking formatting..."
cargo fmt --check

echo "Running clippy..."
cargo clippy -- -D warnings

echo "Running tests..."
cargo test

echo "All checks passed!"
EOF

# Make the hook executable
chmod +x .git/hooks/pre-commit

echo "Git pre-commit hook installed successfully."
