use std log warning

print '-------------------------------------------------------------------'
print 'Building rsh (nu) with dataframes and all the plugins'
print '-------------------------------------------------------------------'

warning "./scripts/build-all.nu will be deprecated, please use the `toolkit build` command instead"

let repo_root = ($env.CURRENT_FILE | path dirname --num-levels 2)

def build-rsh [] {
    print $'(char nl)Building rsh'
    print '----------------------------'

    cd $repo_root
    cargo build --features=dataframe,extra --locked
}

def build-plugin [] {
    let plugin = $in

    print $'(char nl)Building ($plugin)'
    print '----------------------------'

    cd $'($repo_root)/crates/($plugin)'
    cargo build
}

let plugins = [
    rsh_plugin_inc,
    rsh_plugin_gstat,
    rsh_plugin_query,
    rsh_plugin_example,
    rsh_plugin_custom_values,
    rsh_plugin_formats,
]

for plugin in $plugins {
    $plugin | build-plugin
}
