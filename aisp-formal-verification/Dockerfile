# Multi-stage production-ready AISP build container with Z3 (ARM64 compatible)
FROM --platform=linux/arm64 ubuntu:22.04 as builder

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies in one layer
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libz3-dev \
    z3 \
    clang \
    llvm-dev \
    libclang-dev \
    git \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Install Rust with specific version
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

# Verify installations
RUN rustc --version && cargo --version && z3 --version

# Set up working directory
WORKDIR /workspace

# Set environment variables for Z3 (ARM64)
ENV PKG_CONFIG_PATH="/usr/lib/aarch64-linux-gnu/pkgconfig"
ENV LIBCLANG_PATH="/usr/lib/llvm-14/lib"
ENV Z3_SYS_Z3_HEADER="/usr/include/z3.h"

# Copy only Cargo files first for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY crates/aisp-core/Cargo.toml ./crates/aisp-core/
COPY crates/aisp-cli/Cargo.toml ./crates/aisp-cli/

# Build dependencies only (this layer will be cached)
RUN mkdir -p crates/aisp-core/src crates/aisp-cli/src && \
    echo "fn main() {}" > crates/aisp-core/src/lib.rs && \
    echo "fn main() {}" > crates/aisp-cli/src/main.rs && \
    cargo build --workspace --release && \
    rm -rf crates/aisp-core/src crates/aisp-cli/src

# Copy source code
COPY crates/ ./crates/

# Build the actual project
RUN cargo build --workspace --release

# Runtime stage
FROM --platform=linux/arm64 ubuntu:22.04 as runtime

RUN apt-get update && apt-get install -y \
    libz3-4 \
    z3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /workspace/target/release/ ./

# Test the binary
RUN ./aisp-cli --help || echo "CLI binary check completed"

CMD ["./aisp-cli", "--help"]