# QRAP Demo Dockerfile
# Multi-stage build for Rust-based quantum runtime attestation protocol

# Stage 1: Build stage
FROM rust:1.75-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the entire project
COPY . .

# Build the core library
RUN cargo build --release --package qrap-core

# Build the WASM target (optional)
# RUN rustup target add wasm32-unknown-unknown
# RUN cargo build --release --target wasm32-unknown-unknown --package qrap-wasm-visualizer

# Stage 2: Python bindings build
FROM rust:1.75-slim-bookworm AS python-builder

# Install Python and maturin
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    python3-venv \
    && rm -rf /var/lib/apt/lists/*

# Install maturin
RUN pip3 install maturin --break-system-packages

WORKDIR /app
COPY . .

# Build Python bindings
RUN cd python && maturin build --release

# Stage 3: Runtime image
FROM python:3.11-slim-bookworm AS runtime

# Install Jupyter and other Python dependencies
RUN pip install --no-cache-dir \
    jupyter \
    jupyterlab \
    matplotlib \
    numpy \
    && rm -rf /root/.cache

# Create app user
RUN useradd -m -s /bin/bash qrap

# Create directories
WORKDIR /home/qrap/app
RUN mkdir -p notebooks baseline

# Copy notebooks
COPY --chown=qrap:qrap notebooks/ notebooks/

# Copy baseline data
COPY --chown=qrap:qrap baseline/ baseline/

# Copy entrypoint
COPY --chown=qrap:qrap docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Copy Python wheel from builder (if built)
# COPY --from=python-builder /app/python/target/wheels/*.whl /tmp/
# RUN pip install /tmp/*.whl && rm /tmp/*.whl

# Switch to app user
USER qrap

# Expose Jupyter port
EXPOSE 8888

# Set environment
ENV PYTHONUNBUFFERED=1
ENV JUPYTER_TOKEN=qrap-demo

ENTRYPOINT ["/entrypoint.sh"]
CMD ["jupyter", "lab", "--ip=0.0.0.0", "--port=8888", "--no-browser"]
