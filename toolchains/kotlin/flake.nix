{
  description = "Toolchain for Koltin/Java based libraries";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs = { self, nixpkgs } @inputs:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    packages = with pkgs; [
      temurin-bin-21
      maven
      gradle
      kotlin
    ];
  in {
    devShells.${system}.default = pkgs.mkShell { inherit packages; };
    packages.${system}.default = pkgs.buildEnv {
      name = "kotlin-toolchain";
      paths = packages;
    };
  };
}