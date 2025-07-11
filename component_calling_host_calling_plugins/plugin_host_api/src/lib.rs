use std::cell::RefCell;

pub mod host_pluggable_bindings {
    pub use crate::ExtensionState;
    wasmtime::component::bindgen!({
      async: false,
      path: "../wit/all.wit",
      world: "pluggable",
      with: {
        "example:plugin/pluggable-to-plugin/extension-state": ExtensionState
      },
      additional_derives: [
          Eq,
          PartialEq,
          Ord,
          PartialOrd
      ]
    });
}
pub mod host_plugin_bindings {
    pub use crate::ExtensionState;
    wasmtime::component::bindgen!({
      async: false,
      path: "../wit/all.wit",
      world: "plugin",
      with: {
        "example:plugin/pluggable-to-plugin/extension-state": ExtensionState
      },
      additional_derives: [
          Eq,
          PartialEq,
          Ord,
          PartialOrd
      ]
    });
}

pub struct ExtensionState {
    // RefCell so it can be mutated in the interface
    pub index: RefCell<u32>,
}
impl ExtensionState {
    pub fn new() -> Self {
        ExtensionState {
            index: RefCell::new(0),
        }
    }

    pub fn index(&self) -> u32 {
        *self.index.borrow()
    }

    pub fn set_index(&self, i: u32) {
        *self.index.borrow_mut() = i;
    }
}
