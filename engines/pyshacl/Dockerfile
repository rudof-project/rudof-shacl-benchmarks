ARG ROOTFS=busybox

FROM rudof/python_builder:latest AS builder

WORKDIR /app

COPY wheels ./wheels/
COPY .python-version main.py pyproject.toml uv.lock ./

ENV UV_PYTHON_DOWNLOADS=never
ENV UV_PYTHON=/toolchain/bin/python3
ENV CC=gcc
ENV CXX=g++

RUN uv sync --no-cache

RUN uv run python -m nuitka \
    --standalone \
    --static-libpython=yes \
    --assume-yes-for-downloads \
    --include-package=importlib.metadata \
    --include-package=importlib.resources \
    --include-package=rdflib \
    --include-package=pyshacl \
    --include-package-data=rdflib \
    --include-package-data=pyshacl \
    --follow-import-to=importlib \
    --follow-import-to=importlib.metadata \
    --output-filename=binary \
    --output-dir=/app/out \
    main.py

RUN find /app/out/main.dist -type f | while read -r f; do \
        patchelf --remove-rpath "$f" 2>/dev/null || true; \
        patchelf --set-rpath '$ORIGIN' "$f" 2>/dev/null || true; \
        if patchelf --print-interpreter "$f" >/dev/null 2>&1; then \
            patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 "$f"; \
        fi; \
    done

FROM ${ROOTFS} AS rootfs
FROM gcr.io/distroless/cc-debian13

ARG BINARY_NAME=app
ENV BINARY=/${BINARY_NAME}

COPY --from=rootfs /bin/sh /bin/sh
COPY --from=rootfs /bin/ln /bin/ln

COPY --from=builder /app/out/main.dist /opt/app

RUN ln -s /opt/app/binary ${BINARY}

ENTRYPOINT [ "/bin/sh", "-c", "exec ${BINARY} \"$@\"", "--" ]
