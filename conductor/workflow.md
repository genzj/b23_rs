# Project Workflow

## Branching Strategy
We follow a simplified Git Flow model:
- `main`: The primary, production-ready branch. All commits must be stable.
- `feat/<feature-name>`: Branches for developing new features.
- `fix/<bug-name>`: Branches for bug fixes.
- `chore/<chore-name>`: Branches for routine tasks, dependency updates, or refactoring.

## Commit Messages
We adhere to the [Conventional Commits](https://www.conventionalcommits.org/) specification:
- `<type>[optional scope]: <description>`
- Examples:
  - `feat: add REST API for link resolution`
  - `fix(docker): resolve image build failure`
  - `docs: update README with b23.wtf attribution`

## Local Development & Git Hooks
To accelerate the feedback loop and save time, a pre-commit git hook must be configured to run automatically before every commit:
- The hook will execute `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test`.
- Commits will be blocked if any of these checks fail.

## Pull Request Process
1. Push your feature/fix branch to the remote repository.
2. Open a Pull Request targeting the `main` branch.
3. Ensure all CI checks pass.
4. Request a review from at least one other contributor (if applicable).
5. Once approved and checks pass, merge using "Squash and Merge" to keep the `main` history linear.

## CI/CD Integration
- Every PR will automatically trigger the GitHub Actions workflow to verify formatting, linting, and tests as a secondary safeguard.
- Merges to `main` will trigger the build and publication of the Docker image to GHCR and Docker Hub.
