# Multi stage build for stomata to keep image size low
FROM rust:1.90-slim AS builder

# Installing dependencies and musl dependencies to avoid glibc issues
RUN apt-get update && apt-get install -y \
    musl-tools \
    musl-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Add musl target
RUN rustup target add x86_64-unknown-linux-musl

# Create a new working dir for stomata
WORKDIR /usr/src/stomata

# Copy entire workspace
COPY . .

# Build release binary with musl target for static linking
RUN cargo build --release --target x86_64-unknown-linux-musl --bin stomata

# 2nd stage: Creating a minimal runime image
FROM debian:bookworm-slim

# Install runtime dependencies for TUI
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy statically linked binary from builder
COPY --from=builder /usr/src/stomata/target/x86_64-unknown-linux-musl/release/stomata /usr/local/bin/stomata

# Set the entrypoint
ENTRYPOINT ["stomata"]

# Default command
CMD ["--help"]