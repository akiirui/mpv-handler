[English][readme-en] | [簡體中文][readme-zh-hans] | [繁體中文][readme-zh-hant]

[readme-en]: https://github.com/akiirui/mpv-handler/blob/main/README.md
[readme-zh-hans]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hans.md
[readme-zh-hant]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hant.md

# mpv handler

一個 **mpv** 的協議處理程序，使用 Rust 編寫。

使用 **mpv** 和 **yt-dlp** 播放網站上的視頻與音樂。

請配合用戶腳本使用：

[![play-with-mpv][badges-play-with-mpv]][greasyfork-play-with-mpv]

## 協議

![](share/proto.png)

### 協議名

- `mpv`: 在沒有命令行窗口的情況下運行 mpv-handler
- `mpv-debug`: 在有命令行窗口的情況下運行 mpv-handler 以便於查看輸出和錯誤

### 插件 / Plugins

- `play`: 使用 mpv 播放視頻

### 編碼的網址 / Encoded URL

使用 [URL 安全的 base64][rfc-base64-url] 編碼網址。

替換 `/` 至 `_`, `+` 至 `-` 並且刪除填充的 `=`。

示例 (JavaScript):

```javascript
let data = btoa("https://www.youtube.com/watch?v=Ggkn2f5e-IU");
let safe = data.replace(/\//g, "_").replace(/\+/g, "-").replace(/\=/g, "");
```

### 參數 / Parameters (可選)

```
cookies = [ www.domain.com.txt ]
profile = [ default, low-latency, etc... ]
quality = [ 2160p, 1440p, 1080p, 720p, 480p, 360p ]
v_codec = [ av01, vp9, h265, h264 ]
subfile = [ Encoded URL ]
title = [ Player Window Title, encoded with JavaScript fucntion `encodeURIComponent()` ]
```

## 安裝

### Linux

- Arch Linux

  [![mpv-handler][badges-aur]][download-aur] \
  [![mpv-handler-git][badges-aur-git]][download-aur-git]

#### 手動安裝

1. 下載 [最新的 Linux 壓縮包][download-linux]
2. 解壓縮壓縮包
3. 複製 `mpv-handler` 至 `$HOME/.local/bin`
4. 複製 `mpv-handler.desktop` 至 `$HOME/.local/share/applications/`
5. 複製 `mpv-handler-debug.desktop` 至 `$HOME/.local/share/applications/`
6. 爲二進制文件設置可執行權限

   - ```
     $ chmod +x $HOME/.local/bin/mpv-handler
     ```

7. 註冊 xdg-mime（感謝 [linuxuprising][linuxuprising] 的提醒）

   - ```
     $ xdg-mime default mpv-handler.desktop x-scheme-handler/mpv
     $ xdg-mime default mpv-handler-debug.desktop x-scheme-handler/mpv-debug
     ```

8. 添加 `$HOME/.local/bin` 到環境變量 `PATH`
9. **可選**: _複製 `config.toml` 至 `$HOME/.config/mpv-handler/config.toml` 並配置_

### Windows

Windows 用戶目前只能手動安裝。

#### 手動安裝

1. 下載 [最新的 Windows 壓縮包][download-windows]
2. 解壓縮檔案到你想要的位置
3. 運行 `handler-install.bat` 註冊協議處理程序
4. 編輯 `config.toml` 設置 `mpv` 和 `ytdl` 的路徑

## 配置

```toml
mpv = "/usr/bin/mpv"
# 可選，類型：字符串
# mpv 可執行文件的路徑
# 默認值:
# - Linux: mpv
# - Windows: mpv.com

ytdl = "/usr/bin/yt-dlp"
# 可選，類型：字符串
# yt-dlp 可執行文件的路徑

proxy = "http://example.com:8080"
# 可選，類型：字符串
# HTTP(S) 代理服務器的地址

# 對於 Windows 用戶：
#   - 路徑格式可以是 "C:\\folder\\some.exe"，也可以是 "C:/folder/some.exe"
#   - 路徑的目標是可執行二進制文件，而不是目錄
```

[rfc-base64-url]: https://datatracker.ietf.org/doc/html/rfc4648#section-5
[badges-aur-git]: https://img.shields.io/aur/version/mpv-handler-git?label=mpv-handler-git&style=for-the-badge
[badges-aur]: https://img.shields.io/aur/version/mpv-handler?label=mpv-handler&style=for-the-badge
[badges-play-with-mpv]: https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=play-with-mpv&prefix=v&query=version&url=https%3A%2F%2Fgreasyfork.org%2Fscripts%2F416271.json
[download-aur-git]: https://aur.archlinux.org/packages/mpv-handler-git/
[download-aur]: https://aur.archlinux.org/packages/mpv-handler/
[download-linux]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-linux-amd64.zip
[download-macos]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-macos-amd64.zip
[download-windows]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-windows-amd64.zip
[greasyfork-play-with-mpv]: https://greasyfork.org/scripts/416271-play-with-mpv
[linuxuprising]: https://www.linuxuprising.com/2021/07/open-youtube-and-more-videos-from-your.html
