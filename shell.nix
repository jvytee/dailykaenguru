with import <nixpkgs> {};

mkShell {
  nativeBuildInputs = [
    latest.rustChannels.stable.rust
    pkgconfig
    rust-analyzer
  ];
  buildInputs = [
    cacert
    openssl
  ];

  RUST_LOG = "info";
  DAILYKAENGURU_DATA = "data/";
  DAILYKAENGURU_TOKEN = "";
  DAILYKAENGURU_DELIVERY = "10:30";
}
