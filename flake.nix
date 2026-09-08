{
  description = "Nix dev shell and build scripts for agi-idle (Rust Wasm + Angular)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain pinned with wasm32 compilation target
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Helper script: builds the Rust core and outputs Wasm
        buildWasm = pkgs.writeShellScriptBin "build-wasm" ''
          set -e
          echo "🦀 Building Rust core to WebAssembly..."
          # Change into core-engine only if it exists in the current directory
          if [ -d "core-engine" ]; then
            cd core-engine
          fi
          wasm-pack build --target web --out-dir ../frontend/public/wasm
          echo "✨ Wasm module generated"
        '';

        # Helper script: auto-rebuilds Wasm on any change in core-engine/src
        watchWasm = pkgs.writeShellScriptBin "watch-wasm" ''
          set -e
          echo "👀 Watching core-engine for changes..."
          build-wasm
          # Move into the cargo project before starting the watcher
          cd core-engine && cargo watch -w src -s "build-wasm"
        '';

        # Helper script: launches both Wasm auto-rebuild and Angular dev server concurrently
        devAll = pkgs.writeShellScriptBin "dev" ''
          set -e
          build-wasm
          trap 'kill 0' EXIT
          (cd core-engine && cargo watch -w src -s "build-wasm") &
          (cd frontend && npx ng serve) &
          wait
        '';

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.cargo-watch
            pkgs.wasm-pack
            pkgs.nodejs_26            
            # Helper commands
            buildWasm
            watchWasm
            devAll
          ];

          shellHook = ''
            echo "==========================================="
            echo " 🚀 agi-idle Dev Shell Ready"
            echo " Available commands:"
            echo "   • build-wasm : Compile Rust engine to Wasm"
            echo "   • watch-wasm : Watch Rust files & recompile Wasm"
            echo "   • dev        : Run Wasm watcher + Angular dev server"
            echo "==========================================="
          '';
        };
      }
    );
}