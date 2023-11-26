# this module regroups a bunch of development tools to make the development
# process easier for anyone.
#
# the main purpose of `toolkit` is to offer an easy to use interface for the
# developer during a PR cycle, namely to (**1**) format the source base,
# (**2**) catch classical flaws in the new changes with *clippy* and (**3**)
# make sure all the tests pass.

# check standard code formatting and apply the changes
export def fmt [
    --check # do not apply the format changes, only check the syntax
    --verbose  # print extra information about the command's progress
] {
    if $verbose {
        print $"running ('toolkit fmt' | pretty-format-command)"
    }

    if $check {
        try {
            cargo fmt --all -- --check
        } catch {
            error make --unspanned {
                msg: $"\nplease run ('toolkit fmt' | pretty-format-command) to fix formatting!"
            }
        }
    } else {
        cargo fmt --all
    }
}

# check that you're using the standard code style
#
# > it is important to make `clippy` happy :relieved:
export def clippy [
    --verbose # print extra information about the command's progress
    --features: list<string> # the list of features to run *Clippy* on
] {
    if $verbose {
        print $"running ('toolkit clippy' | pretty-format-command)"
    }

    # If changing these settings also change CI settings in .github/workflows/ci.yml
    try {(
        cargo clippy
            --workspace
            --exclude rsh_plugin_*
            --features ($features | str join ",")
        --
            -D warnings
            -D clippy::unwrap_used
    )

    if $verbose {
        print $"running ('toolkit clippy' | pretty-format-command) on tests"
    }
    # In tests we don't have to deny unwrap
    (
        cargo clippy
            --tests
            --workspace
            --exclude rsh_plugin_*
            --features ($features | str join ",")
        --
            -D warnings
    )

    if $verbose {
        print $"running ('toolkit clippy' | pretty-format-command) on plugins"
    }
    (
        cargo clippy
            --package rsh_plugin_*
        --
            -D warnings
            -D clippy::unwrap_used
    )

    } catch {
        error make --unspanned {
            msg: $"\nplease fix the above ('clippy' | pretty-format-command) errors before contirshing!"
        }
    }
}

# check that all the tests pass
export def test [
    --fast # use the "nextext" `cargo` subcommand to speed up the tests (see [`cargo-nextest`](https://nexte.st/) and [`nextest-rs/nextest`](https://github.com/nextest-rs/nextest))
    --features: list<string> # the list of features to run the tests on
    --workspace # run the *Clippy* command on the whole workspace (overrides `--features`)
] {
    if $fast {
        if $workspace {
            cargo nextest run --all
        } else {
            cargo nextest run --features ($features | str join ",")
        }
    } else {
        if $workspace {
            cargo test --workspace
        } else {
            cargo test --features ($features | str join ",")
        }
    }
}

# run the tests for the standard library
export def "test stdlib" [
    --extra-args: string = ''
] {
    cargo run -- -c $"use std testing; testing run-tests --path crates/rsh-std ($extra_args)"
}

# formats the pipe input inside backticks, dimmed and italic, as a pretty command
def pretty-format-command [] {
    $"`(ansi default_dimmed)(ansi default_italic)($in)(ansi reset)`"
}

# return a report about the check stage
#
# - fmt comes first
# - then clippy
# - and finally the tests
#
# without any option, `report` will return an empty report.
# otherwise, the truth values will be incremental, following
# the order above.
def report [
    --fail-fmt
    --fail-clippy
    --fail-test
    --fail-test-stdlib
    --no-fail
] {
    [fmt clippy test "test stdlib"]
    | wrap stage
    | merge (
        if $no_fail               { [true     true     true     true] }
        else if $fail_fmt         { [false    null null null] }
        else if $fail_clippy      { [true     false    null null] }
        else if $fail_test        { [true     true     false    null] }
        else if $fail_test_stdlib { [true     true     true     false] }
        else                      { [null null null null] }
        | wrap success
    )
    | upsert emoji {|it|
        if ($it.success == null) {
            ":black_circle:"
        } else if $it.success {
            ":green_circle:"
        } else {
            ":red_circle:"
        }
    }
    | each {|it|
        $"- ($it.emoji) `toolkit ($it.stage)`"
    }
    | to text
}

