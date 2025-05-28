{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    nixpkgs-compat.url = "github:nixos/nixpkgs?ref=23.05";
    nixpkgs-compat.flake = false;
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs-compat, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        pkgs-compat = import nixpkgs-compat { inherit system; };
        toUpper = pkgs.lib.strings.toUpper;
        toolchain = (pkgs.rust-bin.fromRustupToolchainFile
          ./rust-toolchain.toml).override { extensions = [ "rust-analyzer" ]; };
        cargofy = s: builtins.replaceStrings [ "-" ] [ "_" ] s;
        env = config: cc: {
          nativeBuildInputs = [ cc ];
          "CC_${cargofy config}" = "${cc}/bin/${cc.targetPrefix}cc";
          "CXX_${cargofy config}" = "${cc}/bin/${cc.targetPrefix}c++";
          "CARGO_TARGET_${toUpper (cargofy config)}_LINKER" = "${cc}/bin/${cc.targetPrefix}cc";
        };
      in
      {
        formatter = pkgs.nixpkgs-fmt;
        devShells.default = with pkgs;
          mkShell (lib.zipAttrsWith
            (name: values:
              if builtins.isList (builtins.head values) then
                builtins.concatLists values
              else
                builtins.head values)
            ([
              {
                nativeBuildInputs =
                  [
                    pkg-config
                    cmake
                    rust-bindgen
                    toolchain
                  ];
              }
              (with pkgsCross.mingwW64; (env "x86_64-pc-windows-gnu" (stdenv.cc.override {
                extraBuildCommands = ''
                  echo '-L ${windows.mingw_w64_pthreads}/lib' >> $out/nix-support/cc-ldflags
                '';
              })))
              (with pkgsCross.aarch64-multiplatform-musl; env targetPlatform.config stdenv.cc)
              (with pkgsCross.musl64; env targetPlatform.config stdenv.cc)
              (with pkgs-compat.pkgsCross.aarch64-multiplatform; env targetPlatform.config stdenv.cc)
              (with pkgs-compat.pkgsCross.gnu64; env targetPlatform.config stdenv.cc)
            ] ++ lib.optionals pkgs.stdenv.isDarwin [
              (with pkgsCross.aarch64-darwin; env targetPlatform.config stdenv.cc)
              (with pkgsCross.x86_64-darwin; env targetPlatform.config stdenv.cc)
            ]));
      });
}
