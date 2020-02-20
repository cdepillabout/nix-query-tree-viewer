let
  nixQueryTreeViewerOverlay = self: super: {
    nixQueryTreeViewer =
      self.rustPlatform.buildRustPackage rec {
        name = "nix-query-tree-viewer-${version}";
        version = "0.1.0";

        src = self.nix-gitignore.gitignoreSource [] ./.;

        buildInputs = [
          self.glib
          self.gnome3.gtk3
        ];

        cargoSha256 = "070mvz3igc0dp5vcwqg1ri9lafcfpnwx3ifbq8k1yzkl5aj9yjky";
      };
  };

  # nixpkgs master as of 2020-01-26
  nixpkgsSrc = builtins.fetchTarball {
    url = https://github.com/NixOS/nixpkgs/archive/ba8fbd5352b8aca9410b10c8aa78e84740529e87.tar.gz;
    sha256 = "sha256:0sanh2h4h60ir6mg12m6ckqamzgnipfdkszg1kl4qv39hdmy9xzm";
  };

  nixpkgs = import nixpkgsSrc {
    overlays = [ nixQueryTreeViewerOverlay ];
  };
in

nixpkgs.nixQueryTreeViewer
