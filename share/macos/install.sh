#!/bin/sh
# Install mpv-handler.app for the current user.
#
# macOS cannot register a bare executable as a protocol handler, so this
# script wraps the binary in an application bundle, ad-hoc signs it and
# registers it with LaunchServices.
#
# Usage:
#   ./install.sh [-b /path/to/mpv-handler] [-d /Applications] [-h]
#
#   -b  Path of the mpv-handler binary. Defaults to the one next to this
#       script, which is how the release archive is laid out.
#   -d  Directory to install into. Defaults to $HOME/Applications, which
#       needs no administrator rights. Use /Applications to install for
#       every user of this machine.
#   -h  Print this help.

set -eu

BUNDLE_NAME="mpv-handler.app"
BUNDLE_ID="io.github.akiirui.mpv-handler"
LSREGISTER="/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister"

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
BIN="$SCRIPT_DIR/mpv-handler"
DEST="$HOME/Applications"

usage() {
    sed -n '2,17p' "$0" | sed 's/^# \{0,1\}//'
}

while getopts "b:d:h" opt; do
    case "$opt" in
        b) BIN=$OPTARG ;;
        d) DEST=$OPTARG ;;
        h) usage; exit 0 ;;
        *) usage >&2; exit 2 ;;
    esac
done

[ -f "$BIN" ] || { echo "error: binary not found: $BIN" >&2; exit 1; }
[ -f "$SCRIPT_DIR/Info.plist" ] || {
    echo "error: Info.plist not found next to install.sh: $SCRIPT_DIR" >&2
    exit 1
}

APP="$DEST/$BUNDLE_NAME"
echo "Installing mpv-handler"
echo "  binary: $BIN"
echo "  bundle: $APP"

mkdir -p "$DEST"
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"

cp "$BIN" "$APP/Contents/MacOS/mpv-handler"
chmod 755 "$APP/Contents/MacOS/mpv-handler"

# An archive downloaded from the internet carries com.apple.quarantine, and
# `cp` keeps it. Running a quarantined binary makes Gatekeeper claim it is
# damaged, so drop the flag from the copy before anything executes it. The
# original download is left untouched.
xattr -cr "$APP" 2>/dev/null || true

# Ask the clean copy for its version so that CFBundleVersion matches the
# release. Changing it is what makes LaunchServices pick up a new bundle.
VERSION=$("$APP/Contents/MacOS/mpv-handler" 2>/dev/null |
    sed -n 's/^mpv-handler \([0-9][0-9.]*\).*/\1/p' | head -n 1)
[ -n "$VERSION" ] || VERSION="0.0.0"
echo "  version: $VERSION"

sed "s/__VERSION__/$VERSION/g" "$SCRIPT_DIR/Info.plist" > "$APP/Contents/Info.plist"

# Apple Silicon refuses to run an unsigned binary, and a signed bundle must
# not carry extended attributes, so sign after everything else is in place.
# An ad-hoc signature is enough: nothing here goes through the App Store.
codesign --force --sign - "$APP"

# Configuration lives in $HOME/.config/mpv-handler/, matching the Linux
# layout. Seed it so the path is easy to find.
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/mpv-handler"
if [ ! -e "$CONFIG_DIR/config.toml" ] && [ -f "$SCRIPT_DIR/config.toml" ]; then
    mkdir -p "$CONFIG_DIR/cookies"
    cp "$SCRIPT_DIR/config.toml" "$CONFIG_DIR/config.toml"
    echo "  config: $CONFIG_DIR/config.toml"
fi

"$LSREGISTER" -f "$APP"

# LaunchServices only asks which app should handle a scheme when more than
# one claims it. `duti` makes the choice explicit when it is available.
if command -v duti >/dev/null 2>&1; then
    duti -s "$BUNDLE_ID" mpv-handler all
    duti -s "$BUNDLE_ID" mpv-handler-debug all
    echo "  registered as the default handler for mpv-handler:// and mpv-handler-debug://"
else
    echo "  registered with LaunchServices"
    echo "  note: install \`duti\` to set mpv-handler as the default handler"
    echo "        without being asked by the browser on first use"
fi

cat <<'EOF'

Done. mpv and yt-dlp are looked up in $PATH, /opt/homebrew/bin,
/usr/local/bin, /opt/local/bin, $HOME/.local/bin, $HOME/bin and inside
/Applications/mpv.app. Set `mpv` and `ytdl` in config.toml to override.

Check it with:
  open "mpv-handler://play/aHR0cHM6Ly9leGFtcGxlLmNvbS8"
EOF
