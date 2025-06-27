use std::{cell::RefCell, rc::Rc};

pub mod implementer_bindings {
    use wit_bindgen::generate;

    generate!({
      path: "./wit/plugin.wit",
      pub_export_macro: true,
      export_macro_name: "export",
    });
}

pub use implementer_bindings::{
    export,
    exports::example::plugin::plugin_interface::{
        ExtensionState as ExtensionStateResource, Guest as PluginInterface, GuestExtensionState,
    },
    Guest as Plugin,
};

pub struct ExtensionState {
    // Rc<RefCell so it can be mutated in the interface
    pub index: Rc<RefCell<u32>>,
}

impl GuestExtensionState for ExtensionState {
    fn new() -> Self {
        println!("Plugin ExtensionState::new()");
        ExtensionState {
            index: Rc::new(RefCell::new(0)),
        }
    }

    fn index(&self) -> u32 {
        *self.index.borrow()
    }

    fn set_index(&self, i: u32) {
        *self.index.borrow_mut() = i;
    }
}
