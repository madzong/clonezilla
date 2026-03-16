{
  description = "Example description";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    oxalica.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, oxalica }:
    with flake-utils.lib;
    eachSystem allSystems (system:
      let
        pkgs = nixpkgs.legacyPackages.${system}.extend oxalica.overlays.default;
      in rec {
        packages = {
          example-program = let
            rustPlatform = pkgs.makeRustPlatform {
              cargo = pkgs.rust-bin.stable.latest.minimal;
              rustc = pkgs.rust-bin.stable.latest.minimal;
            };
          in rustPlatform.buildRustPackage rec {
            name = "example-program";
            src = self;
            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = with pkgs; [
                alsa-lib.dev
                udev.dev
                wayland
                libxkbcommon
                vulkan-loader
            ];
            cargoLock = { lockFile = ./Cargo.lock; };
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          };
        };
        defaultPackage = packages.example-program;
        formatter = pkgs.nixfmt;
      });
}
