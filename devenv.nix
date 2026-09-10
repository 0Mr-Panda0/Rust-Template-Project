{ pkgs, ... }:

{
  packages = with pkgs; [
    cargo-deny
    cargo-nextest
    cargo-llvm-cov
    bacon
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };

  scripts = {
    lint.exec = "cargo clippy -- -D warnings";
    format.exec = "cargo fmt --all";
    format-check.exec = "cargo fmt --all -- --check";
    typecheck.exec = "cargo check";
    unit-test.exec = "cargo test -- --nocapture";
    audit.exec = "cargo deny check";
    bench.exec = "cargo bench";
    coverage.exec = "cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info";
    nextest.exec = "cargo nextest run";
    watch.exec = "bacon";
  };

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
