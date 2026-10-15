# Multi-stage build for Robustone CLI
FROM rust:1.99.0-slim-bookworm AS builder

# Set working directory
WORKDIR /app

# Cargo resolves every workspace member, including the architecture backends.
# Keep Cargo.lock in the build context so dependency versions stay locked.
COPY . .

# Build the application
RUN cargo build --locked --release -p robustone --bin robustone

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd --system --uid 10001 --no-create-home --shell /usr/sbin/nologin robustone

# Copy the binary from builder stage
COPY --from=builder --chown=robustone:robustone /app/target/release/robustone /usr/local/bin/robustone
COPY --from=builder /app/LICENSE /usr/share/doc/robustone/LICENSE

# Switch to non-root user
USER robustone

# Set entrypoint
ENTRYPOINT ["robustone"]
CMD ["--help"]
