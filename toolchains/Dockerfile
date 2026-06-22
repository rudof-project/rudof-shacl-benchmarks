ARG NIX_VERSION=2.32.8
FROM nixos/nix:${NIX_VERSION}

WORKDIR /flake

COPY flake.nix flake.lock ./

ENV NIX_CONFIG="experimental-features = nix-command flakes"

RUN nix build . --out-link /toolchain

ENV PATH="/toolchain/bin:$PATH"

ENV CARGO_HOME=/cargo
ENV RUSTUP_HOME=/rustup

RUN mkdir -p /cargo /rustup /.cache && \
    chmod -R 777 /cargo /rustup /.cache

WORKDIR /app
