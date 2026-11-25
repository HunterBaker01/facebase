# -- Rust --
FROM rust:1-bookworm AS builder
WORKDIR /build

COPY organizer/ ./organizer/
RUN cargo build --release --manifest-path organizer/Cargo.toml

# -- Python --
FROM python:3.13-slim-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends libgl1 libglib2.0-0 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /usr/local/bin/

WORKDIR /app

COPY python/pyproject.toml python/uv.lock python/.python-version ./python/
RUN uv sync --frozen --no-dev --project ./python

COPY python/embedding.py ./python/

COPY --from=builder /build/organizer/target/release/organizer /app/organizer

ENV PYTHON_DIR=/app/python \
    BIND_ADDR=0.0.0.0:3000 \
    DEEPFACE_HOME=/app/deepface \
    UV_FROZEN=1 \
    UV_PYTHON_DOWNLOADS=never

EXPOSE 3000
ENTRYPOINT ["/app/organizer"]
CMD ["serve"]
