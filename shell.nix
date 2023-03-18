{ pkgs ? import <nixpkgs> { } }:
let
  wasmedge = pkgs.callPackage ./default.nix {};
in
with pkgs;
mkShell {
  buildInputs = [
    pkg-config
    openssl

    nodejs

    proxychains

    wasmedge
  ];

  WASMEDGE_INCLUDE_DIR="${wasmedge}/include";
  WASMEDGE_LIB_DIR="${wasmedge}/lib";
  WASMEDGE_PLUGIN_PATH="${wasmedge}/lib/wasmedge";
}
