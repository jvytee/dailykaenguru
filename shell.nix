with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "dailykaenguru-env";
  nativeBuildInputs = [
    rustup
    pkg-config
  ];
  buildInputs = [
    openssl
  ];
}
