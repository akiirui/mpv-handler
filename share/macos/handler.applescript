use framework "Foundation"

using terms from application "/System/Library/ScriptingAdditions/StandardAdditions.osax"
    on open location theURL
        set appBundle to current application's NSBundle's mainBundle()
        set launcher to appBundle's pathForResource:"open-url" ofType:(missing value)
        current application's NSTask's launchedTaskWithLaunchPath:launcher arguments:{theURL}
        return
    end open location
end using terms from
