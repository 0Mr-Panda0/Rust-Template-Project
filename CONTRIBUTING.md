# Contributing to Rust Template Project

Thank you for your interest in contributing to this project!

## Development Setup

### Nix / NixOS Users (Recommended)

This project uses [Devenv](https://devenv.sh) to configure a hermetic developer shell with Rust, rust-analyzer, clippy, and git pre-commit hooks.

1. Ensure Nix flakes are enabled on your machine.
2. Activate the shell:
   ```bash
   devenv shell
   ```
   *(Or allow `direnv` with `direnv allow` for automatic activation).*

### Non-Nix Users

1. Install Rust via [rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Add necessary components:
   ```bash
   rustup component add clippy rustfmt
   ```

---

## Validation & Workflow Commands

Before submitting a pull request, verify that your changes pass formatting, linting, and tests:

### Nix / Devenv
```bash
# Check code formatting without modifying files
devenv shell format-check

# Format files in place
devenv shell format

# Run Clippy linter with zero warnings tolerated
devenv shell lint

# Run cargo check
devenv shell typecheck

# Run unit tests and documentation tests
devenv shell test

# Run devenv integration tests and git pre-commit hooks
devenv test
```

### Standard Cargo
```bash
# Check formatting
cargo fmt --all -- --check

# Format code
cargo fmt --all

# Run Clippy
cargo clippy -- -D warnings

# Type check
cargo check

# Run tests
cargo test -- --nocapture
```

---

## Pull Request Guidelines

1. **Keep Commits Focused**: Each commit should represent an atomic, logical change.
2. **Adhere to Code Quality**: Code must pass `clippy -- -D warnings` and `cargo fmt`.
3. **Include Tests**: Add unit tests for new functionality and doctests for public API functions.
4. **Update Documentation**: Keep `README.md` and doc comments up to date with any API or command changes.

