#!/bin/sh
# Put the mpv-handler binary into an .app bundle, ad-hoc sign it, and register
# the mpv-handler:// and mpv-handler-debug:// schemes with LaunchServices.
#
# macOS has no registry key and no .desktop file: a bare executable cannot be a
# protocol handler, only a bundle whose Info.plist declares CFBundleURLTypes.
#
# Usage:
#   sh install.sh [binary] [install-dir]
#
#   binary       defaults to the packaged executable, or target/release/mpv-handler
#                in a source checkout
#   install-dir  defaults to $HOME/Applications, which needs no admin rights;
#                pass /Applications to install for every user of this machine
#
# The bundle is assembled beside the destination and renamed into place only
# after it signs and verifies, so a failed install never replaces a working one.
set -eu

SCHEME=mpv-handler
SCHEME_DEBUG=mpv-handler-debug
APP_NAME=mpv-handler.app
BUNDLE_ID=com.github.akiirui.mpv-handler
EXE=mpv-handler
LSREGISTER="/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister"

here=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
DEST=${2:-$HOME/Applications}
if [ -f "$here/VERSION" ]; then
    BIN=${1:-$here/$EXE}
    VERSION=$(cat "$here/VERSION")
elif [ -f "$here/../../Cargo.toml" ]; then
    BIN=${1:-$here/../../target/release/$EXE}
    VERSION=$(awk -F'"' '/^version = /{print $2; exit}' "$here/../../Cargo.toml")
else
    echo "error: missing release VERSION file or source Cargo.toml" >&2
    exit 1
fi

[ -f "$BIN" ] || {
    echo "error: no binary at $BIN (run: cargo build --release)" >&2
    exit 1
}
[ -n "$VERSION" ] || {
    echo "error: missing package version" >&2
    exit 1
}

APP=$DEST/$APP_NAME
STAGE=$DEST/.$APP_NAME.stage
PREV=$DEST/.$APP_NAME.previous
LOCK=$DEST/.$APP_NAME.lock

mkdir -p "$DEST"
# Keep concurrent installers out of the same staging and rollback directories.
if ! /usr/bin/shlock -p "$$" -f "$LOCK"; then
    echo "error: cannot acquire installation lock at $LOCK" >&2
    exit 1
fi
trap 'rm -rf "$STAGE"; rm -f "$LOCK"' 0
trap 'exit 1' HUP INT TERM
rm -rf "$STAGE"

restore_previous() {
    rm -rf "$APP"
    mv "$PREV" "$APP"
    if ! "$LSREGISTER" -f "$APP"; then
        echo "error: restored $APP but could not re-register it" >&2
        return 1
    fi
}
# A previous installer may have been interrupted between the swap and registration.
if [ -d "$PREV" ]; then
    restore_previous
fi

mkdir -p "$STAGE/Contents/MacOS"
cp "$BIN" "$STAGE/Contents/MacOS/$EXE"
chmod 755 "$STAGE/Contents/MacOS/$EXE"

PLIST_FILE=$STAGE/Contents/Info.plist
cat > "$PLIST_FILE" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleExecutable</key>
	<string>$EXE</string>
	<key>CFBundleIdentifier</key>
	<string>$BUNDLE_ID</string>
	<key>CFBundleInfoDictionaryVersion</key>
	<string>6.0</string>
	<key>CFBundleName</key>
	<string>$EXE</string>
	<key>CFBundlePackageType</key>
	<string>APPL</string>
	<key>CFBundleShortVersionString</key>
	<string>$VERSION</string>
	<key>CFBundleVersion</key>
	<string>$VERSION</string>
	<key>LSMinimumSystemVersion</key>
	<string>10.13</string>
	<key>LSUIElement</key>
	<true/>
	<key>LSEnvironment</key>
	<dict>
		<key>MPV_HANDLER_URL_SERVICE</key>
		<string>1</string>
	</dict>
	<key>CFBundleURLTypes</key>
	<array>
		<dict>
			<key>CFBundleTypeRole</key>
			<string>Viewer</string>
			<key>CFBundleURLName</key>
			<string>$EXE</string>
			<key>LSHandlerRank</key>
			<string>Owner</string>
			<key>CFBundleURLSchemes</key>
			<array>
				<string>$SCHEME</string>
				<string>$SCHEME_DEBUG</string>
			</array>
		</dict>
	</array>
</dict>
</plist>
PLIST

# LaunchServices does not inherit the installing shell's XDG_CONFIG_HOME.
CONFIG_HOME=$HOME/.config
if [ "$DEST" -ef /Applications ]; then
    CONFIG_HOME='${XDG_CONFIG_HOME:-$HOME/.config}'
else
    case ${XDG_CONFIG_HOME:-} in
        /*)
            CONFIG_HOME=$XDG_CONFIG_HOME
            /usr/bin/plutil -insert LSEnvironment.XDG_CONFIG_HOME -string "$CONFIG_HOME" "$PLIST_FILE"
            ;;
        '') ;;
        *) echo "warning: ignoring non-absolute XDG_CONFIG_HOME '$XDG_CONFIG_HOME'" >&2 ;;
    esac
fi

# Clear download and Finder attributes before signing the bundle and its main executable.
xattr -cr "$STAGE" 2>/dev/null || true
codesign --force --sign - "$STAGE"
codesign --verify --strict "$STAGE"

if [ -d "$APP" ]; then
    mv "$APP" "$PREV"
fi
if ! mv "$STAGE" "$APP"; then
    if [ -d "$PREV" ]; then
        mv "$PREV" "$APP"
    fi
    echo "error: cannot move the staged bundle into $APP" >&2
    exit 1
fi
if ! "$LSREGISTER" -f "$APP"; then
    echo "error: cannot register $APP" >&2
    "$LSREGISTER" -u "$APP" 2>/dev/null || true
    if [ -d "$PREV" ]; then
        restore_previous
    else
        rm -rf "$APP"
    fi
    exit 1
fi
# Commit before cleaning up, so an interrupted deletion cannot leave a partial backup.
if [ -d "$PREV" ]; then
    mv "$PREV" "$STAGE"
fi

echo "installed $APP ($VERSION)"
echo "registered as the handler for $SCHEME:// and $SCHEME_DEBUG://"
echo ""
echo "Config directory: $CONFIG_HOME/mpv-handler/"
echo ""
echo "Test with:"
echo "  open '$SCHEME://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g_dj1HZ2tuMmY1ZS1JVQ'"
