{
  description = "sway/i3bar command in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    let
      supportedSystems = nixpkgs.lib.platforms.unix;
    in
    utils.lib.eachSystem supportedSystems (system:
      let pkgs = import nixpkgs { inherit system; };
      in
      rec {
        packages.rustybar = with pkgs; rustPlatform.buildRustPackage rec {
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
            platforms = supportedSystems;
          };
        };
        defaultPackage = packages.rustybar;

        apps.rustybar = utils.lib.mkApp { drv = packages.rustybar; };
        defaultApp = apps.rustybar;

        devShells.rustybar = with pkgs; mkShell {
          packages = [ pkgconfig dbus rustup ];
        };
        devShell = devShells.rustybar;
      });
}
