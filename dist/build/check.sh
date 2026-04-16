#!/usr/bin/env bash
# Check that all distribution prerequisites are installed before building.
#
# Usage:
#   ./dist/build/check.sh
#
# Run this before attempting a local release build to catch missing tools early.

set -euo pipefail

PASS=true

ok()   { printf "  [ok]   %s\n" "${1}"; }
miss() { printf "  [miss] %s  --  install: %s\n" "${1}" "${2}"; PASS=false; }
skip() { printf "  [skip] %s  --  %s\n" "${1}" "${2}"; }

echo ""
echo "=== AURA distribution prerequisite check ==="
echo ""

# Core build tools
echo "-- Build tools --"
if command -v cargo &>/dev/null; then
  ok "Rust / cargo  ($(cargo --version))"
else
  miss "Rust / cargo" "curl https://sh.rustup.rs -sSf | sh"
fi

if command -v cross &>/dev/null; then
  ok "cross  ($(cross --version 2>/dev/null | head -1))"
else
  miss "cross" "cargo install cross"
fi

if docker info &>/dev/null 2>&1; then
  ok "Docker  ($(docker --version))"
else
  miss "Docker daemon" "https://docs.docker.com/get-docker/"
fi

echo ""
echo "-- Packaging tools --"
if command -v cargo-deb &>/dev/null; then
  ok "cargo-deb"
else
  miss "cargo-deb" "cargo install cargo-deb"
fi

if command -v cargo-generate-rpm &>/dev/null; then
  ok "cargo-generate-rpm"
else
  miss "cargo-generate-rpm" "cargo install cargo-generate-rpm"
fi

echo ""
echo "-- Utility tools --"
if command -v jq &>/dev/null; then
  ok "jq  ($(jq --version))"
else
  miss "jq" "apt install jq  /  brew install jq"
fi

if command -v curl &>/dev/null; then
  ok "curl  ($(curl --version | head -1))"
else
  miss "curl" "apt install curl  /  brew install curl"
fi

# macOS-specific: only needed when building on macOS
echo ""
echo "-- macOS targets (only needed when building on macOS) --"
if [[ "$(uname)" == "Darwin" ]]; then
  if rustup target list --installed 2>/dev/null | grep -q "aarch64-apple-darwin"; then
    ok "rustup target: aarch64-apple-darwin"
  else
    miss "rustup target: aarch64-apple-darwin" "rustup target add aarch64-apple-darwin"
  fi
  if rustup target list --installed 2>/dev/null | grep -q "x86_64-apple-darwin"; then
    ok "rustup target: x86_64-apple-darwin"
  else
    miss "rustup target: x86_64-apple-darwin" "rustup target add x86_64-apple-darwin"
  fi
else
  skip "macOS targets" "not on macOS — skipping"
fi

echo ""
if [[ "${PASS}" != "true" ]]; then
  echo "Some prerequisites are missing. Install them and re-run."
  echo ""
  exit 1
else
  echo "All prerequisites satisfied. Ready to build."
  echo ""
fi
