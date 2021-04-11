with import <nixpkgs> {};

rustPlatform.buildRustPackage rec {
  pname = "dailykaenguru";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "jvytee";
    repo = pname;
    rev = "main";
    sha256 = "199my1dbl3lz7p4j2hqdibkhx9yz8b09ssyrqiiqvkdxqwzim3wx";
  };

  cargoSha256 = "1m0mrd7c2ipmnipp3gaays7gmbb0zd4byh66dpsmzs8ihgz4p306";

  buildInputs = [
    cacert
    openssl
  ];
  nativeBuildInputs = [
    pkg-config
  ];

  meta = with lib; {
    description = "Liefert den täglichen Känguru-Comic von Zeit Online auf Telegram";
    homepage = "https://github.com/jvytee/dailykaenguru";
  };
}
