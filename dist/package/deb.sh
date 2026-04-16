#!/usr/bin/env bash
# Create .deb packages using cargo-deb.
#
# Usage:
#   ./dist/package/deb.sh <version>
#   ./dist/package/deb.sh v0.1.0
#
# Prerequisites:
#   cargo install cargo-deb
#   Linux musl binaries built via dist/build/linux.sh
#
# Output files (in dist/out/):
#   aura-{version}-linux-x86_64.deb
#   aura-{version}-linux-arm64.deb
#
# The packages install the aura binary to /usr/bin/aura.
# Install with:  sudo dpkg -i aura-{version}-linux-x86_64.deb
# Or via apt:    sudo apt install ./aura-{version}-linux-x86_64.deb

set -euo pipefail

VERSION="${1:?Usage: deb.sh <version>  (e.g. v0.1.0)}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE="$(cd "${SCRIPT_DIR}/../.." && pwd)"
OUT="${WORKSPACE}/dist/out"

mkdir -p "${OUT}"

if ! command -v cargo-deb &>/dev/null; then
  echo "error: cargo-deb not found — install with: cargo install cargo-deb"
  exit 1
fi

build_deb() {
  local target="${1}"
  local arch_label="${2}"
  local deb_arch="${3}"   # Debian arch names: amd64, arm64

  local bin="${WORKSPACE}/target/${target}/release/aura"
  if [[ ! -f "${bin}" ]]; then
    echo "[deb] skip  linux-${arch_label}  (no binary at target/${target}/release/aura)"
    return
  fi

  local outfile="${OUT}/aura-${VERSION}-linux-${arch_label}.deb"

  echo "[deb] building  linux-${arch_label}  (deb arch: ${deb_arch})"

  cd "${WORKSPACE}"
  # --no-build: binary is already compiled; cargo-deb just packages it.
  # --target:   tells cargo-deb where to find the binary.
  cargo deb \
    --no-build \
    --target "${target}" \
    -p compiler \
    --deb-version "${VERSION#v}" \
    --output "${outfile}"

  local size
  size=$(du -sh "${outfile}" | cut -f1)
  echo "[deb] ok    aura-${VERSION}-linux-${arch_label}.deb  (${size})"
}

echo "==> Packaging .deb for ${VERSION}"
echo ""

build_deb "x86_64-unknown-linux-musl"  "x86_64" "amd64"
build_deb "aarch64-unknown-linux-musl" "arm64"   "arm64"

echo ""
echo "==> Output: ${OUT}/"
ls -lh "${OUT}/"*.deb 2>/dev/null || echo "(no .deb packages produced)"
