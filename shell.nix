let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with nixpkgs;
  mkShell {
    nativeBuildInputs = with nixpkgs; [
      pkgconfig
      rust-analyzer
      nixpkgs.latest.rustChannels.stable.rust
    ];

    buildInputs = [
      cacert
      openssl
    ];
  
    RUST_LOG = "info";
    DAILYKAENGURU_DATA = "data/";
    DAILYKAENGURU_DELIVERY = "10:30";
    DAILYKAENGURU_TOKEN = "";
  }
