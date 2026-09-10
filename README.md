# Rust Template Project

This is a modern Rust project template featuring rapid environment provisioning, strict linting, and seamless integration for both Nix and non-Nix users.

[![CI Pipeline](https://github.com/0Mr-Panda0/Rust-Template-Project/actions/workflows/main.yml/badge.svg)](https://github.com/0Mr-Panda0/Rust-Template-Project/actions/workflows/main.yml)
[![Documentation](https://github.com/0Mr-Panda0/Rust-Template-Project/actions/workflows/docs.yml/badge.svg)](https://0mr-panda0.github.io/Rust-Template-Project/)
[![License: CC0-1.0](https://img.shields.io/badge/License-CC0_1.0-lightgrey.svg)](https://creativecommons.org/publicdomain/zero/1.0/)

---

## Using this Template

### Option A: Via `cargo-generate` (Recommended)

Generate a new, personalized project directly with interactive prompts:

```bash
cargo generate gh:0Mr-Panda0/Rust-Template-Project
```

### Option B: Via GitHub

Click the green **"Use this template"** button at the top of the repository to create a new repository from this template.

---

## Getting Started

### Nix / NixOS Users

1. **Clone your repository:**

```bash
git clone https://github.com/0Mr-Panda0/Rust-Template-Project
cd Rust-Template-Project
```

2. **Enable experimental features in your `configuration.nix` (if you haven't already):**

```nix
nix.settings.experimental-features = [ "nix-command" "flakes" ];
```

3. **Activate the developer environment:**

```bash
devenv shell
```

4. **Optional: Configure `direnv` for automatic activation when entering the project directory:**

```bash
echo "use devenv" > .envrc
direnv allow
```

### General Users (Non-NixOS)

1. **Clone your repository:**

```bash
git clone https://github.com/0Mr-Panda0/Rust-Template-Project
cd Rust-Template-Project
```

2. **Install Rust:**

```bash
# Windows
winget install --id=Rustlang.Rustup -e

# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. **Build the project:**

```bash
cargo build
```

## CLI Usage

```bash
# Display help and available options
cargo run -- --help

# Run with default greeting
cargo run

# Greet a specific person
cargo run -- --name Ferris

# Use formal greeting
cargo run -- --name Ferris --formal

# Increase logging verbosity (-v for debug, -vv for trace)
cargo run -- -v --name Ferris

# Control logging via standard RUST_LOG environment variable
RUST_LOG=debug cargo run -- --name Ferris

# Generate shell completions (bash, zsh, fish, powershell, elvish)
cargo run -- --generate-completions bash
```

## Required Tools

| Tool       | Purpose                        |
| ---------- | ------------------------------ |
| `rustc`    | Rust compiler                  |
| `cargo`    | Build system and package manager |
| `clippy`   | Linter                         |
| `rustfmt`  | Code formatter                 |

## Devenv Commands (Nix / NixOS Users)

| Command                      | Description                                          |
| ---------------------------- | ---------------------------------------------------- |
| `devenv shell lint`          | Lint code with clippy                                |
| `devenv shell format`        | Format code in place with rustfmt                    |
| `devenv shell format-check`  | Check code formatting without modifying              |
| `devenv shell typecheck`     | Check code with cargo check                          |
| `devenv shell unit-test`     | Run unit & doc tests with cargo test                 |
| `devenv shell audit`         | Audit advisories, bans, and licenses with cargo-deny |
| `devenv shell bench`         | Run micro-benchmarks with Divan                      |
| `devenv shell coverage`      | Generate code coverage with cargo-llvm-cov           |
| `devenv shell nextest`       | Run fast parallel tests with cargo-nextest           |
| `devenv shell watch`         | Live background checker with bacon                   |
| `devenv test`                | Run devenv tests and pre-commit hooks                |

## Normal Commands

| Command                              | Description                                          |
| ------------------------------------ | ---------------------------------------------------- |
| `cargo clippy -- -D warnings`        | Lint code with clippy                                |
| `cargo fmt --all`                    | Format code in place with rustfmt                    |
| `cargo fmt --all -- --check`         | Check code formatting without modifying              |
| `cargo check`                        | Type check with cargo check                          |
| `cargo test -- --nocapture`          | Run tests with cargo test                            |
| `cargo deny check`                   | Audit advisories, bans, and licenses with cargo-deny |
| `cargo bench`                        | Run micro-benchmarks with Divan                      |
| `cargo llvm-cov`                     | Generate code coverage with cargo-llvm-cov           |

## CI/CD Pipelines

This project uses GitHub Actions for continuous integration, documentation, and automated releases:

### Continuous Integration (`main.yml`)
Triggered on every push and pull request to `main`:
- Code formatting check (`format-check` / `cargo fmt --check`)
- Strict linting via Clippy (`lint` / `cargo clippy -- -D warnings`)
- Unit and doc tests via Cargo (`unit-test` / `cargo test`)
- Cross-platform matrix test on Ubuntu, macOS (Apple Silicon), and Windows
- MSRV verification ensuring compatibility with Rust 1.85
- Supply chain security & license audit via `cargo-deny`
- Code coverage reporting via `cargo-llvm-cov`
- Devenv integration tests & pre-commit hooks (`devenv test`)

### Documentation (`docs.yml`)
Triggered on pushes to `main` with code changes:
- Builds complete API documentation (`cargo doc --no-deps --all-features`)
- Automatically publishes and deploys to **GitHub Pages**

### Automated Releases (`release.yml`)
Triggered on pushing version tags (e.g. `git tag v0.1.0 && git push origin v0.1.0`):
- Cross-compiles optimized release binaries for Linux (`x86_64`), macOS (`x86_64` + `aarch64`), and Windows (`x86_64`)
- Packages archives (`.tar.gz` and `.zip`) with SHA-256 checksums
- Publishes a GitHub Release with downloadable binary assets attached

## Notes

- `.direnv` and `.devenv` are intentionally excluded from version control.
- `devenv.lock` and `Cargo.lock` are tracked in Git to guarantee strict environment reproducibility across machines.
- `target/` is excluded from version control as it contains build artifacts.
