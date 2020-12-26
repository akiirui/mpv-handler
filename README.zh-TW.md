# Play with mpv handler

通過 [mpv][mpv] & [youtube-dl][youtube-dl] 播放網頁上的視頻和歌曲，無需任何後臺服務。

## 安裝

### Both

- 安裝 [mpv][mpv-install] & [youtube-dl][youtube-dl]
- 安裝油猴腳本 [play-with-mpv][mpv-handler-greasyfork]

> 推薦 Windows 用戶安裝 [shinchiro's mpv builds][mpv-windows].
>
> 這個構建同時包含 `mpv` 和 `youtube-dl`，並且擁有更新它們的腳本。
>
> 在安裝這個構建後，記住運行 `updater.bat` 以更新 `mpv` 和下載最新的 `youtube-dl`。

### Linux

1. 安裝軟件包 [mpv-handler-git][mpv-handler-aur] <sup>AUR</sup>

### Windows

1. 在 [Release][mpv-handler-release] 下載最新的 `mpv-handler-windows-64.zip`
2. 解壓縮至 `mpv` 的安裝文件夾
3. 運行 `handler-install.bat` 註冊協議處理程序

## 使用

打開受支持的網站，點擊左下角的 mpv 圖標

盡情享受！

## 支持的網站

- YouTube (`*://www.youtube.com/*`)
- bilibili (`*://www.bilibili.com/video/*`)
- 還有更多的網站等待添加～

因爲 `youtube-dl` 支持了非常多的網站，我無法一次性添加所有的網站進 `@match`。

如果你需要某些網站的支持，歡迎提交 PR 或者創建一個 [Issues][mpv-handler-issues]。

當然，僅限於 `youtube-dl` [所支持的網站][youtube-dl-sites]。

## GitHub

- [mpv-handler][mpv-handler]

## 鳴謝

- [mpv][mpv]
- [youtube-dl][youtube-dl]
- [papirus-icon-theme][papirus-icon-theme]

[mpv-handler-aur]: https://aur.archlinux.org/packages/mpv-handler-git/
[mpv-handler-issues]: https://github.com/akiirui/mpv-handler/issues/new
[mpv-handler-release]: https://github.com/akiirui/mpv-handler/releases
[mpv-handler-greasyfork]: https://greasyfork.org/scripts/416271-play-with-mpv
[mpv-handler]: https://github.com/akiirui/mpv-handler/
[mpv-install]: https://mpv.io/installation/
[mpv-windows]: https://sourceforge.net/projects/mpv-player-windows/files
[mpv]: https://mpv.io/
[papirus-icon-theme]: https://github.com/PapirusDevelopmentTeam/papirus-icon-theme/
[youtube-dl-sites]: https://ytdl-org.github.io/youtube-dl/supportedsites.html
[youtube-dl]: https://github.com/ytdl-org/youtube-dl/
