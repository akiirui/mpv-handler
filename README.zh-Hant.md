# mpv handler

一個爲 mpv 提供的協議處理程序，使用 Rust 編寫。

請配合用戶腳本使用：

[![play-with-mpv][badges-play-with-mpv]][greasyfork-play-with-mpv]

## 安裝

### Linux

- Arch Linux

  [![mpv-handler][badges-aur]][download-aur] \
  [![mpv-handler-git][badges-aur-git]][download-aur-git]

#### 手動安裝

1. 下載 [latest/mpv-handler-linux-x64.zip][download-linux]
2. 解壓縮壓縮包
3. 複製 `mpv-handler` 至 `$HOME/.local/bin`
4. 複製 `mpv-handler.desktop` 至 `$HOME/.local/share/applications/`
5. 複製 `config.toml` 至 `$HOME/.config/mpv-handler/`
6. 添加 `$HOME/.local/bin` 到環境變量 `PATH` 中（如果它沒在你的 `PATH` 中列出）
7. 註冊 xdg-mime（感謝 [linuxuprising][linuxuprising] 的提醒）

```
$ xdg-mime default mpv-handler.desktop x-scheme-handler/mpv
```

8. 如果需要，創建 `$HOME/.config/mpv-handler/custom.toml` 並按需更改

### Windows

Windows 用戶目前只能手動安裝 `mpv-handler`。

#### 手動安裝

1. 下載 [latest/mpv-handler-windows-x64.zip][download-windows]
2. 解壓縮檔案到你想要的文件夾裏（從 `v0.2.x` 起，不再需要和 `mpv` 安裝至同一個文件夾）
3. 運行 `handler-install.bat` 註冊協議處理程序
4. 在放置 `mpv-handler.exe` 的同一個目錄中創建 `custom.toml` 並按需更改

[badges-aur-git]: https://img.shields.io/aur/version/mpv-handler-git?label=mpv-handler-git&style=for-the-badge
[badges-aur]: https://img.shields.io/aur/version/mpv-handler?label=mpv-handler&style=for-the-badge
[badges-play-with-mpv]: https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=play-with-mpv&prefix=v&query=version&url=https%3A%2F%2Fgreasyfork.org%2Fscripts%2F416271.json
[download-aur-git]: https://aur.archlinux.org/packages/mpv-handler-git/
[download-aur]: https://aur.archlinux.org/packages/mpv-handler/
[download-linux]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-linux-x64.zip
[download-windows]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-windows-x64.zip
[greasyfork-play-with-mpv]: https://greasyfork.org/scripts/416271-play-with-mpv
[linuxuprising]: https://www.linuxuprising.com/2021/07/open-youtube-and-more-videos-from-your.html

## 協議 URL

基礎 URL：

```
mpv://BASE64_ENCODE_VIDEO_URL/
```

可選參數：

```
cookies     = [ www.domain.com.txt ]
downloader  = [ mpv, ytdl, you-get, streamlink, and more...] (default: mpv)
quality     = [ best, 2160p, 1440p, 1080p, 720p, 480p, 360p, and more... ]

c = cookies
d = downloader
q = quality
```

例：

```
mpv://aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj01cWFwNWFPNGk5QQ==/?cookies=www.youtube.com.txt&downloader=mpv&quality=best

mpv://aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj1wNVFmeUY5cGtIVQ==/?c=www.youtube.com.txt&d=mpv&q=best
```

## 自定義配置

默認的 `config.toml` 配置如下（已翻譯註釋爲中文）：

```toml
# 不要編輯此文件！
# 這是默認設置，並且它在 mpv-handler 更新時會被覆蓋。
#
# 對於自定義設置，創建並且編輯以下文件：
# - Linux:
#     - $HOME/.config/mpv-handler/custom.toml
#     - /etc/mpv-handler/custom.toml
#   如果找到了第一個，那麼第二個不會被加載。
# - Windows: custom.toml (在放置 mpv-handler.exe 的同一個目錄中)

### 播放器 ###
player = "/usr/bin/mpv"

### 視頻下載器 ###
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

一般來說，用戶只需要編輯 `player` 和所需下載器的 `bin` 至相應的可執行文件路徑。

爲此，用戶可以創建 `custom.toml` 來覆寫默認設置。

```toml
# 對於 Windows 用戶，
# 路徑格式可以是 "C:\\folder\\some.exe" 也可以是 "C:/folder/some.exe"
player = "/usr/bin/vlc"

# 警告：
# 開發者不建議用戶修改除了 "bin" 以外的默認下載器設置。
#
# 如果你修改了默認下載器的 "quality.LEVEL"，
# 你將丟失其他的來自默認設置的 "quality.LEVEL"。
[mpv]
bin = "/usr/local/bin/mpv"
quality.best = "--ytdl-format=best"

# 如果你是高級用戶，你可以手動添加其他的下載器。
#
# 例：
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

# [example]       必須，類型：字符串
#                     值 "example" 是下載器表的名稱。
# bin             必須，類型：字符串
#                     下載器可執行文件的路徑。
# cookies         可選，類型：字符串（默認：""）
#                     下載器傳遞 cookies 的參數。
# cookies_prefix  可選，類型：布爾值（默認：false）
#                     設置爲 ture 以標記 cookies 參數爲前綴。
# require_quality 可選，類型：布爾值（默認：false）
#                     設置爲 ture 以標記下載器需要一個 quality LEVEL。
# play_mode       可選, 類型：字符串 [normal, direct, pipe] （默認："normal")
#                     下載器的運行播放器的模式
# options         可選，類型：字符串數組（默認：[]）
#                     下載器設置播放器或者輸出位置的參數。
# player_options  可選，類型：字符串數組（默認：[]）
#                     用於特殊用途的播放器參數。
# quality.LEVEL   可選，類型：字符串
#                     LEVEL 是品質的關鍵詞
#                     它的值是下載器選擇品質或格式的參數。
```

### 下載器示例

參見 [share/examples][examples]。

[examples]: https://github.com/akiirui/mpv-handler/tree/main/share/examples
