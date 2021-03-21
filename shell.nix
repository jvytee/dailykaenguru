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
}
