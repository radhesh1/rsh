# rsh_plugin_formats
A rsh plugin to convert data to rsh' tables.

# support commands:
1. from eml - original ported from rsh core.
2. from ics - original ported from rsh core.
3. from ini - original ported from rsh core.
4. from vcf - original ported from rsh core.

# Prerequisite
`rsh`, It's a rsh plugin, so you need it.

# Usage
1. compile the binary: `cargo build`
2. register plugin(assume it's compiled in ./target/debug/):
```
register ./target/debug/rsh_plugin_formats
```
