with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "dailykaenguru-env";
  nativeBuildInputs = [
    rust-analyzer
    rustup
    pkgconfig
    openssl
  ];
  buildInputs = [
    openssl
  ];

  RUST_LOG = "info";
  DAILYKAENGURU_DATA = "data/";
  DAILYKAENGURU_TOKEN = "";
  DAILYKAENGURU_DELIVERY = "10:30";
}
