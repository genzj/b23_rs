# Specification: HTML Linting and Formatting Integration

## 1. Overview
The project currently relies on Cargo for Rust linting and formatting. To maintain code quality across all project files, we need to introduce a fast, Rust-based tooling solution for our frontend HTML codebase. Biome will be used to lint and format the `src/index.html` file.

## 2. Functional Requirements
- **Tool Selection:** Integrate `Biome` as the primary tool for HTML linting and formatting.
- **Local Pre-commit Hook:** Update the existing `git-hooks/pre-commit` script to execute Biome checks before committing. The commit should be blocked if there are errors.
- **CI/CD Integration:** Update the project's GitHub Actions workflow to run Biome checks to ensure no unformatted or lint-failing HTML code makes it into the `main` branch.

## 3. Non-Functional Requirements
- **Performance:** Biome is fast, Rust-based, and does not require Node.js dependencies in the final environment.

## 4. Out of Scope
- Rewriting the frontend to use a complex build step or JavaScript framework.
