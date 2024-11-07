{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            config.allowUnsupportedSystem = true;
          };
        in
        {
          formatter = pkgs.nixpkgs-fmt;
          devShells.default =
            with pkgs;
            mkShell {
              nativeBuildInputs = [
                zig
                pkg-config
                rust-bindgen
                cmake
              ];

              buildInputs = [
              ] ++ lib.optionals stdenv.isDarwin [
                iconv
                darwin.apple_sdk.frameworks.Security
                darwin.apple_sdk.frameworks.CoreFoundation
                darwin.apple_sdk.frameworks.SystemConfiguration
                darwin.apple_sdk.frameworks.CoreServices
              ];

              CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Ctarget-feature=-crt-static";
              CC_x86_64_unknown_linux_gnu =
                "${pkgs.pkgsCross.gnu64.stdenv.cc}/bin/x86_64-unknown-linux-gnu-gcc";
              CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER =
                "${pkgs.pkgsCross.gnu64.stdenv.cc}/bin/x86_64-unknown-linux-gnu-gcc";

              CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-Ctarget-feature=-crt-static";
              CC_aarch64_unknown_linux_gnu =
                "${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-gcc";
              CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =
                "${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-gcc";
            };
        });
}
