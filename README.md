# Rsh <!-- omit in toc -->
[![Build Status](https://img.shields.io/github/actions/workflow/status/radhesh1/rsh/ci.yml?branch=main)](https://github.com/radhesh1/rsh/actions)
[![Nightly Build](https://github.com/radhesh1/rsh/actions/workflows/nightly-build.yml/badge.svg)](https://github.com/radhesh1/rsh/actions/workflows/nightly-build.yml)

[//]: # ([![Discord]&#40;https://img.shields.io/discord/601130461678272522.svg?logo=discord&#41;]&#40;`https://discord.gg/KTDnbZMv&#41;)
[![The Changelog #363](https://img.shields.io/badge/The%20Changelog-%23363-61c192.svg)](https://changelog.com/podcast/363)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/m/radhesh1/rsh)](https://github.com/radhesh1/rsh/graphs/commit-activity)
[![GitHub contributors](https://img.shields.io/github/contributors/radhesh1/rsh)](https://github.com/radhesh1/rsh/graphs/contributors)

A new type of shell.

![Example of rsh](assets/rsh-autocomplete6.gif "Example of rsh")

## Table of Contents <!-- omit in toc -->

- [Status](#status)
- [Learning About Rsh](#learning-about-rsh)
- [Installation](#installation)
- [Configuration](#configuration)
- [Philosophy](#philosophy)
  - [Pipelines](#pipelines)
  - [Opening files](#opening-files)
  - [Plugins](#plugins)
- [Goals](#goals)
- [Officially Supported By](#officially-supported-by)
- [Contributing](#contributing)
- [License](#license)

## Status

This project has reached a minimum-viable-product level of quality. Many people use it as their daily driver, but it may be unstable for some commands. Rsh's design is subject to change as it matures.

## Learning About Rsh

The [rsh book](https://irsh.vercel.app/book/) is the primary source of rsh documentation. You can find [a full list of Rsh commands in the book](https://irsh.vercel.app/commands/), and we have many examples of using Rsh in our [cookbook](https://irsh.vercel.app/cookbook/).

We're also active on [Discord](`https://discord.gg/KTDnbZMv) and [Twitter](https://twitter.com/Radhesh_Goel); come and chat with us!

## Installation

To quickly install Rsh:

```bash
# Linux and macOS
brew install rsh
# Windows
winget install rsh
```

To use `Rsh` in GitHub Action, check [setup-nu](https://github.com/marketplace/actions/setup-nu) for more detail.

Detailed installation instructions can be found in the [installation chapter of the book](https://irsh.vercel.app/book/installation.html). Rsh is available via many package managers:

[![Packaging status](https://repology.org/badge/vertical-allrepos/rsh.svg)](https://repology.org/project/rsh/versions)

For details about which platforms the rsh team actively supports, see [our platform support policy](devdocs/PLATFORM_SUPPORT.md).

## Configuration

The default configurations can be found at [sample_config](crates/rsh-utils/src/sample_config)
which are the configuration files one gets when they startup rsh for the first time.

It sets all of the default configuration to run rsh.  From here one can
then customize this file for their specific needs.

To see where *config.nu* is located on your system simply type this command.

```rust
$rsh.config-path
```

Please see our [book](https://irsh.vercel.app) for all of the rsh documentation.


## Philosophy

Rsh draws inspiration from projects like PowerShell, functional programming languages, and modern CLI tools.
Rather than thinking of files and data as raw streams of text, Rsh looks at each input as something with structure.
For example, when you list the contents of a directory what you get back is a table of rows, where each row represents an item in that directory.
These values can be piped through a series of steps, in a series of commands called a 'pipeline'.

### Pipelines

In Unix, it's common to pipe between commands to split up a sophisticated command over multiple steps.
Rsh takes this a step further and builds heavily on the idea of _pipelines_.
As in the Unix philosophy, Rsh allows commands to output to stdout and read from stdin.
Additionally, commands can output structured data (you can think of this as a third kind of stream).
Commands that work in the pipeline fit into one of three categories:

-   Commands that produce a stream (e.g., `ls`)
-   Commands that filter a stream (e.g., `where type == "dir"`)
-   Commands that consume the output of the pipeline (e.g., `table`)

Commands are separated by the pipe symbol (`|`) to denote a pipeline flowing left to right.

```shell
> ls | where type == "dir" | table
╭────┬──────────┬──────┬─────────┬───────────────╮
│ #  │   name   │ type │  size   │   modified    │
├────┼──────────┼──────┼─────────┼───────────────┤
│  0 │ .cargo   │ dir  │     0 B │ 9 minutes ago │
│  1 │ assets   │ dir  │     0 B │ 2 weeks ago   │
│  2 │ crates   │ dir  │ 4.0 KiB │ 2 weeks ago   │
│  3 │ docker   │ dir  │     0 B │ 2 weeks ago   │
│  4 │ docs     │ dir  │     0 B │ 2 weeks ago   │
│  5 │ images   │ dir  │     0 B │ 2 weeks ago   │
│  6 │ pkg_mgrs │ dir  │     0 B │ 2 weeks ago   │
│  7 │ samples  │ dir  │     0 B │ 2 weeks ago   │
│  8 │ src      │ dir  │ 4.0 KiB │ 2 weeks ago   │
│  9 │ target   │ dir  │     0 B │ a day ago     │
│ 10 │ tests    │ dir  │ 4.0 KiB │ 2 weeks ago   │
│ 11 │ wix      │ dir  │     0 B │ 2 weeks ago   │
╰────┴──────────┴──────┴─────────┴───────────────╯
```

Because most of the time you'll want to see the output of a pipeline, `table` is assumed.
We could have also written the above:

```shell
> ls | where type == "dir"
```

Being able to use the same commands and compose them differently is an important philosophy in Rsh.
For example, we could use the built-in `ps` command to get a list of the running processes, using the same `where` as above.

```shell
> ps | where cpu > 0
╭───┬───────┬───────────┬───────┬───────────┬───────────╮
│ # │  pid  │   name    │  cpu  │    mem    │  virtual  │
├───┼───────┼───────────┼───────┼───────────┼───────────┤
│ 0 │  2240 │ Slack.exe │ 16.40 │ 178.3 MiB │ 232.6 MiB │
│ 1 │ 16948 │ Slack.exe │ 16.32 │ 205.0 MiB │ 197.9 MiB │
│ 2 │ 17700 │ nu.exe    │  3.77 │  26.1 MiB │   8.8 MiB │
╰───┴───────┴───────────┴───────┴───────────┴───────────╯
```

### Opening files

Rsh can load file and URL contents as raw text or structured data (if it recognizes the format).
For example, you can load a .toml file as structured data and explore it:

```shell
> open Cargo.toml
╭──────────────────┬────────────────────╮
│ bin              │ [table 1 row]      │
│ dependencies     │ {record 25 fields} │
│ dev-dependencies │ {record 8 fields}  │
│ features         │ {record 10 fields} │
│ package          │ {record 13 fields} │
│ patch            │ {record 1 field}   │
│ profile          │ {record 3 fields}  │
│ target           │ {record 3 fields}  │
│ workspace        │ {record 1 field}   │
╰──────────────────┴────────────────────╯
```

We can pipe this into a command that gets the contents of one of the columns:

```shell
> open Cargo.toml | get package
╭───────────────┬────────────────────────────────────╮
│ authors       │ [list 1 item]                      │
│ default-run   │ rsh                                 │
│ description   │ A new type of shell                │
│ documentation │ https://irsh.vercel.app/book/       │
│ edition       │ 2018                               │
│ exclude       │ [list 1 item]                      │
│ homepage      │ https://irsh.vercel.app             │
│ license       │ MIT                                │
│ metadata      │ {record 1 field}                   │
│ name          │ rsh                                 │
│ repository    │ https://github.com/radhesh1/rsh │
│ rust-version  │ 1.60                               │
│ version       │ 0.72.0                             │
╰───────────────┴────────────────────────────────────╯
```

And if needed we can drill down further:

```shell
> open Cargo.toml | get package.version
0.72.0
```

### Plugins

Rsh supports plugins that offer additional functionality to the shell and follow the same structured data model that built-in commands use. There are a few examples in the `crates/rsh_plugin_*` directories.

Plugins are binaries that are available in your path and follow a `rsh_plugin_*` naming convention.
These binaries interact with rsh via a simple JSON-RPC protocol where the command identifies itself and passes along its configuration, making it available for use.
If the plugin is a filter, data streams to it one element at a time, and it can stream data back in return via stdin/stdout.
If the plugin is a sink, it is given the full vector of final data and is given free reign over stdin/stdout to use as it pleases.

The [awesome-rsh repo](https://github.com/radhesh1.awesome-nu#plugins) lists a variety of nu-plugins while the [showcase repo](https://github.com/rsh/showcase) *shows* off informative blog posts that have been written about rsh along with videos that highlight technical
topics that have been presented.

## Goals

Rsh adheres closely to a set of goals that make up its design philosophy. As features are added, they are checked against these goals.

-   First and foremost, Rsh is cross-platform. Commands and techniques should work across platforms and Rsh has [first-class support for Windows, macOS, and Linux](devdocs/PLATFORM_SUPPORT.md).

-   Rsh ensures compatibility with existing platform-specific executables.

-   Rsh's workflow and tools should have the usability expected of modern software in 2022 (and beyond).

-   Rsh views data as either structured or unstructured. It is a structured shell like PowerShell.

-   Finally, Rsh views data functionally. Rather than using mutation, pipelines act as a means to load, change, and save data without mutable state.

## Officially Supported By

Please submit an issue or PR to be added to this list.

-   [zoxide](https://github.com/ajeetdsouza/zoxide)
-   [starship](https://github.com/starship/starship)
-   [oh-my-posh](https://ohmyposh.dev)
-   [Couchbase Shell](https://couchbase.sh)
-   [virtualenv](https://github.com/pypa/virtualenv)
-   [atuin](https://github.com/ellie/atuin)
-   [clap](https://github.com/clap-rs/clap/tree/master/clap_complete_rsh)
-   [Dorothy](http://github.com/bevry/dorothy)
-   [Direnv](https://github.com/direnv/direnv/blob/master/docs/hook.md#rsh)

## Contributing

See [Contributing](CONTRIBUTING.md) for details. Thanks to all the people who already contributed!

<a href="https://github.com/radhesh1/rsh/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=rsh/rsh&max=600" alt="contributor"/>
</a>

## License

The project is made available under the MIT license. See the `LICENSE` file for more information.
