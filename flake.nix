{
  description = "haskell env";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in
        {
          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              rustc
              cargo
              rust-analyzer
              cargo-watch
              clippy
              rustfmt
            ];
            shellHook = ''
            '';
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        }
      );
}

