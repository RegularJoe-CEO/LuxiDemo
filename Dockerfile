# -------- Base build stage (builds release binary) --------
FROM rust:1.89-bullseye AS builder
WORKDIR /app
# Copy all sources (including benches and docs)
COPY . .
# Build release binary (workspace crate 'erock_edge')
RUN cargo build --release -p erock_edge

# -------- Optional benchmark stage (keeps toolchain & deps) --------
FROM rust:1.89-bullseye AS bench
WORKDIR /app
COPY . .
RUN cargo fetch
# User can run: docker run --rm -it luxi-edge-bench bash -c 'cargo bench --bench edge_suite'

# -------- Slim runtime stage --------
FROM debian:bookworm-slim AS runtime
RUN useradd -m -u 10001 appuser \
    && apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
# Copy only the binary + benchmark summary docs
COPY --from=builder /app/target/release/erock_edge /usr/local/bin/erock_edge
COPY docs/benchmarks/BENCHMARK_DATA.md docs/benchmarks/COMPARATIVE_ANALYSIS.md docs/benchmarks/README.md ./docs/benchmarks/
ENV RUST_LOG=info
EXPOSE 8080
USER appuser
CMD ["erock_edge"]
