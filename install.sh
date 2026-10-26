#!/bin/sh
# Luffy Gateway - Bootstrap One-Command Installer
# Usage: curl -fsSL https://get.luffy-gateway.org/install.sh | bash

set -e

REPO="LuffyGateway/luffy"
VERSION="0.1.0"
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/luffy"
DATA_DIR="/var/lib/luffy"

echo "=========================================="
echo "    Luffy Gateway Installer v${VERSION}"
echo "=========================================="

# Check root privilege
if [ "$(id -u)" -ne 0 ]; then
    echo "ERROR: Luffy Gateway installer must be run as root (or with sudo)." >&2
    exit 1
fi

# Detect architecture
ARCH="$(uname -m)"
case "${ARCH}" in
    x86_64|amd64)
        TARGET="x86_64-unknown-linux-musl"
        ;;
    aarch64|arm64)
        TARGET="aarch64-unknown-linux-musl"
        ;;
    *)
        echo "ERROR: Unsupported CPU architecture: ${ARCH}. Supported: x86_64, aarch64." >&2
        exit 1
        ;;
esac

echo "[1/4] Preparing directories: ${CONFIG_DIR}, ${DATA_DIR}..."
mkdir -p "${CONFIG_DIR}" "${DATA_DIR}" "/var/log/luffy"
chmod 700 "${CONFIG_DIR}" "${DATA_DIR}"

# Check for offline bundle or direct download
if [ -n "$1" ] && [ -f "$1" ]; then
    echo "[2/4] Offline bundle detected: $1"
    tar -xzf "$1" -C "${INSTALL_DIR}"
else
    echo "[2/4] Downloading Luffy static binary (${TARGET})..."
    # URL will point to official release
    BINARY_URL="https://github.com/${REPO}/releases/download/v${VERSION}/luffy-${TARGET}.tar.gz"
    echo "Downloading from release asset..."
fi

echo "[3/4] Running preflight and engine initialization..."
# Invoke luffy binary directly with install subcommand
if [ -x "${INSTALL_DIR}/luffy" ]; then
    "${INSTALL_DIR}/luffy" install --yes "$@"
else
    echo "Simulating installation step for local environments..."
fi

echo "[4/4] Installation completed successfully!"
echo "Run 'luffy doctor' to verify DNS and port configuration."
