# Play with mpv handler

通过 [mpv](https://mpv.io/) & [youtube-dl](https://github.com/ytdl-org/youtube-dl/) 播放网页上的视频和歌曲，无需任何后台服务。

## 安装与使用

### Both

- 安装 [mpv](https://mpv.io/installation/) & [youtube-dl](https://github.com/ytdl-org/youtube-dl/)
- 安装油猴脚本 [play-with-mpv](https://greasyfork.org/scripts/416271-play-with-mpv)

> 推荐 Windows 用户安装 [shinchiro's mpv builds](https://sourceforge.net/projects/mpv-player-windows/files).

这个构建同时包含 `mpv` 和 `youtube-dl`，并且拥有更新它们的脚本。

### Linux

1. 安装软件包 [mpv-handler-git](https://aur.archlinux.org/packages/mpv-handler-git/) <sup>AUR</sup>
2. 打开受支持的网站，点击左下角的 mpv 图标

### Windows

1. 复制 [`windows/handler-install.bat`](https://github.com/akiirui/play-with-mpv-handler/tree/main/windows/handler-install.bat) & [`windows/mpv-handler.bat`](https://github.com/akiirui/play-with-mpv-handler/tree/main/windows/mpv-handler.bat) 到 `mpv` 的安装文件夹内
2. 运行 `handler-install.bat` 注册协议处理程序
3. 打开受支持的网站，点击左下角的 mpv 图标

## 支持的网站

- YouTube (`*://www.youtube.com/*`)
- bilibili (`*://www.bilibili.com/video/*`)
- 还有更多的网站等待添加～

因为 `youtube-dl` 支持了非常多的网站，我无法一次性添加所有的网站进 `@match`。

如果你需要某些网站的支持，欢迎提交 PR 或者创建一个 [Issues](https://github.com/akiirui/play-with-mpv-handler/issues/new)。

当然，仅限于 `youtube-dl` [所支持的网站](https://ytdl-org.github.io/youtube-dl/supportedsites.html)。

## GitHub

- [play-with-mpv-handler](https://github.com/akiirui/play-with-mpv-handler/)

## 鸣谢

- [mpv](https://mpv.io/)
- [youtube-dl](https://github.com/ytdl-org/youtube-dl/)
- [papirus-icon-theme](https://github.com/PapirusDevelopmentTeam/papirus-icon-theme)
