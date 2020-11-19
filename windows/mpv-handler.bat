@echo OFF

if not "%1" == "" goto :argsexists
    goto:EOF

:argsexists
    set video_url_full=%1
    set video_url_base64=%video_url_full:~6%

    del /q /f "%temp%\mpv_input.tmp" >nul 2>nul
    del /q /f "%temp%\mpv_output.tmp" >nul 2>nul

    echo %video_url_base64%>"%temp%\mpv_input.tmp"
    certutil -decode "%temp%\mpv_input.tmp" "%temp%\mpv_output.tmp" >nul 2>nul
    set /p video_url=<"%temp%\mpv_output.tmp"

    del /q /f "%temp%\mpv_input.tmp" >nul 2>nul
    del /q /f "%temp%\mpv_output.tmp" >nul 2>nul

    start /b "Playing %video_url%" mpv.com %video_url%
