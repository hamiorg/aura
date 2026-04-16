#!/usr/bin/env bash
# Install aura on Linux — auto-detects architecture, downloads from GitHub.
#
# One-line install:
#   curl -fsSL https://hami.aduki.org/install.sh | bash
#
# To pin a specific version:
#   AURA_VERSION=v0.1.0 curl -fsSL https://hami.aduki.org/install.sh | bash
#
# To install to a custom prefix (default: /usr/local/bin):
#   AURA_PREFIX=/usr/bin curl -fsSL https://hami.aduki.org/install.sh | bash
#
# Supports: x86_64, aarch64 (arm64)

set -euo pipefail

REPO="https://github.com/aduki-org/hami"
API="https://api.github.com/repos/aduki-org/hami/releases/latest"
PREFIX="${AURA_PREFIX:-/usr/local/bin}"
VERSION="${AURA_VERSION:-}"

# ------------------------------------------------------------------ #
# Resolve version

if [[ -z "${VERSION}" ]]; then
  if ! command -v curl &>/dev/null; then
    echo "error: curl is required for installation"
    exit 1
  fi
  echo "==> Resolving latest release..."
  VERSION=$(
    curl -fsSL "${API}" \
    | grep '"tag_name"' \
    | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/'
  )
  if [[ -z "${VERSION}" ]]; then
    echo "error: could not determine latest release version"
    echo "       Set AURA_VERSION=vX.Y.Z to install a specific version"
    exit 1
  fi
fi

echo "==> Installing aura ${VERSION} on Linux"

# ------------------------------------------------------------------ #
# Detect architecture

ARCH=$(uname -m)
case "${ARCH}" in
  x86_64)          ARCH_TAG="x86_64" ;;
  aarch64 | arm64) ARCH_TAG="arm64" ;;
  *)
    echo "error: unsupported architecture: ${ARCH}"
    echo "       Supported: x86_64, aarch64"
    exit 1
    ;;
esac

# ------------------------------------------------------------------ #
# Download and install

TARBALL="aura-${VERSION}-linux-${ARCH_TAG}.tar.gz"
URL="${REPO}/releases/download/${VERSION}/${TARBALL}"
TMP=$(mktemp -d)

echo "==> Downloading ${TARBALL}"
echo "    from ${URL}"

trap 'rm -rf "${TMP}"' EXIT

curl -fsSL --progress-bar "${URL}" -o "${TMP}/${TARBALL}"
tar -xzf "${TMP}/${TARBALL}" -C "${TMP}"

# ------------------------------------------------------------------ #
# Install binary

EXTRACTED_BIN="${TMP}/aura-${VERSION}-linux-${ARCH_TAG}/aura"

if [[ ! -f "${EXTRACTED_BIN}" ]]; then
  echo "error: binary not found in downloaded archive"
  exit 1
fi

echo "==> Installing to ${PREFIX}/aura"

# Use sudo only if the prefix is not writable by the current user.
if [[ -w "${PREFIX}" ]]; then
  install -m 755 "${EXTRACTED_BIN}" "${PREFIX}/aura"
else
  echo "    (using sudo — you may be prompted for your password)"
  sudo install -m 755 "${EXTRACTED_BIN}" "${PREFIX}/aura"
fi

# ------------------------------------------------------------------ #
# Verify

if command -v aura &>/dev/null; then
  echo ""
  echo "==> aura installed successfully"
  aura --version
else
  echo ""
  echo "==> aura installed to ${PREFIX}/aura"
  echo "    Add ${PREFIX} to your PATH if it is not already there."
fi
