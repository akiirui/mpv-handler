# mpv handler

A protocol handler for mpv, written by Rust.

Working with userscript [play-with-mpv-handler][mpv-handler-greasyfork].

## Protocol URL

Base URL:

```
mpv://BASE64_ENCODE_URL/
```

Required paramentrs:

```
downloader  = [ ytdl, you-get, streamlink, and more...]
```

Optional parameters:

```
cookies = [ video_domain.txt ]
quality = [ best, 4k, 2k, 1080p, 720p, 480p, 360p, and more... ]
```

Example:

```
mpv://aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj01cWFwNWFPNGk5QQ==/?cookies=www.youtube.com.txt&downloader=ytdl&quality=best
```

## Customize Configure File

```toml
### Player ###
[player]
mpv = "/usr/bin/mpv"  # Change it to your mpv executable file path

### Video Downloader ###
# You should be change the value of "bin" to your downloader executable file path.

# Attention!! [ytdl] bin path should be mpv executable file.
# Because youtube-dl not have `--player` option to set player.
# The pipeline method `youtube-dl URL -o - | player -` Will destroy quality choices.
[ytdl]
bin = "/usr/bin/mpv" # Change it to your mpv executable file path
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

[you-get]
bin = "/usr/bin/you-get" # Change it to your you-get executable file path
cookies = "--cookies"
options = ["--player"]

[streamlink]
bin = "/usr/bin/streamlink" # Change it to your streamlink executable file path
options = ["--player"]

# For advanced user, you can add downloader manually.
# Example:
[example-downloader]
bin = "/usr/bin/example"        # Required, String. The downloader executable file path.
cookies = "--cookies"           # Optional, String. The option to set cookies, if it support pass cookies.
cookies_prefix = false          # Optional, Boolen. The option to mark cookies option is prefix.
direct = false                  # Optional, Boolen. The option to mark downlader is player.
options = ["--player"]          # Optional, String Array. The options to set player or output.
quality.best = "--quality=best" # Optional, String. The option to set quality.
```

### How customize configure works?

Example protocol URL:

```
mpv://aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj01cWFwNWFPNGk5QQ==/?cookies=www.youtube.com.txt&downloader=ytdl&quality=best
```

1. Read customize configure file and parse protocol URL.
2. If protocol URL given option `downloader=example-downloader`
3. Create a command follow `[example-downloader] -> bin`:

```
/usr/bin/example
```

4. Append video URL:

```
/usr/bin/example "https://www.youtube.com/watch?v=5qap5aO4i9A"
```

5. Append cookies option `[example-downloader] -> cookies` and cookies file path

```
/usr/bin/example "https://www.youtube.com/watch?v=5qap5aO4i9A" --cookies "~/.config/mpv/cookies/www.youtube.com.txt"
```

6. Append quailty option `[example-downloader] -> quality.NAME`, here is `quality.best`

```
/usr/bin/example "https://www.youtube.com/watch?v=5qap5aO4i9A" --cookies "~/.config/mpv/cookies/www.youtube.com.txt" --quality=best
```

7. Append player option `[example-downloader] -> options` and `[player] -> mpv`

```
/usr/bin/example "https://www.youtube.com/watch?v=5qap5aO4i9A" --cookies "~/.config/mpv/cookies/www.youtube.com.txt" --quality=best --player /usr/bin/mpv
```

8. Run this command

[mpv-handler-greasyfork]: https://greasyfork.org/scripts/416271-play-with-mpv
[play-with-mpv-handler]: https://github.com/akiirui/userscript/tree/main/play-with-mpv-handler
