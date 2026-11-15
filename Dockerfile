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
# COPY keeps the build context's mtimes, which are older than the stub build just
# above, so cargo would call the real sources fresh and ship the stub. Touch them.
RUN find src -name '*.rs' -exec touch {} +
COPY migrations ./migrations
COPY static ./static
RUN cargo build --release --locked

# Stage 3: minimal runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /build/target/release/server ./server
RUN test "$(stat -c %s /app/server)" -gt 1000000 || { echo "/app/server is stub-sized: the dependency-cache stub was shipped" >&2; exit 1; }
COPY config.json .
COPY migrations ./migrations

RUN useradd -r -s /bin/false appuser
USER appuser

HEALTHCHECK --interval=10s --timeout=5s --start-period=15s --retries=3 \
    CMD ["/app/server", "healthcheck"]

ENTRYPOINT ["/app/server"]
