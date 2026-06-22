ARG ROOTFS=busybox

FROM rudof/rust_builder:latest AS builder

WORKDIR /app

COPY src ./src
COPY Cargo.toml Cargo.lock ./

RUN cargo zigbuild --release --target x86_64-unknown-linux-musl

FROM docker:cli AS docker_cli
FROM ${ROOTFS} AS rootfs
FROM scratch

ARG BINARY_NAME=app
ENV BINARY=/${BINARY_NAME}

COPY --from=rootfs /bin/sh /bin/sh
COPY --from=rootfs /bin/mkdir /bin/mkdir
COPY --from=rootfs /bin/chmod /bin/chmod
COPY --from=docker_cli /usr/local/bin/docker /usr/local/bin/docker
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/${BINARY_NAME} /${BINARY_NAME}

RUN mkdir -p /.cache /tmp && \
    chmod -R 777 /.cache /tmp

ENTRYPOINT [ "/bin/sh", "-c", "exec ${BINARY} \"$@\"", "--" ]
