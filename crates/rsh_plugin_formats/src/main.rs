use rsh_plugin::{serve_plugin, MsgPackSerializer};
use rsh_plugin_formats::FromCmds;

fn main() {
    serve_plugin(&mut FromCmds, MsgPackSerializer {})
}
