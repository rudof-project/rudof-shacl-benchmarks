ARG NIX_VERSION=2.32.8
FROM nixos/nix:${NIX_VERSION}

WORKDIR /flake

COPY flake.nix flake.lock ./

ENV NIX_CONFIG="experimental-features = nix-command flakes"

RUN nix build . --out-link /toolchain

ENV PATH="/toolchain/bin:$PATH"

RUN mkdir -p /.cache && \
    chmod -R 777 /.cache

WORKDIR /app
