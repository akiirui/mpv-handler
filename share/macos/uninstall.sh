#!/bin/sh
# Unregister the bundle from LaunchServices, then delete it.
#
# Unregistering first matters: LaunchServices keeps a claim on the scheme for a
# path that no longer exists, and that stale claim wins resolution over a
# working install, so deleting the bundle by hand leaves the scheme broken.
#
# Usage: sh uninstall.sh [install-dir]     (default: $HOME/Applications)
set -eu

APP_NAME=mpv-handler.app
LSREGISTER="/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister"

DEST=${1:-$HOME/Applications}
APP=$DEST/$APP_NAME
PREV=$DEST/.$APP_NAME.previous
STAGE=$DEST/.$APP_NAME.stage
LOCK=$DEST/.$APP_NAME.lock

if [ ! -d "$DEST" ]; then
    echo "not installed: $APP"
    exit 0
fi
if ! /usr/bin/shlock -p "$$" -f "$LOCK"; then
    echo "error: cannot acquire installation lock at $LOCK" >&2
    exit 1
fi
trap 'rm -f -- "$LOCK"' 0
trap 'exit 1' HUP INT TERM

# An interrupted upgrade can leave the registered bundle at its backup path.
# Put it back at the registered path before unregistering it.
if [ -d "$PREV" ]; then
    if [ -d "$APP" ]; then
        "$LSREGISTER" -u "$APP" 2>/dev/null || true
        rm -rf -- "$APP"
    fi
    mv -- "$PREV" "$APP"
fi
rm -rf -- "$STAGE"

# install.sh bakes XDG_CONFIG_HOME into the bundle, so read the config
# directory back from there instead of re-deriving it from an environment that
# may have changed since. This has to happen before the bundle is deleted.
CONFIG_HOME=$HOME/.config
if [ "$DEST" -ef /Applications ]; then
    CONFIG_HOME='${XDG_CONFIG_HOME:-$HOME/.config}'
fi
PLIST=$APP/Contents/Info.plist
if [ -f "$PLIST" ]; then
    BAKED=$(/usr/libexec/PlistBuddy -c 'Print :LSEnvironment:XDG_CONFIG_HOME' "$PLIST" 2>/dev/null) || BAKED=""
    if [ -n "$BAKED" ]; then
        CONFIG_HOME=$BAKED
    fi
fi

# Failing to unregister is fine: it may never have been registered.
"$LSREGISTER" -u "$APP" 2>/dev/null || true

if [ -d "$APP" ]; then
    rm -rf -- "$APP"
    echo "removed $APP"
else
    echo "not installed: $APP"
fi

echo ""
echo "Note: config directory was NOT removed:"
echo "  $CONFIG_HOME/mpv-handler/"
echo "Remove it manually if you want a clean uninstall."
