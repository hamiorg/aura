#!/usr/bin/env bash
# Build Linux musl binaries using cross (Docker-based cross-compilation).
#
# Produces fully static binaries with no libc dependency — runs on
# Debian, Fedora, Alpine, Ubuntu, and any other x86_64/arm64 Linux.
#
# Usage:
#   ./dist/build/linux.sh [x86_64|arm64|all]
#
# Prerequisites:
#   cargo install cross
#   Docker daemon running
#
# Output:
#   target/x86_64-unknown-linux-musl/release/aura
#   target/aarch64-unknown-linux-musl/release/aura

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE="$(cd "${SCRIPT_DIR}/../.." && pwd)"
ARCH="${1:-all}"

# Verify cross and Docker are available before starting.
if ! command -v cross &>/dev/null; then
  echo "error: cross not found — install with: cargo install cross"
  exit 1
fi
if ! docker info &>/dev/null; then
  echo "error: Docker daemon not running"
  exit 1
fi

build() {
  local target="${1}"
  echo ""
  echo "==> Building aura for ${target}"
  cd "${WORKSPACE}"
  cross build --release --target "${target}" -p compiler
  echo "==> Built: target/${target}/release/aura"
}

case "${ARCH}" in
  x86_64)
    build "x86_64-unknown-linux-musl"
    ;;
  arm64)
    build "aarch64-unknown-linux-musl"
    ;;
  all)
    build "x86_64-unknown-linux-musl"
    build "aarch64-unknown-linux-musl"
    ;;
  *)
    echo "Usage: $0 [x86_64|arm64|all]"
    exit 1
    ;;
esac

echo ""
echo "==> Linux builds complete"
