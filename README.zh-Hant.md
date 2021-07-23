# mpv handler

一個爲 mpv 提供的協議處理程序，使用 Rust 編寫。

請配合用戶腳本使用：

[![play-with-mpv-handler][play-with-mpv-badges]][play-with-mpv-greasyfork ]

## 安裝

### Linux

`mpv-handler.toml` 需要放置在 `$HOME/.config/mpv/mpv-handler.toml`。

- Arch Linux
  - [mpv-handler][mpv-handler-download-aur] <sup>AUR</sup>
  - [mpv-handler-git][mpv-handler-download-aur-git] <sup>AUR</sup>
- GitHub Actions Build
  - [latest/mpv-handler-linux-x64.zip][mpv-handler-download-linux]

### Windows

`mpv-handler.toml` 需要和 `mpv-handler.exe` 放置在同一個目錄。

**Windows 用戶請不要忘記參照註釋編輯壓縮包附帶的 `mpv-handler.toml`。**

- GitHub Actions Build
  - [latest/mpv-handler-windows-x64.zip][mpv-handler-download-windows]

[mpv-handler-download-aur]: https://aur.archlinux.org/packages/mpv-handler/
[mpv-handler-download-aur-git]: https://aur.archlinux.org/packages/mpv-handler-git/
[mpv-handler-download-linux]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-linux-x64.zip
[mpv-handler-download-windows]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-windows-x64.zip
[play-with-mpv-badges]: https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=play-with-mpv&prefix=v&query=version&url=https%3A%2F%2Fgreasyfork.org%2Fscripts%2F416271.json
[play-with-mpv-greasyfork]: https://greasyfork.org/scripts/416271-play-with-mpv
[play-with-mpv-github]: https://github.com/akiirui/userscript/tree/main/play-with-mpv-handler

## 協議 URL

基礎 URL：

```
mpv://BASE64_ENCODE_VIDEO_URL/
```

可選參數：

```
cookies     = [ www.domain.com.txt ]
downloader  = [ mpv, ytdl, you-get, streamlink, and more...] (default: mpv)
quality     = [ best, 4k, 2k, 1080p, 720p, 480p, 360p, and more... ]
```

例：

```
mpv://aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj01cWFwNWFPNGk5QQ==/?cookies=www.youtube.com.txt&downloader=mpv&quality=best
```

## 自定義配置

一般來說，用戶只需要編輯 `player` 和所需下載器的 `bin` 至相應的可執行文件路徑。

截止 `v0.2.3`，默認的 `mpv-handler.toml` 配置如下（已翻譯註釋爲中文）：

```toml
### 播放器 ###
# 你應當修改 "player" 的值至你的播放器的可執行文件路徑。
player = "/usr/bin/mpv"

### 視頻下載器 ###
# 你應當修改 "bin" 的值至你的下載器的可執行文件路徑。
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
options = ["--player"]

# 如果你是高級用戶，你可以手動添加其他的下載器。
# 例：
#
# [example]
# bin = "/usr/bin/example"
# cookies = "--cookies"
# cookies_prefix = false
# direct = false
# pipeline = false
# options = ["--player"]
# quality.best = "--quality=best"
#
#
# [example]       必須，類型：字符串
#                     值 "example" 是下載器表的名稱。
# bin             必須，類型：字符串
#                     下載器可執行文件的路徑。
# cookies         可選，類型：字符串（默認：""）
#                     下載器傳遞 cookies 的參數。
# cookies_prefix  可選，類型：布爾值（默認：false）
#                     設置爲 ture 標記 cookies 參數爲前綴。
# direct          可選，類型：布爾值（默認：false）
#                     設置爲 ture 標記下載器可直接運行，不需要播放器。
# pipeline        可選，類型：布爾值（默認：false）
#                     設置爲 ture 標記下載器通過管道傳遞視頻數據。
# options         可選，類型：字符串數組（默認：[]）
#                     下載器設置播放器或者輸出位置的參數。
# quality.LEVEL   可選，類型：字符串
#                     LEVEL 是品質選擇的關鍵詞
#                     它的值是下載器選擇品質或格式的參數。
```
