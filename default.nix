let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> {
    overlays = [
      moz_overlay
    ];
    crossSystem = {
      config = "aarch64-unknown-linux-gnu";
    };
  };
  latestRustPlatform = nixpkgs.makeRustPlatform {
    rustc = nixpkgs.pkgsBuildHost.latest.rustChannels.stable.rustc;
    cargo = nixpkgs.pkgsBuildHost.latest.rustChannels.stable.cargo;
  };
in
  with nixpkgs;
  latestRustPlatform.buildRustPackage {
    pname = "dailykaenguru";
    version = "0.1.0";
    doCheck = false;
    cargoSha256 = "1dxyrziz30y9c9cp9kkc2kd62mf6ng5l26g63ml0xh1g6bmv8ndn";

    src = fetchFromGitHub {
      owner = "jvytee";
      repo = "dailykaenguru";
      rev = "main";
      sha256 = "0ilwspfvv4sczgymm0f99lkr0ndx58vqx9qhdf5xy6qay1kd833n";

    };

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
  }
