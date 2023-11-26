use rsh_plugin::{serve_plugin, JsonSerializer};
use rsh_plugin_inc::Inc;

fn main() {
    serve_plugin(&mut Inc::new(), JsonSerializer {})
}
