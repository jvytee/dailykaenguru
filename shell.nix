let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> {
    overlays = [ moz_overlay ];
    # crossSystem = { config = "aarch64-unknown-linux-gnu"; };
  };
in
  with nixpkgs;
  mkShell {
    nativeBuildInputs = with pkgsBuildHost; [
      latest.rustChannels.stable.rust
      pkgconfig
      stdenv.cc
      yaml-language-server
    ];

    buildInputs = with pkgsHostTarget; [
      cacert
      openssl
    ];

    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
    RUST_SRC_PATH = "${pkgsBuildHost.latest.rustChannels.stable.rust-src}/lib/rustlib/src/rust/library";

    RUST_LOG = "info";
    KAENGURU_DATA_PATH = "data/";
    KAENGURU_CHATS_FILE = "chats.json";
    KAENGURU_TOKEN_FIlE = "data/token";
  }
