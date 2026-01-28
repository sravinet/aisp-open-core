#!/bin/bash

# AISP Formal Verification Container Build Script
set -euo pipefail

echo "🔧 Building AISP Formal Verification in Linux container with Z3..."

# Build the container image
echo "📦 Building container image..."
podman build -t aisp-formal-verification .

# Run build and tests in container
echo "🚀 Running build and tests in container..."
podman run --rm -v "$(pwd)":/workspace:Z aisp-formal-verification

echo "✅ Container build completed successfully!"

# Optional: Run interactive shell for debugging
echo "To run interactive shell:"
echo "podman run -it --rm -v \"\$(pwd)\":/workspace:Z aisp-formal-verification bash"