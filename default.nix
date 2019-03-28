{ pkgs ? import (builtins.fetchTarball https://github.com/nixos/nixpkgs/archive/master.tar.gz) {} }:

let
  openssl = pkgs.openssl_1_1 or pkgs.openssl_1_1_0;
in

pkgs.rustPlatform.buildRustPackage rec {
  name = "baz_out-${version}";
  version = "test";

  src = ./.;

  cargoSha256 = "1z6s284mm80g0rnl0j0pbgssrmpy6i31jbfadvz2nhy94fjlqi1r";

  # run time dependencies
  OPENSSL_DIR = openssl.dev;
  OPENSSL_LIB_DIR = "${openssl.out}/lib";
}
