let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> {
    overlays = [ moz_overlay ];
    #crossSystem = { config = "aarch64-unknown-linux-gnu"; };
  };
in
  with nixpkgs;
  mkShell {
    nativeBuildInputs = with pkgsBuildHost; [
      latest.rustChannels.stable.rust
      pkgconfig
      rust-analyzer
      stdenv.cc
    ];

    buildInputs = with pkgsHostTarget; [
      cacert
      openssl
    ];

    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";

    RUST_LOG = "info";
    DAILYKAENGURU_DATA = "data/";
    DAILYKAENGURU_DELIVERY = "10:30";
    DAILYKAENGURU_TOKEN = "";
  }
