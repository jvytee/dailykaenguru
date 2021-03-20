with import <nixpkgs> {};

mkShell {
  buildInputs = [
    rustup
    pkg-config
    openssl
  ];
}
