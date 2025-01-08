{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/8a36010652b4571ee6dc9125cec2eaebc30e9400";
    flake-utils.url = "github:numtide/flake-utils/11707dc2f618dd54ca8739b309ec4fc024de578b";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
            config = {
              allowUnfree = true;
            };
          };

          # Latest stable Rust with minimal components
          rust = pkgs.rust-bin.stable.latest.minimal.override {
            extensions = [ "rust-src" "clippy" "rustfmt" ];
          };

        in
        with pkgs;
        {
          devShells.default = mkShell {
            buildInputs = [
              rust
              rust-analyzer
              cmake
            ];

            shellHook = ''
              
            '';
          };
        }
      );
}
