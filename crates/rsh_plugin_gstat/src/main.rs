use rsh_plugin::{serve_plugin, MsgPackSerializer};
use rsh_plugin_gstat::GStat;

fn main() {
    serve_plugin(&mut GStat::new(), MsgPackSerializer {})
}
