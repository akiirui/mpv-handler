# mpv handler

一个为 mpv 提供的协议处理程序，使用 Rust 编写。

请配合用户脚本使用：

[![play-with-mpv-handler][play-with-mpv-badges]][play-with-mpv-greasyfork]

## 安装

### Linux

- Arch Linux
  - [mpv-handler][mpv-handler-download-aur] <sup>AUR</sup>
  - [mpv-handler-git][mpv-handler-download-aur-git] <sup>AUR</sup>

**不要忘记复制 `/usr/share/mpv-handler/mpv-handler.toml` 至 `~/.config/mpv/`。**

#### 手动安装

1. 下载 [latest/mpv-handler-linux-x64.zip][mpv-handler-download-linux]
2. 解压缩压缩包
3. 复制 `mpv-handler` 至 `~/.local/bin`
4. 复制 `mpv-handler.desktop` 至 `~/.local/share/applications/`
5. 复制 `mpv-handler.toml` 至 `~/.config/mpv/`
6. 添加 `~/.local/bin` 到环境变量 `PATH` 中（如果它没在你的 `PATH` 中列出）
7. 注册 xdg-mime（感谢 [linuxuprising][linuxuprising] 的提醒）

```
$ xdg-mime default mpv-handler.desktop x-scheme-handler/mpv
```

8. 检查 `~/.config/mpv/mpv-handler.toml` 并按需更改

### Windows

Windows 用户目前只能手动安装 `mpv-handler`。

#### 手动安装

1. 下载 [latest/mpv-handler-windows-x64.zip][mpv-handler-download-windows]
2. 解压缩档案到你想要的文件夹里（从 `v0.2.x` 起，不再需要和 `mpv` 安装至同一个文件夹）
3. 运行 `handler-install.bat` 注册协议处理程序
4. 检查 `~/.config/mpv/mpv-handler.toml` 并按需更改

[mpv-handler-download-aur]: https://aur.archlinux.org/packages/mpv-handler/
[mpv-handler-download-aur-git]: https://aur.archlinux.org/packages/mpv-handler-git/
[mpv-handler-download-linux]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-linux-x64.zip
[mpv-handler-download-windows]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-windows-x64.zip
[play-with-mpv-badges]: https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=play-with-mpv&prefix=v&query=version&url=https%3A%2F%2Fgreasyfork.org%2Fscripts%2F416271.json
[play-with-mpv-greasyfork]: https://greasyfork.org/scripts/416271-play-with-mpv
[play-with-mpv-github]: https://github.com/akiirui/userscript/tree/main/play-with-mpv-handler
[linuxuprising]: https://www.linuxuprising.com/2021/07/open-youtube-and-more-videos-from-your.html

## 协议 URL

基础 URL：

```
mpv://BASE64_ENCODE_VIDEO_URL/
```

可选参数：

```
cookies     = [ www.domain.com.txt ]
downloader  = [ mpv, ytdl, you-get, streamlink, and more...] (default: mpv)
quality     = [ best, 4k, 2k, 1080p, 720p, 480p, 360p, and more... ]
```

例：

```
mpv://aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj01cWFwNWFPNGk5QQ==/?cookies=www.youtube.com.txt&downloader=mpv&quality=best
```

## 自定义配置

一般来说，用户只需要编辑 `player` 和所需下载器的 `bin` 至相应的可执行文件路径。

默认的 `mpv-handler.toml` 配置如下（已翻译注释为中文）：

```toml
### 播放器 ###
# 你应当修改 "player" 的值至你的播放器的可执行文件路径。
player = "/usr/bin/mpv"

### 视频下载器 ###
# 你应当修改 "bin" 的值至你的下载器的可执行文件路径。
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
require_quality = true
options = ["--player"]
quality.best = "best"
quality.worst = "worst"

# 如果你是高级用户，你可以手动添加其他的下载器。
# 例：
#
# [example]
# bin = "/usr/bin/example"
# cookies = "--cookies"
# cookies_prefix = false
# direct = false
# pipeline = false
# require_quality = false
# options = ["--player"]
# quality.best = "--quality=best"
#
#
# [example]       必须，类型：字符串
#                     值 "example" 是下载器表的名称。
# bin             必须，类型：字符串
#                     下载器可执行文件的路径。
# cookies         可选，类型：字符串（默认：""）
#                     下载器传递 cookies 的参数。
# cookies_prefix  可选，类型：布尔值（默认：false）
#                     设置为 ture 以标记 cookies 参数为前缀。
# direct          可选，类型：布尔值（默认：false）
#                     设置为 ture 以标记下载器可直接运行，不需要播放器。
# pipeline        可选，类型：布尔值（默认：false）
#                     设置为 ture 以标记下载器通过管道传递视频数据。
# require_quality 可选，类型：布尔值（默认：false）
#                     设置为 ture 以标记下载器需要一个 quality LEVEL。
# options         可选，类型：字符串数组（默认：[]）
#                     下载器设置播放器或者输出位置的参数。
# quality.LEVEL   可选，类型：字符串
#                     LEVEL 是品质的关键词
#                     它的值是下载器选择品质或格式的参数。
```
