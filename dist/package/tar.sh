#!/usr/bin/env bash
# Create .tar.gz tarballs from compiled binaries.
#
# Usage:
#   ./dist/package/tar.sh <version>
#   ./dist/package/tar.sh v0.1.0
#
# Expects binaries to already be built via dist/build/linux.sh and/or
# dist/build/mac.sh. Missing targets are skipped with a warning.
#
# Output files (in dist/out/):
#   aura-{version}-linux-x86_64.tar.gz
#   aura-{version}-linux-arm64.tar.gz
#   aura-{version}-macos-arm64.tar.gz
#   aura-{version}-macos-x86_64.tar.gz
#
# Each tarball contains:
#   aura-{version}-{platform}-{arch}/
#     aura          <- the binary
#     README.md     <- repo readme (if present)

set -euo pipefail

VERSION="${1:?Usage: tar.sh <version>  (e.g. v0.1.0)}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE="$(cd "${SCRIPT_DIR}/../.." && pwd)"
OUT="${WORKSPACE}/dist/out"

mkdir -p "${OUT}"

pack() {
  local target="${1}"
  local platform="${2}"
  local arch="${3}"
  local bin="${WORKSPACE}/target/${target}/release/aura"

  if [[ ! -f "${bin}" ]]; then
    echo "[tar] skip  ${platform}-${arch}  (no binary at target/${target}/release/aura)"
    return
  fi

  local name="aura-${VERSION}-${platform}-${arch}"
  local staging="${OUT}/${name}"

  mkdir -p "${staging}"
  cp "${bin}" "${staging}/aura"

  # Include README if it exists at the aura repo root.
  local readme="${WORKSPACE}/README.md"
  if [[ -f "${readme}" ]]; then
    cp "${readme}" "${staging}/README.md"
  fi

  tar -czf "${OUT}/${name}.tar.gz" -C "${OUT}" "${name}"
  rm -rf "${staging}"

  local size
  size=$(du -sh "${OUT}/${name}.tar.gz" | cut -f1)
  echo "[tar] ok    ${name}.tar.gz  (${size})"
}

echo "==> Packaging tarballs for ${VERSION}"
echo ""

pack "x86_64-unknown-linux-musl"  "linux" "x86_64"
pack "aarch64-unknown-linux-musl" "linux" "arm64"
pack "aarch64-apple-darwin"       "macos" "arm64"
pack "x86_64-apple-darwin"        "macos" "x86_64"

echo ""
echo "==> Output: ${OUT}/"
ls -lh "${OUT}/"*.tar.gz 2>/dev/null || echo "(no tarballs produced)"
