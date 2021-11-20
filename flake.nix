{
  description = "sway/i3bar command in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
    in
    (utils.lib.eachSystem supportedSystems (system:
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

        overlay = final: prev: {
          inherit (packages) rustybar;
        };

        devShells.rustybar = with pkgs; mkShell {
          packages = [ pkgconfig dbus rustup ];
        };
        devShell = devShells.rustybar;
      })) // {
      overlay = final: prev: {
        inherit (self.packages."${prev.system}") rustybar;
      };
    };
}
