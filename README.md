[English][readme-en] | [简体中文][readme-zh-hans] | [繁体中文][readme-zh-hant]

[readme-en]: https://github.com/akiirui/mpv-handler/blob/main/README.md
[readme-zh-hans]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hans.md
[readme-zh-hant]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hant.md

# mpv handler

A protocol handler for **mpv**, written by Rust.

Use **mpv** and **yt-dlp** to play video and music from the websites.

Please use it with userscript:

[![play-with-mpv][badges-play-with-mpv]][greasyfork-play-with-mpv]

## Protocol

![](share/proto.png)

### Plugins

- `play`: Use mpv player to play video

### Encoded Video URL

Use [URL-Safe base64][wiki-url-base64] to encode the video URL.

Example (JavaScript):

```javascript
let data = btoa("https://www.youtube.com/watch?v=Ggkn2f5e-IU");
let safe = data.replace(/\//g, "_").replace(/\+/g, "-");
```

### Parameters (Optional)

```
cookies = [ www.domain.com.txt ]
quality = [ best, 2160p, 1440p, 1080p, 720p, 480p, 360p ]
```

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
5. Register xdg-mime (thanks for the [linuxuprising][linuxuprising] reminder)

```
$ xdg-mime default mpv-handler.desktop x-scheme-handler/mpv
```

6. Add `$HOME/.local/bin` to your environment variable `PATH` (if needed)
7. Create `$HOME/.config/mpv-handler/config.toml` and edit it (if needed)

### Windows

Windows users need to install `mpv-handler` manually.

#### Manual installation

1. Download [latest/mpv-handler-windows-x64.zip][download-windows]
2. Unzip the archive to the directory you want
3. Run `handler-install.bat` register protocol handler
4. Add **mpv** and **yt-dlp** to environment variable `PATH` (if needed)
5. Edit `config.toml` (if needed)

## Configuration

If you have already added **mpv** and **yt-dlp** to `PATH`, manual configuration is usually not required.

```toml
# scoop install mpv
mpv = "/usr/bin/mpv"   // for scoop "mpv

# Optional, Type: String
# The path of mpv binary
# Default value:
# - Linux: mpv
# - Windows: mpv.com

# scoop install yt-dlp
ytdl = "/usr/bin/yt-dlp"   // for scoop "yt-dlp"

# Optional, Type: String
# The path of youtube-dl binary
# Default value:
# - Linux: yt-dlp
# - Windows: yt-dlp.exe

# For Windows users:
#   The path can be "C:\\folder\\some.exe" or "C:/folder/some.exe"
```

[wiki-url-base64]: https://en.wikipedia.org/wiki/Base64#URL_applications
[badges-aur-git]: https://img.shields.io/aur/version/mpv-handler-git?label=mpv-handler-git&style=for-the-badge
[badges-aur]: https://img.shields.io/aur/version/mpv-handler?label=mpv-handler&style=for-the-badge
[badges-play-with-mpv]: https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=play-with-mpv&prefix=v&query=version&url=https%3A%2F%2Fgreasyfork.org%2Fscripts%2F416271.json
[download-aur-git]: https://aur.archlinux.org/packages/mpv-handler-git/
[download-aur]: https://aur.archlinux.org/packages/mpv-handler/
[download-linux]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-linux-x64.zip
[download-windows]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-windows-x64.zip
[greasyfork-play-with-mpv]: https://greasyfork.org/scripts/416271-play-with-mpv
[linuxuprising]: https://www.linuxuprising.com/2021/07/open-youtube-and-more-videos-from-your.html
