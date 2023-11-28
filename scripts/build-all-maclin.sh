#!/usr/bin/env bash

set -euo pipefail

DIR=$(readlink -f $(dirname "${BASH_SOURCE[0]}"))
REPO_ROOT=$(dirname $DIR)

echo "---------------------------------------------------------------"
echo "Building rsh (rsh) with dataframes and all the plugins"
echo "---------------------------------------------------------------"
echo ""

rsh_pLUGINS=(
    'rsh_plugin_example'
    'rsh_plugin_gstat'
    'rsh_plugin_inc'
    'rsh_plugin_query'
    'rsh_plugin_custom_values'
)

echo "Building rsh"
(
    cd $REPO_ROOT
    cargo build --features=dataframe,extra --locked
)

for plugin in "${rsh_pLUGINS[@]}"
do
    echo "Building $plugin..."
    echo "-----------------------------"
    (
        cd "$REPO_ROOT/crates/$plugin"
        cargo build
    )
done
