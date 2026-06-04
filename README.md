# Rust Template Project

This is a modern Rust project template featuring rapid environment provisioning, strict linting, and seamless integration for both Nix and non-Nix users.

[![CI Pipeline](https://github.com/0Mr-Panda0/Rust-Project-Template/actions/workflows/main.yml/badge.svg)](https://github.com/0Mr-Panda0/Rust-Project-Template/actions/workflows/main.yml)

---

## Getting Started

### Nix / NixOS Users

1. **Clone the repository:**

```bash
git clone https://github.com/0Mr-Panda0/Rust-Template-Project
cd Rust-Template-Project
```

2. **Enable experimental features in your `configuration.nix` (if you haven't already):**

```nix
nix.settings.experimental-features = [ "nix-command" "flakes" ];
```

3. **Add required devenv inputs:**

```bash
devenv inputs add rust-overlay github:oxalica/rust-overlay --follows nixpkgs
devenv inputs add git-hooks github:cachix/git-hooks.nix --follows nixpkgs
```

4. **Activate the developer environment:**

```bash
devenv shell
```

5. **Optional: Configure `direnv` for automatic activation when entering the project directory:**

```bash
echo "use devenv" > .envrc
direnv allow
```

### General Users (Non-NixOS)

1. **Clone the repository:**

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

## Required Tools

| Tool       | Purpose                        |
| ---------- | ------------------------------ |
| `rustc`    | Rust compiler                  |
| `cargo`    | Build system and package manager |
| `clippy`   | Linter                         |
| `rustfmt`  | Code formatter                 |

## Devenv Commands (Nix / NixOS Users)

| Command                  | Description                        |
| ------------------------ | ---------------------------------- |
| `devenv shell lint`      | Lint code with clippy              |
| `devenv shell format`    | Format code with rustfmt           |
| `devenv shell typecheck` | Check code with cargo check        |
| `devenv shell test`      | Run tests with cargo test          |

## Normal Commands

| Command                              | Description                        |
| ------------------------------------ | ---------------------------------- |
| `cargo clippy -- -D warnings`        | Lint code with clippy              |
| `cargo fmt --all`                    | Format code with rustfmt           |
| `cargo check`                        | Type check with cargo check        |
| `cargo test -- --nocapture`          | Run tests with cargo test          |

## CI Pipeline

This project uses GitHub Actions for continuous integration. The pipeline is triggered on every push and pull request to `main` and runs the following steps in order:

- Linting via `devenv shell lint`
- Type checking via `devenv shell typecheck`
- Tests via `devenv shell test`

Tools used in CI: `rustc`, `cargo`, `clippy`, and `rustfmt` — all managed via `rust-overlay` through devenv.

## Notes

- `.direnv` and `.devenv` are intentionally excluded from version control.
- `devenv.lock` and `Cargo.lock` are tracked in Git to guarantee strict environment reproducibility across machines.
- `target/` is excluded from version control as it contains build artifacts.
