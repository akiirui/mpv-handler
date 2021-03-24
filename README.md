# mpv handler

A protocol handler for mpv, written by Rust.

Working with userscript [play-with-mpv-handler][mpv-handler-greasyfork].

## Protocol URL

Base URL:

```
mpv://BASE64_ENCODE_URL/
```

Optional parameters:

```
quality = [ best, 4k, 2k, 1080p, 720p ]
cookies = [ yes, no ]
```

Example:

```
mpv://aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj01cWFwNWFPNGk5QQ==/?quality=best&cookies=no
```

[mpv-handler-greasyfork]: https://greasyfork.org/scripts/416271-play-with-mpv
[play-with-mpv-handler]: https://github.com/akiirui/userscript/tree/main/play-with-mpv-handler
