with import <nixpkgs> {};

mkShell {
  nativeBuildInputs = [
    pkgconfig
    rust-analyzer
    rustup
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
