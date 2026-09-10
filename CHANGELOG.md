# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure with dual binary (`main.rs`) and library (`lib.rs`) layout.
- Devenv/Nix hermetic development environment with `rust-overlay` and `git-hooks`.
- Dual CI workflow supporting Nix and cross-platform matrix (Ubuntu, macOS, Windows).
- Minimum Supported Rust Version (MSRV) check for Rust 1.80.
- Automatic dependency updates with GitHub Dependabot.
- Developer configurations: `.editorconfig`, `.vscode/settings.json`, `.vscode/extensions.json`, and `rustfmt.toml`.
- Personalizable `Greeter` struct with trait derives (`Debug`, `Clone`, `PartialEq`, `Eq`, `Default`), doctests, and unit tests.
- CLI argument parsing via `clap` (`--name`, `--formal`, `--verbose`).
- Rich, colorful error and panic reporting via `color-eyre`.
- Strongly typed custom library errors with `thiserror` (`GreetError`).
- Structured application logging and diagnostics via `tracing` and `tracing-subscriber`.
- Interactive template instantiation support with `cargo-generate` (`cargo-generate.toml`).
- Supply chain security, license compliance, and crate ban auditing via `cargo-deny` (`deny.toml`).
- Multi-platform automated release workflow (`.github/workflows/release.yml`) for Linux, macOS, and Windows with SHA-256 checksums.
- Micro-benchmarking suite using `divan` (`benches/greeting_benchmark.rs`).
- Dynamic shell completion generation for Bash, Zsh, Fish, PowerShell, and Elvish via `clap_complete`.
- Interactive developer tooling in Devenv: `bacon` background checker, `cargo-nextest` parallel test runner, and `cargo-llvm-cov`.
- Automated code coverage workflow in GitHub Actions with artifact generation.
- Automated API documentation generation and deployment to GitHub Pages (`.github/workflows/docs.yml`).

