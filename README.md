[English][readme-en] | [简体中文][readme-zh-hans] | [繁体中文][readme-zh-hant]

[readme-en]: https://github.com/akiirui/mpv-handler/blob/main/README.md
[readme-zh-hans]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hans.md
[readme-zh-hant]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hant.md

# mpv handler

A protocol handler for mpv, written by Rust.

Please use with userscript:

[![play-with-mpv][badges-play-with-mpv]][greasyfork-play-with-mpv]

## Installation

### Linux

- Arch Linux

  [![mpv-handler][badges-aur]][download-aur] \
  [![mpv-handler-git][badges-aur-git]][download-aur-git]

#### Manual installation

1. Download [latest/mpv-handler-linux-x64.zip][download-linux]
2. Unzip the archive
3. Copy `mpv-handler` to `$HOME/.local/bin`
4. Copy `mpv-handler.desktop` to `$HOME/.local/share/applications/`
5. Copy `config.toml` to `$HOME/.config/mpv-handler/`
6. Add `$HOME/.local/bin` to your environment variable `PATH` (if it not lists in your `PATH`)
7. Register xdg-mime (thanks for the [linuxuprising][linuxuprising] reminder)

```
$ xdg-mime default mpv-handler.desktop x-scheme-handler/mpv
```

8. Create `$HOME/.config/mpv-handler/custom.toml` and edit it, if needed

### Windows

Windows users need to install `mpv-handler` manually.

#### Manual installation

1. Download [latest/mpv-handler-windows-x64.zip][download-windows]
2. Unzip the archive to the directory you want (since v0.2.x, not requires to install in the same directory with `mpv` anymore)
3. Run `handler-install.bat` register protocol handler
4. Create `custom.toml` in the same directory as `mpv-handler.exe` and edit it

[badges-aur-git]: https://img.shields.io/aur/version/mpv-handler-git?label=mpv-handler-git&style=for-the-badge
[badges-aur]: https://img.shields.io/aur/version/mpv-handler?label=mpv-handler&style=for-the-badge
[badges-play-with-mpv]: https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=play-with-mpv&prefix=v&query=version&url=https%3A%2F%2Fgreasyfork.org%2Fscripts%2F416271.json
[download-aur-git]: https://aur.archlinux.org/packages/mpv-handler-git/
[download-aur]: https://aur.archlinux.org/packages/mpv-handler/
[download-linux]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-linux-x64.zip
[download-windows]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-windows-x64.zip
[greasyfork-play-with-mpv]: https://greasyfork.org/scripts/416271-play-with-mpv
[linuxuprising]: https://www.linuxuprising.com/2021/07/open-youtube-and-more-videos-from-your.html

## Protocol URL

Base URL:

```
mpv://BASE64_ENCODE_VIDEO_URL/
```

Optional parameters:

```
cookies     = [ www.domain.com.txt ]
downloader  = [ mpv, ytdl, you-get, streamlink, and more...] (default: mpv)
quality     = [ best, 2160p, 1440p, 1080p, 720p, 480p, 360p, and more... ]

c = cookies
d = downloader
q = quality
```

Example:

```
mpv://aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj01cWFwNWFPNGk5QQ==/?cookies=www.youtube.com.txt&downloader=mpv&quality=best

mpv://aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj1wNVFmeUY5cGtIVQ==/?c=www.youtube.com.txt&d=mpv&q=best
```

## Custom Configuration

The default `config.toml` configuration is like this:

```toml
# Don't edit this file!
# This is default settings, It will be overwritten when update mpv-handler.
#
# For customize settings, create and edit file:
# - Linux:
#     - $HOME/.config/mpv-handler/custom.toml
#     - /etc/mpv-handler/custom.toml
#   If the first one is found, the second one will not be loaded.
# - Windows: custom.toml (in the same directory as mpv-handler.exe)

### Player ###
player = "/usr/bin/mpv"

### Video Downloader ###
[mpv]
bin = "/usr/bin/mpv"
cookies = "--ytdl-raw-options-append=cookies="
cookies_prefix = true
play_mode = "direct"
quality.best = "--ytdl-format=bestvideo+bestaudio/best"
quality.2160p = "--ytdl-format=bestvideo[height<=2160]+bestaudio/best[height<=2160]/best"
quality.1440p = "--ytdl-format=bestvideo[height<=1440]+bestaudio/best[height<=1440]/best"
quality.1080p = "--ytdl-format=bestvideo[height<=1080]+bestaudio/best[height<=1080]/best"
quality.720p = "--ytdl-format=bestvideo[height<=720]+bestaudio/best[height<=720]/best"
quality.480p = "--ytdl-format=bestvideo[height<=480]+bestaudio/best[height<=480]/best"
quality.360p = "--ytdl-format=bestvideo[height<=360]+bestaudio/best[height<=360]/best"
```

Generally, users only need to edit `player` and downloader `bin` to corresponding executable binary.

For this, users can create `custom.toml` to overwrite default settings:

```toml
# For Windows users,
# The path format can be "C:\\folder\\some.exe" or "C:/folder/some.exe"
player = "/usr/bin/vlc"

# Warning:
# Users are not recommended to change default downloader settings except "bin".
#
# If you've changed "quality.LEVEL" for default downloader,
# You will lost other "quality.LEVEL" from default settings.
[mpv]
bin = "/usr/local/bin/mpv"
quality.best = "--ytdl-format=best"

# For advanced user, you can add other downloader manually.
#
# Example:
[example]
bin = "/usr/bin/example"
cookies = "--cookies"
cookies_prefix = false
require_quality = false
play_mode = "normal"
options = ["--player"]
player_options = ["--http-header-fields='referer: https://www.domain.com'"]
quality.best = "--quality=best"
quality.worst = "--quality=worst"

# [example]       Required, Type: String
#                     The value "example" is downloader table name
# bin             Required, Type: String
#                     The downloader executable binary path
# cookies         Optional, Type: String (default: "")
#                     The downloader parameter of passthorgh cookies
# cookies_prefix  Optional, Type: Boolen (default: false)
#                     Set to true to mark cookies parameter as prefix
# require_quality Optional, Type: Boolen (default: false)
#                     Set to true to mark the downloader requires a quality LEVEL given
# play_mode       Optional, Type: String [normal, direct, pipe] (default: "normal")
#                     The mode of downloader to run player
# options         Optional, Type: Array of Strings (default: [])
#                     The parameters of downloader to set player or output
# player_options  Optional, Type: Array of Strings (default: [])
#                     The parameters of player for some special purposes
# quality.LEVEL   Optional, Type: String
#                     The LEVEL is a key name
#                     The value is parameter of downloader to choose quality/format
```

### Downloader examples

See [share/examples][examples].

[examples]: https://github.com/akiirui/mpv-handler/tree/main/share/examples
