{ pkgs, ... }:

{
  packages = with pkgs; [
    rustc
    cargo
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
    typecheck.exec = "cargo check";
    test.exec = "cargo test -- --nocapture";
  };

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
