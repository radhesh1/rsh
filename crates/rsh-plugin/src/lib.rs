#![allow(clippy::needless_doctest_main)]
//! # Rsh Plugin: Plugin library for rsh
//!
//! This crate contains the interface necessary to build rsh plugins in Rust.
//! Additionally, it contains public, but undocumented, items used by rsh itself
//! to interface with rsh plugins. This documentation focuses on the interface
//! needed to write an independent plugin.
//!
//! rsh plugins are stand-alone applications that communicate with rsh
//! over stdin and stdout using a standardizes serialization framework to exchange
//! the typed data that rsh commands utilize natively.
//!
//! A typical plugin application will define a struct that implements the [Plugin]
//! trait and then, in it's main method, pass that [Plugin] to the [serve_plugin]
//! function, which will handle all of the input and output serialization when
//! invoked by rsh.
//!
//! ```
//! use rsh_plugin::{EvaluatedCall, LabeledError, MsgPackSerializer, Plugin, serve_plugin};
//! use rsh_protocol::{PluginSignature, Value};
//!
//! struct MyPlugin;
//!
//! impl Plugin for MyPlugin {
//!     fn signature(&self) -> Vec<PluginSignature> {
//!         todo!();
//!     }
//!     fn run(
//!         &mut self,
//!         name: &str,
//!         call: &EvaluatedCall,
//!         input: &Value
//!     ) -> Result<Value, LabeledError> {
//!         todo!();
//!     }
//! }
//!
//! fn main() {
//!    serve_plugin(&mut MyPlugin{}, MsgPackSerializer)
//! }
//! ```
//!
//! rsh's source tree contains a
//! [Plugin Example](https://github.com/radhesh1/rsh/tree/main/crates/rsh_plugin_example)
//! that demonstrates the full range of plugin capabilities.
mod plugin;
mod protocol;
mod serializers;

pub use plugin::{get_signature, serve_plugin, Plugin, PluginDeclaration};
pub use protocol::{EvaluatedCall, LabeledError, PluginResponse};
pub use serializers::{json::JsonSerializer, msgpack::MsgPackSerializer, EncodingType};
