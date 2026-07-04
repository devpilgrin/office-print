# Build office-print CLI binary in a slim Docker image.
# WASM build is optional — only for publishing to npm.

FROM rust:1.96-slim-bookworm AS builder

# Build the CLI binary
WORKDIR /app
COPY . .
RUN cargo build --release -p office-print-cli

# Runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/office-print /usr/local/bin/office-print
ENTRYPOINT ["office-print"]
