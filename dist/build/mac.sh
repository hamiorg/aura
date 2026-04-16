#!/usr/bin/env bash
# Build macOS binaries using the native Rust toolchain.
#
# MUST run on macOS. For CI/CD, this script runs on GitHub's macOS runners.
#
# Usage:
#   ./dist/build/mac.sh [x86_64|arm64|all]
#
# Prerequisites:
#   rustup (installed automatically if missing)
#   Xcode Command Line Tools
#
# Output:
#   target/aarch64-apple-darwin/release/aura  (arm64 / Apple Silicon)
#   target/x86_64-apple-darwin/release/aura   (x86_64 / Intel)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE="$(cd "${SCRIPT_DIR}/../.." && pwd)"
ARCH="${1:-all}"

if [[ "$(uname)" != "Darwin" ]]; then
  echo "error: mac.sh must run on macOS (current OS: $(uname))"
  exit 1
fi

if ! command -v cargo &>/dev/null; then
  echo "error: Rust not found — install from https://rustup.rs"
  exit 1
fi

build() {
  local target="${1}"
  echo ""
  echo "==> Adding Rust target ${target}"
  rustup target add "${target}"
  echo "==> Building aura for ${target}"
  cd "${WORKSPACE}"
  cargo build --release --target "${target}" -p compiler
  echo "==> Built: target/${target}/release/aura"
}

case "${ARCH}" in
  arm64)
    build "aarch64-apple-darwin"
    ;;
  x86_64)
    build "x86_64-apple-darwin"
    ;;
  all)
    build "aarch64-apple-darwin"
    build "x86_64-apple-darwin"
    ;;
  *)
    echo "Usage: $0 [x86_64|arm64|all]"
    exit 1
    ;;
esac

echo ""
echo "==> macOS builds complete"
