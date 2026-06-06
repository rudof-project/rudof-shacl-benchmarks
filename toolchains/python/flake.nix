{
  description = "Toolchain for Python based libraries";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs = { self, nixpkgs } @inputs:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    packages = with pkgs; [
      python3
      uv
      gcc
      patchelf
      glibc
    ];
  in {
    devShells.${system}.default = pkgs.mkShell { inherit packages; };
    packages.${system}.default = pkgs.buildEnv {
      name = "python-toolchain";
      paths = packages;
    };
  };
}
