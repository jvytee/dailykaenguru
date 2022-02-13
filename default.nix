{ crossSystem ? builtins.currentSystem }:

let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> {
    overlays = [ moz_overlay ];
    crossSystem = crossSystem;
  };

  latestRustPlatform = nixpkgs.makeRustPlatform {
    rustc = nixpkgs.pkgsBuildHost.latest.rustChannels.stable.rustc;
    cargo = nixpkgs.pkgsBuildHost.latest.rustChannels.stable.cargo;
  };

  dailykaenguru = with nixpkgs; latestRustPlatform.buildRustPackage {
    pname = "dailykaenguru";
    version = "0.1.0";
    doCheck = false;

    src = builtins.fetchTarball "https://github.com/jvytee/dailykaenguru/archive/main.tar.gz";
    cargoSha256 = "1dxyrziz30y9c9cp9kkc2kd62mf6ng5l26g63ml0xh1g6bmv8ndn";

    nativeBuildInputs = with pkgsBuildHost; [
      latest.rustChannels.stable.rust
      pkgconfig
      stdenv.cc
    ];

    buildInputs = with pkgsHostTarget; [
      cacert
      openssl
    ];

    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";

    meta = with lib; {
      description = "Liefert den täglichen Känguru-Comic von Zeit Online auf Telegram";
      homepage = "https://github.com/jvytee/dailykaenguru";
      license = licenses.gpl3Only;
    };
  };
in
  nixpkgs.dockerTools.buildLayeredImage {
    name = "dailykaenguru";
    tag = "latest";
    created = "now";

    contents = [
      nixpkgs.pkgsHostTarget.cacert
      dailykaenguru
    ];
    config = {
      Cmd = [ "/bin/dailykaenguru" ];
      Volumes = {
        "/data" = {};
      };
    };
  }
