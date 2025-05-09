{
  description = "Flake for running rust script with Numpy, pandas, etc";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.zlib
          pkgs.glibc
          pkgs.gcc.cc.lib
          pkgs.pkgs.libffi
          pkgs.stdenv.cc.cc.lib
        ];
      };
    };
}
