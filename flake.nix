{
  description = "My Rust dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            clippy
            rustc
            cargo
            rustfmt
          ];

          shellHook = ''
            echo "Rust command-line utility versions:"
            rustc --version 			#rust compiler
            cargo --version 			#rust package manager
            rustfmt --version			#rust code formatter
            clippy-driver --version		#rust linter
          '';
        };
      }
    );
}
