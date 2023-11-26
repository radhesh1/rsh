rsh-glob
====

Support for matching file paths against Unix shell style patterns.

## Usage

To use `rsh-glob`, add this to your `Cargo.toml`:

```toml
[dependencies]
rsh-glob = "0.60.0"
```

## Examples

Print all jpg files in /media/ and all of its subdirectories.

```rust
use rsh_rsh_glob::glob;

for entry in glob("/media/**/*.jpg").expect("Failed to read glob pattern") {
    match entry {
        Ok(path) => println!("{:?}", path.display()),
        Err(e) => println!("{:?}", e),
    }
}
```
