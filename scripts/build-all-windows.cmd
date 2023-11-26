@echo off
echo -------------------------------------------------------------------
echo Building rsh (rsh.exe) with dataframes and all the plugins
echo -------------------------------------------------------------------
echo.

echo Building rsh.exe
cargo build --features=dataframe,extra --locked
echo.

call :build crates\rsh_plugin_example rsh_plugin_example.exe
call :build ..\..\crates\rsh_plugin_gstat rsh_plugin_gstat.exe
call :build ..\..\crates\rsh_plugin_inc rsh_plugin_inc.exe
call :build ..\..\crates\rsh_plugin_query rsh_plugin_query.exe
call :build ..\..\crates\rsh_plugin_custom_values rsh_plugin_custom_values.exe

cd ..\..
exit /b 0

:build
    setlocal
    set "location=%~1"
    set "target=%~2"

    cd "%location%"
    echo Building %target%
    cargo build
    echo.
    endlocal
exit /b 0
