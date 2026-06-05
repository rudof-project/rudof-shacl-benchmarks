{
  description = "Toolchain for Rust based libraries";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix } @inputs:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ fenix.overlays.default ];
    };
    rustToolchain = with pkgs.fenix; combine [
      stable.cargo
      stable.rustc
      targets.x86_64-unknown-linux-musl.stable.rust-std
    ];
    packages = with pkgs; [
      rustToolchain
      openssl
      gcc
      bash
      docker
      zig
      cargo-zigbuild
    ];
  in {
    devShells.${system}.default = pkgs.mkShell { inherit packages; };
    packages.${system}.default = pkgs.buildEnv {
      name = "rust-toolchain";
      paths = packages;
    };
  };
}
