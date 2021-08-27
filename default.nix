{ pkgs ? import <nixpkgs> { } }:
with pkgs;

rustPlatform.buildRustPackage rec {
  name = "rustybar";
  version = "0.1";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  RUSTC_BOOTSTRAP = 1;
  doCheck = false;
  nativeBuildInputs = [ pkgconfig ];
  buildInputs = [ dbus ];

  meta = with lib; {
    homepage = "https://github.com/mildlyfunctionalgays/rustybar";
    description = "swaybar/i3bar command in Rust";
    maintainers = with maintainers; [ artemist ];
    license = with licenses; [ mit ];
    platforms = platforms.linux;
  };
}

