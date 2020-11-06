{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    pkg-config
  ];
}
