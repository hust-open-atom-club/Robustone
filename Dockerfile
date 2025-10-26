# Multi-stage build for Robustone CLI
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    python3 \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY robustone-core ./robustone-core/
COPY robustone-cli ./robustone-cli/
COPY robustone ./robustone/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false robustone

# Copy the binary from builder stage
COPY --from=builder /app/target/release/robustone-cli /usr/local/bin/robustone-cli

# Set ownership
RUN chown robustone:robustone /usr/local/bin/robustone-cli

# Switch to non-root user
USER robustone

# Set entrypoint
ENTRYPOINT ["robustone-cli"]
CMD ["--help"]