{ pkgs ? import <nixpkgs> {} }:

with pkgs; stdenv.mkDerivation {
  name = "rustybar-env";
  nativeBuildInputs = [ pkgconfig ];
  buildInputs = [ dbus rustup ];
}

