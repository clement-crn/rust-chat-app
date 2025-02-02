# Use the official Rust image as a build stage
FROM rust:1.66 as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the project in release mode
RUN cargo build --release

# Use a minimal image to run the binary
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Copy the compiled Rust binary from the builder stage
COPY --from=builder /app/target/release/hello_world /usr/local/bin/hello_world

# Set the entrypoint to run the binary
ENTRYPOINT ["/usr/local/bin/hello_world"]

