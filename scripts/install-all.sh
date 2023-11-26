#!/usr/bin/env bash

set -euo pipefail

DIR=$(readlink -f $(dirname "${BASH_SOURCE[0]}"))
REPO_ROOT=$(dirname $DIR)

echo "-----------------------------------------------------------------"
echo "Installing rsh (rsh) with dataframes and all the plugins"
echo "-----------------------------------------------------------------"
echo ""

echo "Install rsh from local..."
echo "----------------------------------------------"
cargo install --force --path "$REPO_ROOT" --features=dataframe,extra --locked

RSH_PLUGINS=(
    'rsh_plugin_inc'
    'rsh_plugin_gstat'
    'rsh_plugin_query'
    'rsh_plugin_example'
    'rsh_plugin_custom_values'
    'rsh_plugin_formats'
)

for plugin in "${RSH_PLUGINS[@]}"
do
    echo ''
    echo "----------------------------------------------"
    echo "Install plugin $plugin from local..."
    echo "----------------------------------------------"
    cargo install --force --path "$REPO_ROOT/crates/$plugin"
done
