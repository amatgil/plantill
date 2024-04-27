{
  pkgs ? import <nixpkgs> { },
  lib,
}:
let
  packages = with pkgs; [
    rust-analyzer
    rustfmt
    clippy
    clang
    mold

    wasm-pack
    wasm-bindgen-cli
  ];
in
pkgs.mkShell {
  # Get dependencies from the main package
  #inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  nativeBuildInputs = packages;
  buildInputs = packages;
  env = {
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    LD_LIBRARY_PATH = "${lib.makeLibraryPath packages}";
  };
}
