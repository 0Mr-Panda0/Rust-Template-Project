{ pkgs, ... }:

{

  packages = with pkgs; [
    just
    cargo-edit
  ];

  languages.rust = {
    enable = true;

    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
    ];
  };

  scripts = {
    lint.exec = "cd hello_world  && cargo clippy";
    format.exec = "cd hello_world && cargo fmt --all";
    test.exec = "cd hello_world  && cargo test";
    build.exec = "lint && format && test";
  };
}