# run all the necessary checks and tests to submit a perfect PR
#
# # Example
# let us say we apply a change that
# - breaks the formatting, e.g. with extra newlines everywhere
# - makes clippy sad, e.g. by adding unnecessary string conversions with `.to_string()`
# - breaks the tests by output bad string data from a data structure conversion
#
# > the following diff breaks all of the three checks!
# > ```diff
# > diff --git a/crates/rsh-command/src/formats/to/nuon.rs b/crates/rsh-command/src/formats/to/nuon.rs
# > index abe34c054..927d6a3de 100644
# > --- a/crates/rsh-command/src/formats/to/rshon.rs
# > +++ b/crates/rsh-command/src/formats/to/rshon.rs
# > @@ -131,7 +131,8 @@ pub fn value_to_string(v: &Value, span: Span) -> Result<String, ShellError> {
# >                          }
# >                      })
# >                      .collect();
# > -                let headers_output = headers.join(", ");
# > +                let headers_output = headers.join(&format!("x {}", "")
# > +                    .to_string());
# >
# >                  let mut table_output = vec![];
# >                  for val in vals {
# > ```
#
# > **Note**
# > at every stage, the `toolkit check pr` will return a report of the few stages being run.
#
# - we run the toolkit once and it fails...
# ```rsh
# >_ toolkit check pr
# running `toolkit fmt`
# Diff in /home/amtoine/.local/share/git/store/github.com/amtoine/rsh/crates/rsh-command/src/formats/to/nuon.rs at line 131:
#                          }
#                      })
#                      .collect();
# -                let headers_output = headers.join(&format!("x {}", "")
# -                    .to_string());
# +                let headers_output = headers.join(&format!("x {}", "").to_string());
#
#                  let mut table_output = vec![];
#                  for val in vals {
#
# please run toolkit fmt to fix the formatting
# ```
# - we run `toolkit fmt` as proposed and rerun the toolkit... to see clippy is sad...
# ```rsh
# running `toolkit fmt`
# running `toolkit clippy`
# ...
# error: redundant clone
#    --> crates/rsh-command/src/formats/to/nuon.rs:134:71
#     |
# 134 |                 let headers_output = headers.join(&format!("x {}", "").to_string());
#     |                                                                       ^^^^^^^^^^^^ help: remove this
#     |
# note: this value is dropped without further use
#    --> crates/rsh-command/src/formats/to/nuon.rs:134:52
#     |
# 134 |                 let headers_output = headers.join(&format!("x {}", "").to_string());
#     |                                                    ^^^^^^^^^^^^^^^^^^^
#     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone
#     = note: `-D clippy::redundant-clone` implied by `-D warnings`
#
# error: could not compile `rsh-command` due to previous error
# ```
# - we remove the useless `.to_string()`, and in that cases, the whole format is useless, only `"x "` is useful!
# but now the tests do not pass :sob:
# ```rsh
# running `toolkit fmt`
# running `toolkit clippy`
# ...
# running `toolkit test`
# ...
# failures:
#     commands::insert::insert_uses_enumerate_index
#     commands::merge::multi_row_table_overwrite
#     commands::merge::single_row_table_no_overwrite
#     commands::merge::single_row_table_overwrite
#     commands::update::update_uses_enumerate_index
#     commands::upsert::upsert_uses_enumerate_index_inserting
#     commands::upsert::upsert_uses_enumerate_index_updating
#     commands::where_::where_uses_enumerate_index
#     format_conversions::nuon::does_not_quote_strings_unnecessarily
#     format_conversions::nuon::to_nuon_table
# ```
# - finally let's fix the tests by removing the `x`, essentially removing the whole diff we applied at the top!
#
# now the whole `toolkit check pr` passes! :tada:
export def "check pr" [
    --fast # use the "nextext" `cargo` subcommand to speed up the tests (see [`cargo-nextest`](https://nexte.st/) and [`nextest-rs/nextest`](https://github.com/nextest-rs/nextest))
    --features: list<string> # the list of features to check the current PR on
] {
    $env.RSH_TEST_LOCALE_OVERRIDE = 'en_US.utf8';
    try {
        fmt --check --verbose
    } catch {
        return (report --fail-fmt)
    }

    try {
        clippy --features $features --verbose
    } catch {
        return (report --fail-clippy)
    }

    print $"running ('toolkit test' | pretty-format-command)"
    try {
        if $fast {
            if ($features | is-empty) {
                test --workspace --fast
            } else {
                test --features $features --fast
            }
        } else {
            if ($features | is-empty) {
                test --workspace
            } else {
                test --features $features
            }
        }
    } catch {
        return (report --fail-test)
    }

    print $"running ('toolkit test stdlib' | pretty-format-command)"
    try {
        test stdlib
    } catch {
        return (report --fail-test-stdlib)
    }

    report --no-fail
}

# run rsh from source with a right indicator
export def run [] {
    cargo run -- [
        -e "$env.PROMPT_COMMAND_RIGHT = $'(ansi magenta_reverse)trying rsh inside Cargo(ansi reset)'"
    ]
}

