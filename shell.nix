with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "koalabot-env";
  nativeBuildInputs = [
    rustup
    pkg-config
  ];
  buildInputs = [
    openssl
  ];
}
