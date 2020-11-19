# Play with mpv handler

通過 [mpv](https://mpv.io/) & [youtube-dl](https://github.com/ytdl-org/youtube-dl/) 播放網頁上的視頻和歌曲，無需任何後臺服務。

## 安裝與使用

### Both

- 安裝 [mpv](https://mpv.io/installation/) & [youtube-dl](https://github.com/ytdl-org/youtube-dl/)
- 安裝油猴腳本 [play-with-mpv](https://greasyfork.org/scripts/416271-play-with-mpv)

> 推薦 Windows 用戶安裝 [shinchiro's mpv builds](https://sourceforge.net/projects/mpv-player-windows/files).

這個構建同時包含 `mpv` 和 `youtube-dl`，並且擁有更新它們的腳本。

### Linux

1. 安裝軟件包 [mpv-handler-git](https://aur.archlinux.org/packages/mpv-handler-git/) <sup>AUR</sup>
2. 打開受支持的網站，點擊左下角的 mpv 圖標

### Windows

1. 複製 [`windows/handler-install.bat`](https://github.com/akiirui/play-with-mpv-handler/tree/main/windows/handler-install.bat) & [`windows/mpv-handler.bat`](https://github.com/akiirui/play-with-mpv-handler/tree/main/windows/mpv-handler.bat) 到 `mpv` 的安裝文件夾內
2. 運行 `handler-install.bat` 註冊協議處理程序
3. 打開受支持的網站，點擊左下角的 mpv 圖標

## 支持的網站

- YouTube (`*://www.youtube.com/*`)
- bilibili (`*://www.bilibili.com/video/*`)
- 還有更多的網站等待添加～

因爲 `youtube-dl` 支持了非常多的網站，我無法一次性添加所有的網站進 `@match`。

如果你需要某些網站的支持，歡迎提交 PR 或者創建一個 [Issues](https://github.com/akiirui/play-with-mpv-handler/issues/new)。

當然，僅限於 `youtube-dl` [所支持的網站](https://ytdl-org.github.io/youtube-dl/supportedsites.html)。

## GitHub

- [play-with-mpv-handler](https://github.com/akiirui/play-with-mpv-handler/)

## 鳴謝

- [mpv](https://mpv.io/)
- [youtube-dl](https://github.com/ytdl-org/youtube-dl/)
- [papirus-icon-theme](https://github.com/PapirusDevelopmentTeam/papirus-icon-theme)
