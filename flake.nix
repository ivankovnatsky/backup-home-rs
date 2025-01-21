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

          # Define the package
          backup-home = pkgs.rustPlatform.buildRustPackage {
            pname = "backup-home";
            version = "0.1.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = [ 
              pkgs.cmake 
              pkgs.go
            ];
            
            buildInputs = [ pkgs.openssl.dev ];

            preBuildPhases = ["goPathSetup"];
            goPathSetup = ''
              export GOCACHE=$TMPDIR/go-cache
              export GOPATH=$TMPDIR/go
              mkdir -p $GOCACHE $GOPATH
            '';

            meta = with pkgs.lib; {
              description = "A tool for creating and uploading backups of user directories";
              homepage = "https://github.com/ivankovnatsky/backup-home";
              license = licenses.mit; # Adjust according to your license
            };
          };

        in
        {
          # Expose the package
          packages.default = backup-home;
          packages.backup-home = backup-home;

          # Keep the development shell
          devShells.default = pkgs.mkShell {
            buildInputs = [
              rust
              pkgs.rust-analyzer
              pkgs.cmake
            ];

            shellHook = ''
              
            '';
          };
        }
      );
}
