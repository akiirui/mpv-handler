#!/bin/sh
# Remove mpv-handler.app and unregister its URL schemes.
#
# Usage:
#   ./uninstall.sh [-d /Applications] [-c] [-h]
#
#   -d  Directory the bundle was installed into. Defaults to
#       $HOME/Applications, matching the install.sh default.
#   -c  Also remove $HOME/.config/mpv-handler/, including any cookies.
#   -h  Print this help.

set -eu

BUNDLE_NAME="mpv-handler.app"
LSREGISTER="/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister"

DEST="$HOME/Applications"
REMOVE_CONFIG=0

usage() {
    sed -n '2,12p' "$0" | sed 's/^# \{0,1\}//'
}

while getopts "d:ch" opt; do
    case "$opt" in
        d) DEST=$OPTARG ;;
        c) REMOVE_CONFIG=1 ;;
        h) usage; exit 0 ;;
        *) usage >&2; exit 2 ;;
    esac
done

APP="$DEST/$BUNDLE_NAME"

if [ -d "$APP" ]; then
    # Unregister before deleting, so LaunchServices drops the claim on
    # mpv-handler:// and mpv-handler-debug:// instead of keeping a stale one.
    "$LSREGISTER" -u "$APP" 2>/dev/null || true
    rm -rf "$APP"
    echo "Removed $APP"
else
    echo "Not installed: $APP"
fi

if [ "$REMOVE_CONFIG" -eq 1 ]; then
    CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/mpv-handler"
    if [ -d "$CONFIG_DIR" ]; then
        rm -rf "$CONFIG_DIR"
        echo "Removed $CONFIG_DIR"
    fi
fi

echo "Done. mpv-handler:// and mpv-handler-debug:// are no longer handled."
