use rsh_plugin::{serve_plugin, MsgPackSerializer};
use rsh_plugin_example::Example;

fn main() {
    // When defining your plugin, you can select the Serializer that could be
    // used to encode and decode the messages. The available options are
    // MsgPackSerializer and JsonSerializer. Both are defined in the serializer
    // folder in rsh-plugin.
    serve_plugin(&mut Example {}, MsgPackSerializer {})

    // Note
    // When creating plugins in other languages one needs to consider how a plugin
    // is added and used in rsh.
    // The steps are:
    // - The plugin is register. In this stage rsh calls the binary file of
    //      the plugin sending information using the encoded PluginCall::PluginSignature object.
    //      Use this encoded data in your plugin to design the logic that will return
    //      the encoded signatures.
    //      rsh is expecting and encoded PluginResponse::PluginSignature with all the
    //      plugin signatures
    // - When calling the plugin, rsh sends to the binary file the encoded
    //      PluginCall::CallInfo which has all the call information, such as the
    //      values of the arguments, the name of the signature called and the input
    //      from the pipeline.
    //      Use this data to design your plugin login and to create the value that
    //      will be sent to rsh
    //      rsh expects an encoded PluginResponse::Value from the plugin
    // - If an error needs to be sent back to rsh, one can encode PluginResponse::Error.
    //      This is a labeled error that rsh can format for pretty printing
}
