{
  description = "Toolchain for dataset building";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs = { self, nixpkgs } @inputs:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    packages = with pkgs; [
      temurin-bin-21
      ant
      gzip
      librdf_raptor2
      bash
      perl
      curl
      gnused
      xz
      libuuid
    ];
  in {
    devShells.${system}.default = pkgs.mkShell { inherit packages; };
    packages.${system}.default = pkgs.buildEnv {
      name = "dataset-toolchain";
      paths = packages;
    };
  };
}
