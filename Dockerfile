# Multi stage build for stomata to keep image size low
FROM rust:1.90-slim-bookworm@sha256:64232e656c058f4468e8d024e990acff04f0fd5a5c0a88a574dc37773d7325c9 AS builder

# Installing dependencies and musl dependencies to avoid glibc issues
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new working dir for stomata
WORKDIR /usr/src/stomata

# Copy entire workspace
COPY . .

# Build release binary with musl target for static linking
RUN cargo build --release --bin stomata

# 2nd stage: Creating a minimal runime image
FROM debian:bookworm-slim

# Install runtime dependencies for TUI
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy statically linked binary from builder
COPY --from=builder /usr/src/stomata/target/release/stomata /usr/local/bin/stomata

# Set the entrypoint
ENTRYPOINT ["stomata"]

# Default command
CMD ["--help"]