#!/bin/sh
# One-line installer for mpv-handler on macOS.
#
#   curl -fsSL https://raw.githubusercontent.com/akiirui/mpv-handler/main/share/macos/remote-install.sh | sh
#
# Downloads the latest release archive, verifies its sha512 checksum and
# hands over to the install.sh shipped inside the archive, which assembles
# and registers mpv-handler.app.
#
# Environment:
#   MPV_HANDLER_REPO  owner/repo to download from (default: akiirui/mpv-handler)
#   MPV_HANDLER_TAG   release tag to pin instead of "latest"
#
# Arguments are passed to install.sh, e.g.  | sh -s -- -d /Applications

set -eu

REPO=${MPV_HANDLER_REPO:-akiirui/mpv-handler}
TAG=${MPV_HANDLER_TAG:-latest}
ASSET="mpv-handler-macos-universal.zip"

if [ "$(uname -s)" != "Darwin" ]; then
    echo "error: this installer only supports macOS" >&2
    exit 1
fi

case "$TAG" in
    latest) BASE="https://github.com/$REPO/releases/latest/download" ;;
    *)      BASE="https://github.com/$REPO/releases/download/$TAG" ;;
esac

TMP=$(mktemp -d "${TMPDIR:-/tmp}/mpv-handler-install.XXXXXX")
trap 'rm -rf "$TMP"' EXIT INT TERM

echo "Downloading $BASE/$ASSET"
curl -fsSL "$BASE/sha512sum" -o "$TMP/sha512sum"
curl -fsSL "$BASE/$ASSET" -o "$TMP/$ASSET"

# The checksum file travels in the same release, over the same TLS
# connection, so a tampered archive cannot pass this check unnoticed.
echo "Verifying checksum"
(
    cd "$TMP"
    grep "$ASSET" sha512sum | shasum -a 512 -c -
)

echo "Unpacking"
unzip -q -o "$TMP/$ASSET" -d "$TMP/pkg"

sh "$TMP/pkg/install.sh" "$@"
