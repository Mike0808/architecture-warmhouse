# Use the official Rust image as a build environment
FROM rust:1.70 as builder

# Create a new empty shell project
WORKDIR /app
RUN USER=root cargo new --bin temperature-api
WORKDIR /app/temperature-api

# Copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/temperature_api*
RUN cargo build --release

# Our final base
FROM debian:bullseye-slim

# Install OpenSSL - required by actix-web
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the build stage
COPY --from=builder /app/temperature-api/target/release/temperature-api .

# Set the startup command to run your binary
CMD ["./temperature-api"]