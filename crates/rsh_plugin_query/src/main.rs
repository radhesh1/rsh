use rsh_plugin::{serve_plugin, JsonSerializer};
use rsh_plugin_query::Query;

fn main() {
    serve_plugin(&mut Query {}, JsonSerializer {})
}
