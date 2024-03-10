{
  description = "A tree viewer for nix query output";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          name = "nix-query-tree-viewer-${version}";
          version = "0.2.1";
          src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;
          buildInputs = [ pkgs.glib pkgs.gtk3 ];
          cargoSha256 = "sha256-NSLBIvgo5EdCvZq52d+UbAa7K4uOST++2zbhO9DW38E=";
        };
      });
}
