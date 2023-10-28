# Dockerfile for Rust Actix Web Service

# Set the base image
FROM rust:alpine3.18

# Set environment variables
ENV PORT=8080

# Install dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config

# Copy source code
COPY . /usr/src/api

# Set working directory
WORKDIR /usr/src/api

# Build the application
RUN cargo build --release

# Expose port
EXPOSE $PORT

# Run the application
CMD ["cargo", "run", "--release"]
