## Project Structure
| Path | Description |
|------|-------------|
|application/| The application which implements the host and runs plugins|
|application/main.rs| Implements host interfaces (business logic)
|application/plugin_host.rs| Implements boilerplate with as little business logic as possible, although a clean abstraction wasn't found|
|pluggable/| A component that can be plugged into (supposed to call multiple other components that implement the "plugin" interface)
|custom_plugin/| An example Rust implementation of a plugin, using the *plugin_api*|
|plugin_api/| A convenience layer and bindings for Rust plugin implementers|
|plugin_host_api/| Generated Rust bindings for the host|
|.plugins/| A directory to hold built plugins that can be run|

## Prerequisites
Run
```
rustup target add wasm32-wasip2
```

Also this for some reason
```
cargo install wasm-tools
```

> Important: It's not possible to have a single crate generate both host and plugin bindings via *wasmtime* and *wit-bindgen* respectively, i.e. cannot use `wasmtime::component::bindgen!` and `wit_bindgen::generate!` in the same crate.

## Development
When making changes to the *custom_component*, call `cargo component build` and move it to *.plugins/*.

It's usually fine to make changes to anything else, but sometimes a window reload is necessary for generated bindings to reflect.
