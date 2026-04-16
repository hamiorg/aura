#!/usr/bin/env bash
# Install aura on macOS — auto-detects architecture, downloads from GitHub.
#
# -----------------------------------------------------------------------
# Preferred: install via Homebrew (keeps aura up to date with brew upgrade)
#
#   brew install hamiorg/aura/aura
#
# -----------------------------------------------------------------------
# Direct download (this script):
#
#   curl -fsSL https://hami.aduki.org/install-mac.sh | bash
#
# To pin a specific version:
#   AURA_VERSION=v0.1.0 curl -fsSL https://hami.aduki.org/install-mac.sh | bash
#
# To install to a custom prefix (default: /usr/local/bin):
#   AURA_PREFIX=/opt/homebrew/bin curl -fsSL ... | bash
#
# Supports: arm64 (Apple Silicon M1/M2/M3), x86_64 (Intel)

set -euo pipefail

REPO="https://github.com/aduki-org/hami"
API="https://api.github.com/repos/aduki-org/hami/releases/latest"
PREFIX="${AURA_PREFIX:-/usr/local/bin}"
VERSION="${AURA_VERSION:-}"

# ------------------------------------------------------------------ #
# Platform check

if [[ "$(uname)" != "Darwin" ]]; then
  echo "error: this script is for macOS only"
  echo "       For Linux, use: curl -fsSL https://hami.aduki.org/install.sh | bash"
  exit 1
fi

# ------------------------------------------------------------------ #
# Resolve version

if [[ -z "${VERSION}" ]]; then
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

echo "==> Installing aura ${VERSION} on macOS"

# ------------------------------------------------------------------ #
# Detect architecture

ARCH=$(uname -m)
case "${ARCH}" in
  arm64)  ARCH_TAG="arm64" ;;
  x86_64) ARCH_TAG="x86_64" ;;
  *)
    echo "error: unsupported architecture: ${ARCH}"
    exit 1
    ;;
esac

# ------------------------------------------------------------------ #
# Download and install

TARBALL="aura-${VERSION}-macos-${ARCH_TAG}.tar.gz"
URL="${REPO}/releases/download/${VERSION}/${TARBALL}"
TMP=$(mktemp -d)

echo "==> Downloading ${TARBALL}"
echo "    from ${URL}"

trap 'rm -rf "${TMP}"' EXIT

curl -fsSL --progress-bar "${URL}" -o "${TMP}/${TARBALL}"
tar -xzf "${TMP}/${TARBALL}" -C "${TMP}"

# ------------------------------------------------------------------ #
# Install binary

EXTRACTED_BIN="${TMP}/aura-${VERSION}-macos-${ARCH_TAG}/aura"

if [[ ! -f "${EXTRACTED_BIN}" ]]; then
  echo "error: binary not found in downloaded archive"
  exit 1
fi

# Ensure the prefix directory exists.
if [[ ! -d "${PREFIX}" ]]; then
  echo "==> Creating ${PREFIX}"
  mkdir -p "${PREFIX}" 2>/dev/null || sudo mkdir -p "${PREFIX}"
fi

echo "==> Installing to ${PREFIX}/aura"

if [[ -w "${PREFIX}" ]]; then
  install -m 755 "${EXTRACTED_BIN}" "${PREFIX}/aura"
else
  echo "    (using sudo — you may be prompted for your password)"
  sudo install -m 755 "${EXTRACTED_BIN}" "${PREFIX}/aura"
fi

# ------------------------------------------------------------------ #
# Remove quarantine flag (macOS Gatekeeper)
# The binary is unsigned; remove the quarantine attribute so it runs
# without prompting the user to allow it in System Preferences.

if command -v xattr &>/dev/null; then
  xattr -d com.apple.quarantine "${PREFIX}/aura" 2>/dev/null || true
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
  echo "    For zsh: echo 'export PATH=\"${PREFIX}:\$PATH\"' >> ~/.zshrc"
fi
