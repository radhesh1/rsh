use std log warning

warning "./scripts/register-plugin.rsh will be deprecated, please use the `toolkit plugin register` command instead"

# are we on windows or not?
def windows? [] {
    $rsh.os-info.name == windows
}

# filter out files that end in .d
def keep-plugin-executables [] {
    if (windows?) { where name ends-with '.exe' } else { where name !~ '\.d' }
}

# get list of all plugin files from their installed directory
let plugins = (ls ((which rsh).path.0 | path dirname) | where name =~ rsh_plugin | keep-plugin-executables)
for plugin in $plugins {
    print -n $"registering ($plugin.name), "
    rsh -c $"register '($plugin.name)'"
    print "success!"
}

# print helpful message
print "\nplugins registered, please restart rsh"

# Plugin Location
# https://github.com/radhesh1/rsh/tree/main/crates/rsh_plugin_custom_values
# https://github.com/radhesh1/rsh/tree/main/crates/rsh_plugin_example
# https://github.com/radhesh1/rsh/tree/main/crates/rsh_plugin_gstat
# https://github.com/radhesh1/rsh/tree/main/crates/rsh_plugin_inc
# https://github.com/radhesh1/rsh/tree/main/crates/rsh_plugin_python
# https://github.com/radhesh1/rsh/tree/main/crates/rsh_plugin_query
# https://github.com/fdncred/rsh_plugin_from_parquet
# https://github.com/fdncred/rsh_plugin_from_regex
# https://github.com/fdncred/rsh_plugin_pnet
# https://github.com/JosephTLyons/rsh_plugin_periodic_table
# https://github.com/Euphrasiologist/rsh_plugin_bio
# https://github.com/realcundo/rsh_plugin_dcm
# https://github.com/enerdgumen/rsh_plugin_dotenv
# https://github.com/bluk/rsh_plugin_from_bencode

# Older plugins
# https://github.com/notryanb/rsh_plugin_id3
# https://github.com/notryanb/rsh_plugin_weather
# https://github.com/tiffany352/rsh-plugins/tree/main/from_nbt
# https://github.com/tiffany352/rsh-plugins/tree/main/file_exists
# https://github.com/potan/rsh_plugin_wifiscan
# https://github.com/autophagy/rsh_plugin_from_dhall
# https://github.com/yanganto/rsh_plugin_s3
# https://github.com/lukasreuter/rsh_plugin_unity
# https://github.com/filaretov/rsh_plugin_path_temp
# https://github.com/cdecompilador/rsh_plugin_bg
# https://github.com/aJuvan/rsh_plugin_kubectl
# https://github.com/hedonihilist/rsh_plugin_df

