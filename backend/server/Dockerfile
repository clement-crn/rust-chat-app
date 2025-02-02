# Stage 1: Build the Rust application
FROM rust:1.81-alpine as builder

# Install dependencies for building Rust applications
RUN apk add --no-cache musl-dev openssl-dev pkgconfig

# Set the working directory
WORKDIR /app

# Copy the Cargo manifest and lock file
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies first (to leverage Docker caching)
RUN cargo build --release || true

# Remove the dummy file
RUN rm -f src/main.rs

# Copy the actual source code
COPY ./src ./src

# Build the final binary
RUN cargo build --release

# Stage 2: Runtime container (lightweight)
FROM alpine:latest

# Install necessary runtime dependencies (like OpenSSL if needed)
RUN apk add --no-cache ca-certificates

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/server /usr/local/bin/server

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["server"]

