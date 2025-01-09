{ lib
, stdenv
, craneLib
, pkg-config
, cmake
, apple-sdk
}:
let target = lib.strings.toUpper (builtins.replaceStrings [ "-" ] [ "_" ] stdenv.targetPlatform.config);
in
craneLib.buildPackage {
  src = lib.fileset.toSource rec {
    root = ./.;
    fileset = lib.fileset.unions [
      (craneLib.fileset.commonCargoSources root)
      (lib.fileset.fileFilter (file: file.hasExt "h") root)
    ];
  };

  strictDeps = true;
  doCheck = false;

  nativeBuildInputs = [
    pkg-config
    cmake
    stdenv.cc
  ];

  nativeInput = lib.optionals stdenv.isDarwin [ apple-sdk ];

  CARGO_BUILD_TARGET = stdenv.targetPlatform.config;

  "CARGO_TARGET_${target}_LINKER" = "${stdenv.cc.targetPrefix}cc";

  RUSTFLAGS = "-C target-feature=-crt-static";
  cargoExtraArgs = "--features encryption";

  HOST_CC = "${stdenv.cc.nativePrefix}cc";
  TARGET_CC = "${stdenv.cc.targetPrefix}cc";
}
