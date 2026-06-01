ARG NIX_VERSION=2.32.8
FROM nixos/nix:${NIX_VERSION}

WORKDIR /flake

COPY flake.nix ./
COPY flake.lock ./

ENV NIX_CONFIG="experimental-features = nix-command flakes"

RUN nix build . --out-link /toolchain

WORKDIR /app
