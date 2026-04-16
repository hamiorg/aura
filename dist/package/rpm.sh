#!/usr/bin/env bash
# Create .rpm packages using cargo-generate-rpm.
#
# Usage:
#   ./dist/package/rpm.sh <version>
#   ./dist/package/rpm.sh v0.1.0
#
# Prerequisites:
#   cargo install cargo-generate-rpm
#   Linux musl binaries built via dist/build/linux.sh
#
# Output files (in dist/out/):
#   aura-{version}-linux-x86_64.rpm
#   aura-{version}-linux-arm64.rpm
#
# The packages install the aura binary to /usr/bin/aura.
# Install with:  sudo rpm -i aura-{version}-linux-x86_64.rpm
# Or via dnf:    sudo dnf install ./aura-{version}-linux-x86_64.rpm

set -euo pipefail

VERSION="${1:?Usage: rpm.sh <version>  (e.g. v0.1.0)}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE="$(cd "${SCRIPT_DIR}/../.." && pwd)"
OUT="${WORKSPACE}/dist/out"

mkdir -p "${OUT}"

if ! command -v cargo-generate-rpm &>/dev/null; then
  echo "error: cargo-generate-rpm not found — install with: cargo install cargo-generate-rpm"
  exit 1
fi

# RPM Version must be numbers and dots only — no hyphens allowed.
# Split a SemVer pre-release tag like v0.1.0-alpha.1 into:
#   RPM_VER  = "0.1.0"   (the part before the first hyphen)
#   RPM_REL  = "alpha.1" (the pre-release identifier, or "1" for stable)
VERSION_PLAIN="${VERSION#v}"
if [[ "${VERSION_PLAIN}" == *-* ]]; then
  RPM_VER="${VERSION_PLAIN%%-*}"
  RPM_REL="${VERSION_PLAIN#*-}"
else
  RPM_VER="${VERSION_PLAIN}"
  RPM_REL="1"
fi

build_rpm() {
  local target="${1}"
  local arch_label="${2}"
  local rpm_arch="${3}"   # RPM arch names: x86_64, aarch64

  local bin="${WORKSPACE}/target/${target}/release/aura"
  if [[ ! -f "${bin}" ]]; then
    echo "[rpm] skip  linux-${arch_label}  (no binary at target/${target}/release/aura)"
    return
  fi

  local outfile="${OUT}/aura-${VERSION}-linux-${arch_label}.rpm"

  echo "[rpm] building  linux-${arch_label}  (rpm arch: ${rpm_arch})"

  cd "${WORKSPACE}"
  # RPM Version and Release are passed as quoted TOML string values.
  # version = base semver (e.g. "0.1.0"), release = pre-release or "1".
  cargo generate-rpm \
    --target "${target}" \
    -p compiler \
    --arch "${rpm_arch}" \
    --set-metadata "version='${RPM_VER}'" \
    --set-metadata "release='${RPM_REL}'" \
    --output "${outfile}"

  local size
  size=$(du -sh "${outfile}" | cut -f1)
  echo "[rpm] ok    aura-${VERSION}-linux-${arch_label}.rpm  (${size})"
}

echo "==> Packaging .rpm for ${VERSION}"
echo ""

# Note: RPM uses 'aarch64', not 'arm64', as the architecture identifier.
build_rpm "x86_64-unknown-linux-musl"  "x86_64" "x86_64"
build_rpm "aarch64-unknown-linux-musl" "arm64"   "aarch64"

echo ""
echo "==> Output: ${OUT}/"
ls -lh "${OUT}/"*.rpm 2>/dev/null || echo "(no .rpm packages produced)"
