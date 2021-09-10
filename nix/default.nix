
let
  # nixpkgs master as of 2020-01-26
  nixpkgsSrc = builtins.fetchTarball {
    url = https://github.com/NixOS/nixpkgs/archive/1e5c35dfbc8e0ad8c35aa1a6446f442ca1ec5234.tar.gz;
    sha256 = "0q7q7f7qhcag6mm3sp10gla5k0f9z1b68h04navrfd4dbz8zza0c";
  };

  overlay = import ./overlay.nix;

  nixpkgs = import nixpkgsSrc {
    overlays = [ overlay ];
  };
in
nixpkgs
