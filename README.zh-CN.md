# Play with mpv handler

通过 [mpv][mpv] & [youtube-dl][youtube-dl] 播放网页上的视频和歌曲，无需任何后台服务。

## 安装

### Both

- 安装 [mpv][mpv-install] & [youtube-dl][youtube-dl]
- 安装油猴脚本 [play-with-mpv][mpv-handler-greasyfork]

> 推荐 Windows 用户安装 [shinchiro's mpv builds][mpv-windows].
>
> 这个构建同时包含 `mpv` 和 `youtube-dl`，并且拥有更新它们的脚本。
>
> 在安装这个构建后，记住运行 `updater.bat` 以更新 `mpv` 和下载最新的 `youtube-dl`。

### Linux

1. 安装软件包 [mpv-handler-git][mpv-handler-aur] <sup>AUR</sup>

### Windows

1. 下载最新的 [`mpv-handler-windows-64.zip`][mpv-handler-release]
2. 解压缩至 `mpv` 的安装文件夹
3. 运行 `handler-install.bat` 注册协议处理程序

## 使用

打开受支持的网站，点击左下角的 mpv 图标

尽情享受！

## 支持的网站

- YouTube (`*://www.youtube.com/*`)
- bilibili (`*://www.bilibili.com/video/*`)
- 还有更多的网站等待添加～

因为 `youtube-dl` 支持了非常多的网站，我无法一次性添加所有的网站进 `@match`。

如果你需要某些网站的支持，欢迎提交 PR 或者创建一个 [Issues][mpv-handler-issues]。

当然，仅限于 `youtube-dl` [所支持的网站][youtube-dl-sites]。

## GitHub

- [mpv-handler][mpv-handler]

## 鸣谢

- [mpv][mpv]
- [youtube-dl][youtube-dl]
- [papirus-icon-theme][papirus-icon-theme]

[mpv-handler-aur]: https://aur.archlinux.org/packages/mpv-handler-git/
[mpv-handler-issues]: https://github.com/akiirui/mpv-handler/issues/new
[mpv-handler-release]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-windows-64.zip
[mpv-handler-greasyfork]: https://greasyfork.org/scripts/416271-play-with-mpv
[mpv-handler]: https://github.com/akiirui/mpv-handler/
[mpv-install]: https://mpv.io/installation/
[mpv-windows]: https://sourceforge.net/projects/mpv-player-windows/files
[mpv]: https://mpv.io/
[papirus-icon-theme]: https://github.com/PapirusDevelopmentTeam/papirus-icon-theme/
[youtube-dl-sites]: https://ytdl-org.github.io/youtube-dl/supportedsites.html
[youtube-dl]: https://github.com/ytdl-org/youtube-dl/
