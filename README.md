[English][readme-en] | [简体中文][readme-zh-hans] | [繁体中文][readme-zh-hant]

[readme-en]: https://github.com/akiirui/mpv-handler/blob/main/README.md
[readme-zh-hans]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hans.md
[readme-zh-hant]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hant.md

# mpv handler

A protocol handler for **mpv**, written by Rust.

Use **mpv** and **yt-dlp** to play video and music from the websites.

Please use it with userscript:

[![play-with-mpv][badges-play-with-mpv]][greasyfork-play-with-mpv]

## Breaking changes

### [v0.4.0][v0.4.0]

To avoid conflicts with the `mpv://` protocol provided by mpv.

> mpv://...
>
> mpv protocol. This is used for starting mpv from URL handler. The protocol is stripped and the rest is passed to the player as a normal open argument. Only safe network protocols are allowed to be opened this way.

Scheme `mpv://` and `mpv-debug://` are deprecated, use `mpv-handler://` and `mpv-handler-debug://`.

**Require manual intervention**

#### Windows

Run `handler-uninstall.bat` to uninstall deprecated protocol, and run `handler-install.bat` to install new procotol.

#### Linux

If you installed manually, please repeat the manual installation process.

## Protocol

![](share/proto.png)

### Scheme

- `mpv-handler`: Run mpv-handler without console window
- `mpv-handler-debug`: Run mpv-handler with console window to view outputs and errors

### Plugins

- `play`: Use mpv player to play video

### Encoded Data

Use [URL-safe base64][rfc-base64-url] to encode the URL or TITLE.

Replace `/` to `_`, `+` to `-` and remove padding `=`.

Example (JavaScript):

```javascript
let data = btoa("https://www.youtube.com/watch?v=Ggkn2f5e-IU");
let safe = data.replace(/\//g, "_").replace(/\+/g, "-").replace(/\=/g, "");
```

### Parameters (Optional)

```
cookies  = [ www.domain.com.txt ]
profile  = [ default, low-latency, etc... ]
quality  = [ 2160p, 1440p, 1080p, 720p, 480p, 360p ]
v_codec  = [ av01, vp9, h265, h264 ]
v_title  = [ Encoded Title ]
subfile  = [ Encoded URL ]
startat  = [ Seconds (float) ]
referrer = [ Encoded URL ]
```

## Installation

### Linux

#### Arch Linux

[![mpv-handler][badges-aur]][download-aur]
[![mpv-handler-git][badges-aur-git]][download-aur-git]

#### Manual installation

1. Download [latest Linux release][download-linux]
2. Unzip the archive
3. Copy `mpv-handler` to `$HOME/.local/bin`
4. Copy `mpv-handler.desktop` to `$HOME/.local/share/applications/`
5. Copy `mpv-handler-debug.desktop` to `$HOME/.local/share/applications/`
6. Set executable permission for binary

   - ```
     $ chmod +x $HOME/.local/bin/mpv-handler
     ```

7. Register xdg-mime (thanks for the [linuxuprising][linuxuprising] reminder)

   - ```
     $ xdg-mime default mpv-handler.desktop x-scheme-handler/mpv-handler
     $ xdg-mime default mpv-handler-debug.desktop x-scheme-handler/mpv-handler-debug
     ```

8. Add `$HOME/.local/bin` to your environment variable `PATH`
9. **Optional**: _Copy `config.toml` to `$HOME/.config/mpv-handler/config.toml` and configure_

### Windows

Windows users need to install manually.

#### Manual installation

1. Download [latest Windows release][download-windows]
2. Unzip the archive to the directory you want
3. Run `handler-install.bat` to register protocol handler
4. Edit `config.toml` and set `mpv` and `ytdl` path

## Configuration

```toml
mpv = "/usr/bin/mpv"
# Optional, Type: String
# The path of mpv executable binary
# Default value:
# - Linux: mpv
# - Windows: mpv.com

ytdl = "/usr/bin/yt-dlp"
# Optional, Type: String
# The path of yt-dlp executable binary

proxy = "http://example.com:8080"
# Optional, Type: String
# HTTP(S) proxy server address

# For Windows users:
#   - The path can be "C:\\folder\\some.exe" or "C:/folder/some.exe"
#   - The path target is an executable binary file, not a directory
```

[v0.4.0]: https://github.com/akiirui/mpv-handler/releases/tag/v0.4.0
[rfc-base64-url]: https://datatracker.ietf.org/doc/html/rfc4648#section-5
[badges-aur-git]: https://img.shields.io/aur/version/mpv-handler-git?style=for-the-badge&logo=archlinux&label=mpv-handler-git
[badges-aur]: https://img.shields.io/aur/version/mpv-handler?style=for-the-badge&logo=archlinux&label=mpv-handler
[badges-play-with-mpv]: https://img.shields.io/greasyfork/v/416271?style=for-the-badge&logo=greasyfork&label=play-with-mpv
[download-aur-git]: https://aur.archlinux.org/packages/mpv-handler-git/
[download-aur]: https://aur.archlinux.org/packages/mpv-handler/
[download-linux]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-linux-amd64.zip
[download-macos]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-macos-amd64.zip
[download-windows]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-windows-amd64.zip
[greasyfork-play-with-mpv]: https://greasyfork.org/scripts/416271-play-with-mpv
[linuxuprising]: https://www.linuxuprising.com/2021/07/open-youtube-and-more-videos-from-your.html
