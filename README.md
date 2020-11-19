# Play with mpv handler

Play website videos and songs with [mpv](https://mpv.io/) & [youtube-dl](https://github.com/ytdl-org/youtube-dl/), without any background service.

## Install & Usage

### Both

- Install [mpv](https://mpv.io/installation/) & [youtube-dl](https://github.com/ytdl-org/youtube-dl/)
- Install userscript [play-with-mpv](https://greasyfork.org/scripts/416271-play-with-mpv)

> For Windows users, recommend to install [shinchiro's mpv builds](https://sourceforge.net/projects/mpv-player-windows/files).

This build include `mpv` and `youtube-dl` both, and have a script to update these.

### Linux

1. Install package [mpv-handler-git](https://aur.archlinux.org/packages/mpv-handler-git/) <sup>AUR</sup>
2. Open supported sites and click left-bottom mpv icon

### Windows

1. Copy [`windows/handler-install.bat`](https://github.com/akiirui/play-with-mpv-handler/tree/main/windows/handler-install.bat) & [`windows/mpv-handler.bat`](https://github.com/akiirui/play-with-mpv-handler/tree/main/windows/mpv-handler.bat) to `mpv` installation folder
2. Run `handler-install.bat` to register protocol handler
3. Open supported sites and click left-bottom mpv icon

## Supported sites

- YouTube (`*://www.youtube.com/*`)
- bilibili (`*://www.bilibili.com/video/*`)
- And more sites waiting to add ~

Because `youtube-dl` is supported too many sites , I cannot add all of these sites to `@match` at onetime.

If you need support for some sites, welcome to PR or create a [Issues](https://github.com/akiirui/play-with-mpv-handler/issues/new).

Of course, `youtube-dl` [supported sites](https://ytdl-org.github.io/youtube-dl/supportedsites.html) only.

## GitHub

- [play-with-mpv-handler](https://github.com/akiirui/play-with-mpv-handler/)

## Thanks

- [mpv](https://mpv.io/)
- [youtube-dl](https://github.com/ytdl-org/youtube-dl/)
- [papirus-icon-theme](https://github.com/PapirusDevelopmentTeam/papirus-icon-theme)
