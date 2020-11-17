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
// @version             2020.11.17.2
// @author              Akatsuki Rui
// @license             MIT License
// @grant               GM_info
// @run-at              document-idle
// @match               *://www.youtube.com/watch*
// @match               *://www.bilibili.com/video/*
// ==/UserScript==

"use strict";

function buttonAppend() {
  let head;
  let styl;
  let body;
  let button;

  head = document.getElementsByTagName("head")[0];
  if (!head) {
    console.log("Not found elements <head>");
    return;
  }
  body = document.getElementsByTagName("body")[0];
  if (!body) {
    console.log("Not found elements <body>");
    return;
  }

  styl = document.createElement("style");
  styl.innerHTML =
    ".play-with-mpv{display:inline-block;position:fixed;left:12px;bottom:12px;width:48px;height:48px;border:0;border-radius:50%;background-size:100%;background-image:url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI2NCIgaGVpZ2h0PSI2NCIgdmVyc2lvbj0iMSI+CiA8Y2lyY2xlIHN0eWxlPSJvcGFjaXR5Oi4yIiBjeD0iMzIiIGN5PSIzMyIgcj0iMjgiLz4KIDxjaXJjbGUgc3R5bGU9ImZpbGw6IzhkMzQ4ZSIgY3g9IjMyIiBjeT0iMzIiIHI9IjI4Ii8+CiA8Y2lyY2xlIHN0eWxlPSJvcGFjaXR5Oi4zIiBjeD0iMzQuNSIgY3k9IjI5LjUiIHI9IjIwLjUiLz4KIDxjaXJjbGUgc3R5bGU9Im9wYWNpdHk6LjIiIGN4PSIzMiIgY3k9IjMzIiByPSIxNCIvPgogPGNpcmNsZSBzdHlsZT0iZmlsbDojZmZmZmZmIiBjeD0iMzIiIGN5PSIzMiIgcj0iMTQiLz4KIDxwYXRoIHN0eWxlPSJmaWxsOiM2OTFmNjkiIHRyYW5zZm9ybT0ibWF0cml4KDEuNTE1NTQ0NSwwLDAsMS41LC0zLjY1Mzg3OSwtNC45ODczODQ4KSIgZD0ibTI3LjE1NDUxNyAyNC42NTgyNTctMy40NjQxMDEgMi0zLjQ2NDEwMiAxLjk5OTk5OXYtNC0zLjk5OTk5OWwzLjQ2NDEwMiAyeiIvPgogPHBhdGggc3R5bGU9ImZpbGw6I2ZmZmZmZjtvcGFjaXR5Oi4xIiBkPSJNIDMyIDQgQSAyOCAyOCAwIDAgMCA0IDMyIEEgMjggMjggMCAwIDAgNC4wMjE0ODQ0IDMyLjU4NTkzOCBBIDI4IDI4IDAgMCAxIDMyIDUgQSAyOCAyOCAwIDAgMSA1OS45Nzg1MTYgMzIuNDE0MDYyIEEgMjggMjggMCAwIDAgNjAgMzIgQSAyOCAyOCAwIDAgMCAzMiA0IHoiLz4KPC9zdmc+Cgo=);background-repeat:no-repeat}";

  button = document.createElement("a");
  button.className = "play-with-mpv";

  head.appendChild(styl);
  body.appendChild(button);
}

function detectPJAX() {
  let previousUrl = null;
  setInterval(() => {
    let currentUrl = location.href;
    if (currentUrl !== null && previousUrl !== currentUrl) {
      document.querySelector("a[class='play-with-mpv']").href =
        "mpv://" + btoa(currentUrl);
      previousUrl = currentUrl;
    }
  }, 500);
}

buttonAppend();
detectPJAX();
