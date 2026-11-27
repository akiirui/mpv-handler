use framework "Foundation"

using terms from application "/System/Library/ScriptingAdditions/StandardAdditions.osax"
    on open location theURL
        set appBundle to current application's NSBundle's mainBundle()
        set launcher to appBundle's pathForResource:"open-url" ofType:(missing value)
        if theURL starts with "mpv-handler-debug://" then
            set commandText to quoted form of (launcher as text) & " " & quoted form of (theURL as text)
            set configHome to system attribute "XDG_CONFIG_HOME"
            if configHome is not "" then set commandText to "/usr/bin/env " & quoted form of ("XDG_CONFIG_HOME=" & configHome) & " " & commandText
            tell application "/System/Applications/Utilities/Terminal.app"
                launch
                do script ("exec " & commandText)
                activate
            end tell
        else
            current application's NSTask's launchedTaskWithLaunchPath:launcher arguments:{theURL}
        end if
        return
    end open location
end using terms from
