#!/bin/bash
# Test runner with suppressed unused code warnings
# Usage: ./.cargo/test-quiet.sh [cargo test arguments]

export RUSTFLAGS="-A unused-variables -A unused-imports -A dead-code -A unused-mut -A unused-assignments"
cargo test "$@"