
final: prev: {
  nix-query-tree-viewer =
    final.rustPlatform.buildRustPackage rec {
      name = "nix-query-tree-viewer-${version}";
      version = "0.2.1";

      src = final.nix-gitignore.gitignoreSource [] ./..;

      buildInputs = [
        final.glib
        final.gtk3
      ];

      cargoSha256 = "sha256-gD9lWnf6i7YyulbMOnPrrz1/c4bUMgoSY3bxIy+U+98=";
    };

  nix-query-tree-viewer-shell = final.stdenv.mkDerivation {
    name = "nix-query-tree-viewer-rust-env";
    nativeBuildInputs = [
      # Things like cargo, rustc, rustfmt, and clippy can be installed with commands like
      #
      # $ rustup component add clippy
      final.rustup

      # Some rust libraries use pkgconfig.
      final.pkgconfig

      # For creating the UI.
      final.gnome3.glade
    ];
    buildInputs = [
      final.openssl

      final.glib
      final.gtk3
    ];

    shellHook = ''
      # TODO: This clobbers MANPATH if it is already set.
      # export MANPATH=":${final.xorg.libxcb.man}/share/man"
    '';

    # Set Environment Variables
    #RUST_BACKTRACE = 1;
  };
}
