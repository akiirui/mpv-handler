// ==UserScript==
// @name                Play with mpv
// @name:en-US          Play with mpv
// @name:zh-CN          使用 mpv 播放
// @name:zh-TW          使用 mpv 播放
// @description         Play website videos and songs with mpv & youtube-dl
// @description:en-US   Play website videos and songs with mpv & youtube-dl
// @description:zh-CN   通过 mpv 和 youtube-dl 播放网页上的视频和歌曲
// @description:zh-TW   通過 mpv 和 youtube-dl 播放網頁上的視頻和歌曲
// @namespace           play-with-mpv-handler
// @version             2020.12.27
// @author              Akatsuki Rui
// @license             MIT License
// @grant               GM_getValue
// @grant               GM_setValue
// @grant               GM_notification
// @run-at              document-idle
// @noframes
// @match               *://www.youtube.com/*
// @match               *://www.bilibili.com/video/*
// ==/UserScript==

"use strict";

const MPV_HANDLER_VERSION = "v0.1.0";
const MPV_ICON =
  "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI2NCIgaGVpZ2h0PSI2NCIgdmVyc2lvbj0iMSI+CiA8Y2lyY2xlIHN0eWxlPSJvcGFjaXR5Oi4yIiBjeD0iMzIiIGN5PSIzMyIgcj0iMjgiLz4KIDxjaXJjbGUgc3R5bGU9ImZpbGw6IzhkMzQ4ZSIgY3g9IjMyIiBjeT0iMzIiIHI9IjI4Ii8+CiA8Y2lyY2xlIHN0eWxlPSJvcGFjaXR5Oi4zIiBjeD0iMzQuNSIgY3k9IjI5LjUiIHI9IjIwLjUiLz4KIDxjaXJjbGUgc3R5bGU9Im9wYWNpdHk6LjIiIGN4PSIzMiIgY3k9IjMzIiByPSIxNCIvPgogPGNpcmNsZSBzdHlsZT0iZmlsbDojZmZmZmZmIiBjeD0iMzIiIGN5PSIzMiIgcj0iMTQiLz4KIDxwYXRoIHN0eWxlPSJmaWxsOiM2OTFmNjkiIHRyYW5zZm9ybT0ibWF0cml4KDEuNTE1NTQ0NSwwLDAsMS41LC0zLjY1Mzg3OSwtNC45ODczODQ4KSIgZD0ibTI3LjE1NDUxNyAyNC42NTgyNTctMy40NjQxMDEgMi0zLjQ2NDEwMiAxLjk5OTk5OXYtNC0zLjk5OTk5OWwzLjQ2NDEwMiAyeiIvPgogPHBhdGggc3R5bGU9ImZpbGw6I2ZmZmZmZjtvcGFjaXR5Oi4xIiBkPSJNIDMyIDQgQSAyOCAyOCAwIDAgMCA0IDMyIEEgMjggMjggMCAwIDAgNC4wMjE0ODQ0IDMyLjU4NTkzOCBBIDI4IDI4IDAgMCAxIDMyIDUgQSAyOCAyOCAwIDAgMSA1OS45Nzg1MTYgMzIuNDE0MDYyIEEgMjggMjggMCAwIDAgNjAgMzIgQSAyOCAyOCAwIDAgMCAzMiA0IHoiLz4KPC9zdmc+Cgo=";
const MATCH_URLS = ["www.youtube.com/watch", "www.bilibili.com/video/"];

function notifyHandlerUpdate() {
  const NOTIFICATION = {
    title: "Play with mpv",
    text: `mpv-handler is upgraded to ${MPV_HANDLER_VERSION}

Check updates at:
https://github.com/akiirui/mpv-handler/releases`,
  };

  let version = GM_getValue("mpvHandlerVersion", null);
  if (version !== MPV_HANDLER_VERSION) {
    GM_notification(NOTIFICATION);
    GM_setValue("mpvHandlerVersion", MPV_HANDLER_VERSION);
  }
}

function appendButton() {
  let head = document.getElementsByTagName("head")[0];
  let body = document.getElementsByTagName("body")[0];
  let style = document.createElement("style");
  let buttonIframe = document.createElement("iframe");
  let button = document.createElement("a");

  if (head) {
    style.innerHTML = `.play-with-mpv {
  position: fixed;
  left: 12px;
  bottom: 12px;
  width: 48px;
  height: 48px;
  border: 0;
  border-radius: 50%;
  background-size: 100%;
  background-image: url(${MPV_ICON});
  background-repeat: no-repeat;
}`;
    head.appendChild(style);
  }

  if (body) {
    buttonIframe.name = "play-with-mpv";
    buttonIframe.style = "display: none";
    body.appendChild(buttonIframe);

    button.className = "play-with-mpv";
    button.style = "display: none";
    button.target = "play-with-mpv";
    button.addEventListener("click", () => {
      let videoElement = document.getElementsByTagName("video")[0];
      if (videoElement) videoElement.pause();
    });
    body.appendChild(button);

    updateButton(location.href);
  }
}

function updateButton(currentUrl) {
  let isMatch = false;
  let button = document.getElementsByClassName("play-with-mpv")[0];

  for (const element of MATCH_URLS) {
    if ((isMatch = currentUrl.includes(element))) break;
  }

  if (button) {
    button.style = isMatch ? "display: inline-block" : "display: none";
    button.href = isMatch ? "mpv://" + btoa(currentUrl) : "";
  }
}

function detectPJAX() {
  let previousUrl = null;

  setInterval(() => {
    let currentUrl = location.href;

    if (currentUrl && previousUrl !== currentUrl) {
      updateButton(currentUrl);
      previousUrl = currentUrl;
    }
  }, 500);
}

notifyHandlerUpdate();
appendButton();
detectPJAX();
