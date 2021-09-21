[English][readme-en] | [简体中文][readme-zh-hans] | [繁体中文][readme-zh-hant]

[readme-en]: https://github.com/akiirui/mpv-handler/blob/main/README.md
[readme-zh-hans]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hans.md
[readme-zh-hant]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hant.md

# mpv handler

一个为 mpv 提供的协议处理程序，使用 Rust 编写。

请配合用户脚本使用：

[![play-with-mpv][badges-play-with-mpv]][greasyfork-play-with-mpv]

## 协议 URL

基础 URL：

```
mpv://play/BASE64_ENCODE_VIDEO_URL/
```

可选参数：

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
mpv://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj1HZ2tuMmY1ZS1JVQ==/?cookies=www.youtube.com.txt&downloader=mpv&quality=best

mpv://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj1EcnZ1c29zekJLQQ==/?c=www.youtube.com.txt&d=mpv&q=best
```

## 安装

### Linux

- Arch Linux

  [![mpv-handler][badges-aur]][download-aur] \
  [![mpv-handler-git][badges-aur-git]][download-aur-git]

#### 手动安装

1. 下载 [latest/mpv-handler-linux-x64.zip][download-linux]
2. 解压缩压缩包
3. 复制 `mpv-handler` 至 `$HOME/.local/bin`
4. 复制 `mpv-handler.desktop` 至 `$HOME/.local/share/applications/`
5. 复制 `config.toml` 至 `$HOME/.config/mpv-handler/`
6. 添加 `$HOME/.local/bin` 到环境变量 `PATH` 中（如果它没在你的 `PATH` 中列出）
7. 注册 xdg-mime（感谢 [linuxuprising][linuxuprising] 的提醒）

```
$ xdg-mime default mpv-handler.desktop x-scheme-handler/mpv
```

8. 如果需要，创建 `$HOME/.config/mpv-handler/custom.toml` 并按需更改

### Windows

Windows 用户目前只能手动安装 `mpv-handler`。

#### 手动安装

1. 下载 [latest/mpv-handler-windows-x64.zip][download-windows]
2. 解压缩档案到你想要的文件夹里（从 `v0.2.x` 起，不再需要和 `mpv` 安装至同一个文件夹）
3. 运行 `handler-install.bat` 注册协议处理程序
4. 在放置 `mpv-handler.exe` 的同一个目录中创建 `custom.toml` 并按需更改

[badges-aur-git]: https://img.shields.io/aur/version/mpv-handler-git?label=mpv-handler-git&style=for-the-badge
[badges-aur]: https://img.shields.io/aur/version/mpv-handler?label=mpv-handler&style=for-the-badge
[badges-play-with-mpv]: https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=play-with-mpv&prefix=v&query=version&url=https%3A%2F%2Fgreasyfork.org%2Fscripts%2F416271.json
[download-aur-git]: https://aur.archlinux.org/packages/mpv-handler-git/
[download-aur]: https://aur.archlinux.org/packages/mpv-handler/
[download-linux]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-linux-x64.zip
[download-windows]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-windows-x64.zip
[greasyfork-play-with-mpv]: https://greasyfork.org/scripts/416271-play-with-mpv
[linuxuprising]: https://www.linuxuprising.com/2021/07/open-youtube-and-more-videos-from-your.html

## 配置

## 默认配置

```toml
# 不要编辑此文件！
# 这是默认设置，它会在 mpv-handler 更新时会被覆盖
#
# 对于自定义设置，创建并且编辑以下文件：
# - Linux:
#     - $HOME/.config/mpv-handler/custom.toml
#     - /etc/mpv-handler/custom.toml
#     如果找到了第一个，那么第二个不会被加载
# - Windows: custom.toml
#     在放置 mpv-handler.exe 的同一个目录中

### 播放器 ###
player = "/usr/bin/mpv"

### 视频下载器 ###
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

### 自定义配置

一般来说，用户只需编辑 `player` 和下载器的 `bin` 至相应的可执行文件路径。

为此，用户可以创建 `custom.toml` 来覆盖默认设置：

```toml
player = "/usr/bin/vlc"
ld_path = "/usr/lib/:/usr/local/lib"

# player    必须，类型：字符串
#             播放器可执行文件的路径
#             对于 Windows 用户：
#             路径格式可以是 "C:\\folder\\some.exe"，也可以是 "C:/folder/some.exe"
# ld_path   可选，类型：字符串（仅适用于 Linux）
#             为播放器和下载器设置环境变量 "LD_LIBRARY_PATH"
#             使用 ":" 来分割多个路径
#             更多有关此选项的详情，见：
#             https://github.com/akiirui/mpv-handler/commit/4ad298ddd82bc3fa0303f8cc11474df506531d33

# 警告：
# 不建议用户修改除了 "bin" 以外的默认下载器设置
#
# 如果你修改了默认下载器的 "quality.LEVEL"
# 你将丢失其他的来自默认设置的 "quality.LEVEL"
[mpv]
bin = "/usr/local/bin/mpv"

# bin       必须，类型：字符串
#             下载器可执行文件的路径
#             对于 Windows 用户：
#             路径格式可以是 "C:\\folder\\some.exe"，也可以是 "C:/folder/some.exe"
```

### 自定义下载器

默认情况下，`mpv-handler` 只有一个下载器 `mpv`。

如果用户出于特殊目的需要其他下载器，则需要添加自定义下载器。

```toml
# 如果你是高级用户，你可以手动添加其他的下载器
# 将你的自定义下载器添加到 "custom.toml" 中
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

# [example]       必须，类型：字符串
#                   值 "example" 是下载器表的名称
# bin             必须，类型：字符串
#                   下载器可执行文件的路径
#                   对于 Windows 用户：
#                   路径格式可以是 "C:\\folder\\some.exe"，也可以是 "C:/folder/some.exe"
# cookies         可选，类型：字符串（默认：""）
#                   下载器传递 cookies 的参数
# cookies_prefix  可选，类型：布尔值（默认：false）
#                   设置为 ture 以标记 cookies 参数为前缀
# require_quality 可选，类型：布尔值（默认：false）
#                   设置为 ture 以标记下载器需要一个 quality LEVEL
# play_mode       可选, 类型：字符串 [normal, direct, pipe] （默认："normal")
#                   下载器的运行播放器的模式
# options         可选，类型：字符串数组（默认：[]）
#                   下载器设置播放器或者输出位置的参数
# player_options  可选，类型：字符串数组（默认：[]）
#                   用于特殊用途的播放器参数
# quality.LEVEL   可选，类型：字符串
#                   LEVEL 是品质的关键词
#                   它的值是下载器选择品质或格式的参数
```

### 下载器示例

参见 [share/examples][examples]。

欢迎分享你的自定义下载器！

[examples]: https://github.com/akiirui/mpv-handler/tree/main/share/examples
