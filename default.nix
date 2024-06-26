{pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  packages = with pkgs; [
    pkg-config
    libGLU
    libGL
    libxkbcommon
    wayland
  ];
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    meta = {
      description = manifest.description ? null;
    };
  }

