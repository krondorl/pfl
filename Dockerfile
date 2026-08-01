# --- Stage 1: Build ---
FROM rust:1.92-slim AS builder

WORKDIR /usr/src/pfl

# Copy the complete project from the Docker build context
COPY . .

# Build the optimized release binary
RUN cargo build --release --locked


# --- Stage 2: Runtime ---
FROM debian:bookworm-slim

# Create a non-root system user and group
RUN groupadd --system pfl \
    && useradd \
        --system \
        --gid pfl \
        --create-home \
        --home-dir /app \
        pfl

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder \
    /usr/src/pfl/target/release/pfl \
    /usr/local/bin/pfl

# Run the application as a non-root user
USER pfl

# Document the port used by the application
EXPOSE 8080

# Use an absolute path so the executable can always be found
ENTRYPOINT ["/usr/local/bin/pfl"]