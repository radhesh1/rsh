
# Usage: Just run `powershell install-all.ps1` in rsh root directory

Write-Output "-----------------------------------------------------------------"
Write-Output "Installing rsh (rsh) with dataframes and all the plugins"
Write-Output "-----------------------------------------------------------------"
Write-Output ""

Write-Output "Install rsh from local..."
Write-Output "----------------------------------------------"
cargo install --force --path . --features=dataframe,extra --locked

$RSH_PLUGINS = @(
    'rsh_plugin_example',
    'rsh_plugin_gstat',
    'rsh_plugin_inc',
    'rsh_plugin_query',
    'rsh_plugin_custom_values',
    'rsh_plugin_formats'
)

foreach ( $plugin in $RSH_PLUGINS) {
    Write-Output ''
    Write-Output "----------------------------------------------"
    Write-Output "Install plugin $plugin from local..."
    Write-Output "----------------------------------------------"
    Set-Location crates/$plugin
    cargo install --force --path .
    Set-Location ../../
}

