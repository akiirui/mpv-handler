# mpv handler

A protocol handler for mpv, written by Rust.

Please use with userscript:

[![play-with-mpv-handler][play-with-mpv-badges]][play-with-mpv-greasyfork]

## Installation

### Linux

- Arch Linux
  - [mpv-handler][mpv-handler-download-aur] <sup>AUR</sup>
  - [mpv-handler-git][mpv-handler-download-aur-git] <sup>AUR</sup>

**Don't forget copy `/usr/share/mpv-handler/mpv-handler.toml` to `~/.config/mpv/`.**

#### Manual installation

1. Download [latest/mpv-handler-linux-x64.zip][mpv-handler-download-linux]
2. Unzip the archive
3. Copy `mpv-handler` to `~/.local/bin`
4. Copy `mpv-handler.desktop` to `~/.local/share/applications/`
5. Copy `mpv-handler.toml` to `~/.config/mpv/`
6. Add `~/.local/bin` to your environment variable `PATH` (if it not lists in your `PATH`)
7. Register xdg-mime (thanks for the [linuxuprising][linuxuprising] reminder)

```
$ xdg-mime default mpv-handler.desktop x-scheme-handler/mpv
```

8. Check `~/.config/mpv/mpv-handler.toml` and change it as needed

### Windows

Windows users need to install `mpv-handler` manually.

#### Manual installation

1. Download [latest/mpv-handler-windows-x64.zip][mpv-handler-download-windows]
2. Unzip the archive to the directory you want (since v0.2.x, not requires to install in the same directory with `mpv` anymore)
3. Run `handler-install.bat` register protocol handler
4. Check `~/.config/mpv/mpv-handler.toml` and change it as needed

[mpv-handler-download-aur]: https://aur.archlinux.org/packages/mpv-handler/
[mpv-handler-download-aur-git]: https://aur.archlinux.org/packages/mpv-handler-git/
[mpv-handler-download-linux]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-linux-x64.zip
[mpv-handler-download-windows]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-windows-x64.zip
[play-with-mpv-badges]: https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=play-with-mpv&prefix=v&query=version&url=https%3A%2F%2Fgreasyfork.org%2Fscripts%2F416271.json
[play-with-mpv-greasyfork]: https://greasyfork.org/scripts/416271-play-with-mpv
[play-with-mpv-github]: https://github.com/akiirui/userscript/tree/main/play-with-mpv-handler
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
```

Example:

```
mpv://aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj01cWFwNWFPNGk5QQ==/?cookies=www.youtube.com.txt&downloader=mpv&quality=best
```

## Customize Configuration

Generally, users only need to edit `player` and downloader `bin` to corresponding executable binary.

The default `mpv-handler.toml` configuration is like this:

```toml
### Player ###
# You should be change the value of "player" to your player executalbe binary path.
player = "/usr/bin/mpv"

### Video Downloader ###
# You should be change the value of "bin" to your downloader executable binary path.
[mpv]
bin = "/usr/bin/mpv"
cookies = "--ytdl-raw-options-append=cookies="
cookies_prefix = true
direct = true
quality.best = "--ytdl-format=bestvideo+bestaudio/best"
quality.360p = "--ytdl-format=bestvideo[height<=360]+bestaudio/best[height<=360]/best"
quality.480p = "--ytdl-format=bestvideo[height<=480]+bestaudio/best[height<=480]/best"
quality.720p = "--ytdl-format=bestvideo[height<=720]+bestaudio/best[height<=720]/best"
quality.1080p = "--ytdl-format=bestvideo[height<=1080]+bestaudio/best[height<=1080]/best"
quality.1440p = "--ytdl-format=bestvideo[height<=1440]+bestaudio/best[height<=1440]/best"
quality.2160p = "--ytdl-format=bestvideo[height<=2160]+bestaudio/best[height<=2160]/best"

[ytdl]
bin = "/usr/bin/youtube-dl"
cookies = "--cookies"
pipeline = true
options = ["--quiet", "--output", "-"]

[you-get]
bin = "/usr/bin/you-get"
cookies = "--cookies"
options = ["--player"]

[streamlink]
bin = "/usr/bin/streamlink"
require_quality = true
options = ["--player"]
quality.best = "best"
quality.worst = "worst"

# For advanced user, you can add other downloader manually.
# Example:
#
# [example]
# bin = "/usr/bin/example"
# cookies = "--cookies"
# cookies_prefix = false
# direct = false
# pipeline = false
# require_quality = false
# options = ["--player"]
# quality.best = "--quality=best"
#
#
# [example]       Required, Type: String
#                     The value "example" is downloader table name
# bin             Required, Type: String
#                     The downloader executable binary path.
# cookies         Optional, Type: String (default: "")
#                     The downloader parameter of passthorgh cookies.
# cookies_prefix  Optional, Type: Boolen (default: false)
#                     Set to true to mark cookies parameter as prefix.
# direct          Optional, Type: Boolen (defalut: false)
#                     Set to true to mark the downloader run directly without player.
# pipeline        Optional, Type: Boolen (default: false)
#                     Set to true to mark the downloader transfer video data through pipeline.
# require_quality Optional, Type: Boolen (default: false)
#                     Set to true to mark the downloader requires a quality LEVEL given.
# options         Optional, Type: Array of Strings (default: [])
#                     The parameters of downloader to set player or output.
# quality.LEVEL   Optional, Type: String
#                     The LEVEL is a key name
#                     The value is parameter of downloader to choose quality/format.
```
