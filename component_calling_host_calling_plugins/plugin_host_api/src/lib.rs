pub mod host_pluggable_bindings {
    wasmtime::component::bindgen!({
      async: false,
      path: "../pluggable/wit/pluggable.wit",
      additional_derives: [
          Eq,
          PartialEq,
          Ord,
          PartialOrd
      ]
    });
}
pub mod host_plugin_bindings {
    wasmtime::component::bindgen!({
      async: false,
      path: "../plugin_api/wit/world.wit",
      additional_derives: [
          Eq,
          PartialEq,
          Ord,
          PartialOrd
      ]
    });
}
