# Stage 1: cache all Cargo dependencies using a stub main
FROM rust:1-bookworm AS deps
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main(){}' > src/main.rs
RUN cargo build --release --locked
RUN rm -f target/release/deps/server* target/release/server

# Stage 2: build real source on top of the cached dep layer
FROM deps AS builder
COPY src ./src
COPY migrations ./migrations
COPY static ./static
RUN cargo build --release --locked

# Stage 3: minimal runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /build/target/release/server ./server
COPY config.json .
COPY migrations ./migrations

RUN useradd -r -s /bin/false appuser
USER appuser

HEALTHCHECK --interval=10s --timeout=5s --start-period=15s --retries=3 \
    CMD ["/app/server", "healthcheck"]

ENTRYPOINT ["/app/server"]