# set up git hooks to run:
# - `toolkit fmt --check --verbose` on `git commit`
# - `toolkit fmt --check --verbose` and `toolkit clippy --verbose` on `git push`
export def setup-git-hooks [] {
    print "This command will change your local git configuration and hence modify your development workflow. Are you sure you want to continue? [y]"
    if (input) == "y" {
        print $"running ('toolkit setup-git-hooks' | pretty-format-command)"
        git config --local core.hooksPath .githooks
    } else {
        print $"aborting ('toolkit setup-git-hooks' | pretty-format-command)"
    }
}

def build-rsh [features: string] {
    print $'(char nl)Building rsh'
    print '----------------------------'

    cargo build --features $features --locked
}

def build-plugin [] {
    let plugin = $in

    print $'(char nl)Building ($plugin)'
    print '----------------------------'

    cd $"crates/($plugin)"
    cargo build
}

# build rsh and plugins with some features
export def build [
    ...features: string@"rsh-complete list features"  # a space-separated list of feature to install with rsh
    --all # build all plugins with rsh
] {
    build-rsh ($features | str join ",")

    if not $all {
        return
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
}

def "rsh-complete list features" [] {
    open Cargo.toml | get features | transpose feature dependencies | get feature
}

def install-plugin [] {
    let plugin = $in

    print $'(char nl)Installing ($plugin)'
    print '----------------------------'

    cargo install --path $"crates/($plugin)"
}

# install rsh and features you want
export def install [
    ...features: string@"rsh-complete list features"  # a space-separated list of feature to install with rsh
    --all # install all plugins with rsh
] {
    touch crates/rsh-cmd-lang/build.rs # needed to make sure `version` has the correct `commit_hash`
    cargo install --path . --features ($features | str join ",") --locked --force
    if not $all {
        return
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
        $plugin | install-plugin
    }
}

def windows? [] {
    $rsh.os-info.name == windows
}

# filter out files that end in .d
def keep-plugin-executables [] {
    if (windows?) { where name ends-with '.exe' } else { where name !~ '\.d' }
}

# register all installed plugins
export def "register plugins" [] {
    let plugin_path = (which rsh | get path.0 | path dirname)
    let plugins = (ls $plugin_path | where name =~ rsh_plugin | keep-plugin-executables)

    if ($plugins | is-empty) {
        print $"no plugins found in ($plugin_path)..."
        return
    }

    for plugin in $plugins {
        print -n $"registering ($plugin.name), "
        rsh -c $"register '($plugin.name)'"
        print "success!"
    }

    print "\nplugins registered, please restart rsh"
}

def compute-coverage [] {
    print "Setting up environment variables for coverage"
    # Enable LLVM coverage tracking through environment variables
    # show env outputs .ini/.toml style description of the variables
    # In order to use from toml, we need to make sure our string literals are single quoted
    # This is especially important when running on Windows since "C:\blah" is treated as an escape
    cargo llvm-cov show-env | str replace (char dq) (char sq) -a | from toml | load-env

    print "Cleaning up coverage data"
    cargo llvm-cov clean --workspace

    print "Building with workspace and profile=ci"
    # Apparently we need to explicitly build the necessary parts
    # using the `--profile=ci` is basically `debug` build with unnecessary symbols stripped
    # leads to smaller binaries and potential savings when compiling and running
    cargo build --workspace --profile=ci

    print "Running tests with --workspace and profile=ci"
    cargo test --workspace --profile=ci

    # You need to provide the used profile to find the raw data
    print "Generating coverage report as lcov.info"
    cargo llvm-cov report --lcov --output-path lcov.info --profile=ci
}

# Script to generate coverage locally
#
# Output: `lcov.info` file
#
# Relies on `cargo-llvm-cov`. Install via `cargo install cargo-llvm-cov`
# https://github.com/taiki-e/cargo-llvm-cov
#
# You probably have to run `cargo llvm-cov clean` once manually,
# as you have to confirm to install additional tooling for your rustup toolchain.
# Else the script might stall waiting for your `y<ENTER>`
#
# Some of the internal tests rely on the exact cargo profile
# (This is somewhat criminal itself)
# but we have to signal to the tests that we use the `ci` `--profile`
#
# Manual gathering of coverage to catch invocation of the `rsh` binary.
# This is relevant for tests using the `rsh!` macro from `rsh-test-support`
# see: https://github.com/taiki-e/cargo-llvm-cov#get-coverage-of-external-tests
#
# To display the coverage in your editor see:
#
# - https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters
# - https://github.com/umaumax/vim-lcov
# - https://github.com/andythigpen/nvim-coverage (probably needs some additional config)
export def cov [] {
    let start = (date now)
    $env.rsh_CARGO_PROFILE = "ci"

    compute-coverage

    let end = (date now)
    print $"Coverage generation took ($end - $start)."
}

export def main [] { help toolkit }
