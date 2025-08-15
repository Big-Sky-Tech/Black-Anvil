#!/usr/bin/env bash
# Self-installer for Black Anvil.
# Builds Black Anvil and uses it to install itself with default settings.
set -euo pipefail

INSTALL_DIR="${1:-/usr/local/bin}"

echo "Building Black Anvil..."
cargo build --release

CONFIG_FILE="$(mktemp)"
cat > "$CONFIG_FILE" <<CONFIG
project_path = "."
build_type = "release"
install_dir = "$INSTALL_DIR"
vendor = false
CONFIG

echo "Installing Black Anvil to $INSTALL_DIR..."
./target/release/black-anvil "$CONFIG_FILE"
rm "$CONFIG_FILE"

echo "Black Anvil installed to $INSTALL_DIR"
