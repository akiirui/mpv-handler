[English][readme-en] | [簡體中文][readme-zh-hans] | [繁體中文][readme-zh-hant]

[readme-en]: https://github.com/akiirui/mpv-handler/blob/main/README.md
[readme-zh-hans]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hans.md
[readme-zh-hant]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hant.md

# mpv handler

一個 **mpv** 的協議處理程序，使用 Rust 編寫。

使用 **mpv** 和 **yt-dlp** 播放網站上的視頻與音樂。

請配合用戶腳本使用：

[![play-with-mpv][badges-play-with-mpv]][greasyfork-play-with-mpv]

## 重大變更

### [v0.4.0][v0.4.0]

爲了避免與 mpv 所提供的 `mpv://` 協議衝突。

> mpv://...
>
> mpv protocol. This is used for starting mpv from URL handler. The protocol is stripped and the rest is passed to the player as a normal open argument. Only safe network protocols are allowed to be opened this way.

協議 `mpv://` 和 `mpv-debug://` 已棄用, 請使用 `mpv-handler://` 和 `mpv-handler-debug://`.

**需要手動干預**

#### Windows

運行 `handler-uninstall.bat` 卸載已棄用的協議, 然後運行 `handler-install.bat` 安裝新的協議.

#### Linux

如果你是手動安裝的，請重新執行一遍手動安裝流程。

## 協議

![](share/proto.png)

### 協議名

- `mpv-handler`: 在沒有命令行窗口的情況下運行 mpv-handler
- `mpv-handler-debug`: 在有命令行窗口的情況下運行 mpv-handler 以便於查看輸出和錯誤

### 插件 / Plugins

- `play`: 使用 mpv 播放視頻

### 編碼數據 / Encoded Data

使用 [URL 安全的 base64][rfc-base64-url] 編碼網址或標題。

替換 `/` 至 `_`, `+` 至 `-` 並且刪除填充的 `=`。

示例 (JavaScript):

```javascript
let data = btoa("https://www.youtube.com/watch?v=Ggkn2f5e-IU");
let safe = data.replace(/\//g, "_").replace(/\+/g, "-").replace(/\=/g, "");
```

### 參數 / Parameters (可選)

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

## 安裝

### Linux

#### Arch Linux

[![mpv-handler][badges-aur]][download-aur]
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

7. 註冊 xdg-mime（感謝 [linuxuprising][linuxuprising] 和 [muse90673#122][muse90673]）
   - ```
     $ xdg-mime default mpv-handler.desktop x-scheme-handler/mpv-handler
     $ xdg-mime default mpv-handler-debug.desktop x-scheme-handler/mpv-handler-debug
     $ update-desktop-database $HOME/.local/share/applications/
     ```

8. 添加 `$HOME/.local/bin` 到環境變量 `PATH`
9. **可選**: _複製 `config.toml` 至 `$HOME/.config/mpv-handler/config.toml` 並配置_

### macOS

macOS 無法把裸的可執行文件註冊爲協議處理程序，因此必須把它包進一個 `.app`
bundle。`install.sh` 會組裝這個 bundle、做 ad-hoc 簽名，並向 LaunchServices 註冊。

1. 安裝 **mpv** 和 **yt-dlp**
   - ```
     $ brew install mpv yt-dlp
     ```

2. 依晶片架構解壓縮 macOS 安裝包。在含有 macOS 建置流程的分支上手動執行 GitHub Actions 後，
   可從建置產物 `macos-packages` 取得這兩個壓縮檔。

   - Apple Silicon — `unzip mpv-handler-macos-arm64.zip -d mpv-handler-macos`
   - Intel — `unzip mpv-handler-macos-amd64.zip -d mpv-handler-macos`

3. 進入解壓縮後的目錄並執行安裝腳本
   - ```
     $ cd mpv-handler-macos
     $ sh install.sh ./mpv-handler
     ```

   bundle 會被安裝到 `$HOME/Applications`，該位置不需要管理員權限。傳入第二個
   參數可爲本機的所有用戶安裝
   - ```
     $ sh install.sh ./mpv-handler /Applications
     ```

4. **可選**: _複製 `config.toml` 至 `${XDG_CONFIG_HOME:-$HOME/.config}/mpv-handler/config.toml` 並配置_

個人安裝時，如果 shell 中的 `$XDG_CONFIG_HOME` 是絕對路徑，`install.sh` 會保存該路徑，
供從瀏覽器開啓連結時使用。設置、更改或取消該變量後，請重新運行 `install.sh`。
共享安裝在 `/Applications` 時，應用啓動後從當前用戶的環境讀取 `$XDG_CONFIG_HOME`。
如果它是絕對路徑，就使用該路徑，否則使用該用戶的 `~/.config`。

從原始碼安裝時，請使用含有 `share/macos/install.sh` 的原始碼版本。
在專案根目錄執行 `cargo build --release --locked`，再執行 `sh share/macos/install.sh`。

卸載運行 `sh uninstall.sh`；如果安裝目錄不是 `$HOME/Applications`，把它作爲參數
傳入。腳本會先註銷再刪除，這個順序很重要：LaunchServices 會爲已不存在的路徑保留
一條協議聲明，而這條殘留聲明的解析優先級高於正常安裝，會讓協議失效。

開啟 `mpv-handler-debug://` 連結時，系統會為這次播放啟動獨立的「終端機」程序，
直接執行 mpv-handler。視窗顯示 handler 和 mpv 的輸出及錯誤訊息，正常播放的首行是
`Binaries:`。以下測試連結的編碼資料不完整，會顯示
`Failed to decode`。

```sh
open 'mpv-handler-debug://play/A'
```

從 LaunchServices 啟動時，mpv-handler 會在繼承的 `PATH` 中追加
`/opt/homebrew/bin`、`/usr/local/bin`、`/opt/local/bin` 和 `$HOME/.local/bin`，
以便找到透過 Homebrew 或 MacPorts 安裝的程式。也可以在 `config.toml` 中指定
`mpv` 和 `ytdl` 的路徑。

下載的壓縮包帶有 `com.apple.quarantine` 屬性，`install.sh` 會在組裝 bundle 時清除
它，因此 Gatekeeper 不會報告應用已損壞。簽名是 ad-hoc 的：在 Apple Silicon 上本地
運行足夠，但 bundle 並未經過公證。

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
# - macOS: mpv
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

[v0.4.0]: https://github.com/akiirui/mpv-handler/releases/tag/v0.4.0
[rfc-base64-url]: https://datatracker.ietf.org/doc/html/rfc4648#section-5
[badges-aur-git]: https://img.shields.io/aur/version/mpv-handler-git?style=for-the-badge&logo=archlinux&label=mpv-handler-git
[badges-aur]: https://img.shields.io/aur/version/mpv-handler?style=for-the-badge&logo=archlinux&label=mpv-handler
[badges-play-with-mpv]: https://img.shields.io/greasyfork/v/416271?style=for-the-badge&logo=greasyfork&label=play-with-mpv
[download-aur-git]: https://aur.archlinux.org/packages/mpv-handler-git/
[download-aur]: https://aur.archlinux.org/packages/mpv-handler/
[download-linux]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-linux-amd64.zip
[download-windows]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-windows-amd64.zip
[greasyfork-play-with-mpv]: https://greasyfork.org/scripts/416271-play-with-mpv
[linuxuprising]: https://www.linuxuprising.com/2021/07/open-youtube-and-more-videos-from-your.html
[muse90673]: https://github.com/akiirui/mpv-handler/issues/122
