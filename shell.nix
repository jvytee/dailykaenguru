with import <nixpkgs> {};

mkShell {
  nativeBuildInputs = [
    rustc
    cargo
    pkgconfig
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
