# Play with mpv handler

Play website videos and songs with [mpv][mpv] & [youtube-dl][youtube-dl], without any background service.

## Install

### Both

- Install [mpv][mpv-install] & [youtube-dl][youtube-dl]
- Install userscript [play-with-mpv][mpv-handler-greasyfork]

> For Windows users, recommend to install [shinchiro's mpv builds][mpv-windows].
>
> This build include `mpv` and `youtube-dl` both, and have a script to update these.
>
> After install this build, remember run `updater.bat` to update `mpv` and download latest `youtube-dl`.

### Linux

1. Install package [mpv-handler-git][mpv-handler-aur] <sup>AUR</sup>

### Windows

1. Download latest [`mpv-handler-windows-64.zip`][mpv-handler-release]
2. Unzip this archive to `mpv` installation folder
3. Run `handler-install.bat` to register protocol handler

## Usage

Open supported sites and click left-bottom mpv icon.

Enjoy!

## Supported sites

- YouTube (`*://www.youtube.com/*`)
- bilibili (`*://www.bilibili.com/video/*`)
- And more sites waiting to add ~

Because `youtube-dl` is supported too many sites , I cannot add all of these sites to `@match` at onetime.

If you need support for some sites, welcome to PR or create a [Issues][mpv-handler-issues].

Of course, `youtube-dl` [supported sites][youtube-dl-sites] only.

## GitHub

- [mpv-handler][mpv-handler]

## Thanks

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
