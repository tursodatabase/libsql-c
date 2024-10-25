{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        zcc = target: pkgs.writeShellScriptBin "zcc" ''
          filtered_args=()
          for arg in "$@"; do
            if [[ $arg != "--target="* ]]; then
                filtered_args+=("$arg")
            fi
          done
          ${pkgs.zig}/bin/zig cc -target ${target} "''${filtered_args[@]}"
        '';
      in
      {
        formatter = pkgs.nixpkgs-fmt;
        devShells.default =
          with pkgs;
          mkShell {
            nativeBuildInputs = [
              pkg-config
            ];

            buildInputs = [
              cmake
            ] ++ lib.optionals stdenv.isDarwin [
              iconv
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.CoreFoundation
              darwin.apple_sdk.frameworks.SystemConfiguration
              darwin.apple_sdk.frameworks.CoreServices
            ];

            CC_aarch64_unknown_linux_gnu = "${zcc "aarch64-linux-gnu"}/bin/zcc";
          };
      });
}
