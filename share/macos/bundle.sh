#!/bin/sh
set -eu

# Usage: sh share/macos/bundle.sh <mpv-handler binary> <output.app>
src=$(dirname "$0")
app=$2
osacompile -o "$app" "$src/handler.applescript"
plist="$app/Contents/Info.plist"
plutil -replace CFBundleIdentifier -string io.github.akiirui.mpv-handler "$plist"
plutil -replace LSUIElement -bool YES "$plist"
plutil -replace NSAppleEventsUsageDescription -string "mpv-handler uses Terminal to display debug playback logs." "$plist"
plutil -replace CFBundleURLTypes -json '[{"CFBundleTypeRole":"Viewer","CFBundleURLName":"mpv-handler","CFBundleURLSchemes":["mpv-handler","mpv-handler-debug"]}]' "$plist"
install -m 755 "$src/open-url" "$app/Contents/Resources/open-url"
install -m 755 "$1" "$app/Contents/MacOS/mpv-handler"
codesign --force --sign - "$app/Contents/MacOS/mpv-handler"
codesign --force --sign - "$app"
