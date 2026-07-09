#!/usr/bin/env bash
set -e

# Configure git to use the custom hooks directory
git config core.hooksPath git-hooks

echo "Git pre-commit hook configured successfully."
