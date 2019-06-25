let
  defaultPkgs = import <nixpkgs> {};
in

{
  openssl ? defaultPkgs.openssl,
  pkg-config ? defaultPkgs.pkg-config,
  rustPlatform ? defaultPkgs.rustPlatform
}:

rustPlatform.buildRustPackage rec {
  name = "baz_out-${version}";
  version = "unstable";

  src = ./.;

  cargoSha256 = "14hdgqnii6di38a0hsmjqhqj5q9wps9fdh8phm0hqgsf5knglvaq";

  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    openssl
  ];
}
