# rs-protocol

The rsh-protocol crate holds the definitions of structs/traits that are used throughout rsh. This gives us one way to expose them to many other crates, as well as make these definitions available to each other, without causing mutually recursive dependencies.