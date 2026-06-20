#!/usr/bin/env bash
# Download yt-dlp into Tauri bundle resources for release installers.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEST_DIR="${SCRIPT_DIR}/../src-tauri/resources/bin"
mkdir -p "$DEST_DIR"

OS="$(uname -s)"
case "$OS" in
  Darwin)
    URL="https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos"
    ;;
  *)
    URL="https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp"
    ;;
esac
DEST="${DEST_DIR}/yt-dlp"

echo "Downloading ${URL} -> ${DEST} (platform: ${OS})"
curl -fsSL -A "wisper-bundle-script" -o "$DEST" "$URL"
chmod +x "$DEST"
if [[ "$OS" == "Darwin" ]]; then
  xattr -cr "$DEST" 2>/dev/null || true
  codesign --force --sign - "$DEST" 2>/dev/null || true
fi

if [[ ! -f "$DEST" ]]; then
  echo "yt-dlp bundle download failed" >&2
  exit 1
fi

echo "Bundled yt-dlp ($(wc -c < "$DEST") bytes)"
