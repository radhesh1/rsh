#!/usr/bin/env bash

set -euo pipefail

echo ''
echo "----------------------------------------------"
echo "Uninstall rsh and all plugins from cargo/bin..."
echo "----------------------------------------------"

RSH_PLUGINS=(
    'rsh_plugin_inc'
    'rsh_plugin_gstat'
    'rsh_plugin_query'
    'rsh_plugin_example'
    'rsh_plugin_formats'
    'rsh_plugin_custom_values'
)

cargo uninstall rsh
for plugin in "${RSH_PLUGINS[@]}"
do
    cargo uninstall "$plugin"
done
