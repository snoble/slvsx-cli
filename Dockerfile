# Use official Rust image based on Debian
# Using latest stable Rust for lock file v4 compatibility
FROM rust:1.81-bookworm

# Install build dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    build-essential \
    libpng-dev \
    zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the entire project
COPY . .

# Build libslvs-static first
RUN cd libslvs-static && \
    mkdir -p build && \
    cd build && \
    cmake .. -DCMAKE_BUILD_TYPE=Release && \
    make -j$(nproc)

# Set environment variables for Rust build
ENV SLVS_LIB_DIR=/app/libslvs-static/build
ENV SLVS_STATIC=1

# Build and test the project
RUN cargo build --release
RUN cargo test

# The binary will be at /app/target/release/slvsx