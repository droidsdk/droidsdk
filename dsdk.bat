ECHO off

rem TODO might not work when calling from a different drive. need to look into exit
rem  also might not restore original dir if interrupted.
rem  look. CMD is a bad shell.
pushd %~dp0

REM Gather args (https://serverfault.com/questions/22443/do-windows-batch-files-have-a-construction/22541#22541)
set args=%1
shift
:start
if [%1] == [] goto done
set args=%args% %1
shift
goto start
:done

set setvars_path=%UserProfile%\.droidsdk\setvars.bat

call && (
    if exist %setvars_path% del %setvars_path%

    REM TODO: don't hardcode cli exec path

    %cd%\droidsdk %args% || (
        echo Failed invoking exec
        exit /b 1
    );

    if exist  %setvars_path% (
        echo "Sourcing from setvars.bat"
        call %setvars_path%
    )
) && popd