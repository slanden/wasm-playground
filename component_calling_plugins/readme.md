## Project Structure
| Path | Description |
|------|-------------|
|host.js| A Node application which implements the host and loads components (some of which are "plugins")|
|custom_plugin/| An example Rust implementation of a plugin, using the *plugin_api*|
|pluggable/| An example of a component that calls plugins
|plugin_api/| A convenience layer and bindings for Rust plugin implementers|
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
When making changes to the *custom_plugin*, call `cargo component build` and move it to *.plugins/*.

It's usually fine to make changes to anything else, but sometimes a window reload is necessary for generated bindings to reflect.
