{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        toUpper = pkgs.lib.strings.toUpper;
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = pkgs.lib.debug.traveVal ((crane.mkLib pkgs).overrideToolchain rust);
        cargofy = s: builtins.replaceStrings [ "-" ] [ "_" ] s;
        env = config: cc: {
          nativeBuildInputs = [ cc ];
          "CC_${cargofy config}" = "${cc.targetPrefix}cc";
          "CXX_${cargofy config}" = "${cc.targetPrefix}c++";
          "CARGO_TARGET_${toUpper (cargofy config)}_LINKER" = "${cc.targetPrefix}cc";
        };
      in
      {
        formatter = pkgs.nixpkgs-fmt;
        packages.default = pkgs.callPackage ./package.nix { inherit craneLib; };
        packages.musl64 = pkgs.pkgsCross.musl64.callPackage ./package.nix { inherit craneLib; };
        packages.gnu64 = pkgs.pkgsCross.gnu64.callPackage ./package.nix { inherit craneLib; };
        packages.apple64 = pkgs.pkgsCross.x86_64-darwin.callPackage ./package.nix { inherit craneLib; };
        devShells.default =
          with pkgs; mkShell (lib.zipAttrsWith
            (name: values: if builtins.isList (builtins.head values) then builtins.concatLists values else builtins.head values)
            [
              {
                nativeBuildInputs = [
                  pkg-config
                  cmake
                  rust-bindgen
                  rust
                  rust-analyzer
                ];
              }
              (with pkgsCross.mingwW64; env "x86_64-pc-windows-gnu" stdenv.cc)
              (with pkgsCross.x86_64-darwin; env targetPlatform.config stdenv.cc)
              (with pkgsCross.aarch64-multiplatform; env targetPlatform.config stdenv.cc)
              (with pkgsCross.aarch64-multiplatform-musl; env targetPlatform.config stdenv.cc)
              (with pkgsCross.gnu64; env targetPlatform.config stdenv.cc)
              (with pkgsCross.musl64; env targetPlatform.config stdenv.cc)
              (with pkgsCross.aarch64-darwin; env targetPlatform.config stdenv.cc)
            ]);
      });
}
