# ---------- builder ----------
FROM rust:1.84-slim AS builder
WORKDIR /src
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates pkg-config build-essential && rm -rf /var/lib/apt/lists/*

# Copy manifests first (better caching)
COPY Cargo.toml Cargo.toml
COPY edge/Cargo.toml edge/Cargo.toml
COPY edge/Cargo.lock edge/Cargo.lock

# Copy sources
COPY src src
COPY benches benches
COPY edge edge

# Build only the edge binary
RUN cargo build --release --manifest-path edge/Cargo.toml

# ---------- runtime ----------
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /src/edge/target/release/erock_edge /usr/local/bin/erock_edge
EXPOSE 8080
LABEL org.opencontainers.image.title="Luxi Edge" \
      org.opencontainers.image.description="Ultra-fast, energy-efficient numeric microservice (Rust/Axum)" \
      org.opencontainers.image.licenses="LicenseRef-Luxi-Business-1.0"
ENTRYPOINT ["erock_edge"]
