pub mod bindings {
    // use super::ExtensionState;
    use wit_bindgen::generate;
    generate!({
      // path: [
      //   "../plugin_host_api/wit/world.wit",
      //   "./wit/pluggable.wit"
      // ],
      path: "../wit/all.wit",
      world: "pluggable",
      // with: {
      //   "example:plugin/plugin-interface/extension-state": plugin_api::ExtensionState
      // },
      // generate_all,
      pub_export_macro: true,
      export_macro_name: "export",
    });
}
use std::cell::RefCell;

// use bindings::exports::example::plugin::pluggable_interface::GuestExtensionState;
// use plugin_api::ExtensionState;
// use plugin_api::implementer_bindings::exports::example::plugin::plugin_interface::GuestExtensionState;
// use bindings::example::pluggable::host_interface;
use bindings::example::plugin::host_interface;
// use bindings::example::plugin::host_interface::ExtensionState as ExtensionStateResource;
use bindings::example::plugin::pluggable_to_plugin::ExtensionState as ExtensionStateResource;
// use bindings::exports::example::plugin::pluggable_to_plugin::ExtensionState;
use bindings::exports::example::plugin::pluggable_interface::Guest;

struct Pluggable;

impl Guest for Pluggable {
    fn run(input: Vec<u8>) -> bool {
        println!("pluggable: Hello from `pluggable.run`");
        println!(
            "pluggable: plugin length = {}",
            host_interface::plugins_len()
        );
        for i in 0..host_interface::plugins_len() {
            println!("pluggable: call plugin {i}");
            let state = ExtensionStateResource::new();
            // If the `ExtensionState` here was created in the
            // plugin (its handle returned through the host
            // function to here) it will not be able to be
            // manipulated here, even though we can see the
            // methods because of the below issue
            host_interface::run_plugin(i, &input, &state);
            println!("pluggable: ran plugin");
            // ! Right here, the `.set_index()` uses
            // ! pluggable's table, not the table of the
            // ! plugin (even if it was created in the
            // ! plugin), so `state` lives in plugin's
            // ! `self.table`, and not pluggable's `self.table`.
            state.set_index(12);
            println!(
                "pluggable: plugin {i} returned with an index value of {}",
                state.index()
            );
        }
        true
    }
}
impl bindings::exports::example::plugin::pluggable_to_plugin::Guest for Pluggable {
    type ExtensionState = ExtensionState;
}

pub struct ExtensionState {
    // RefCell so it can be mutated in the interface
    pub index: RefCell<u32>,
}

impl bindings::exports::example::plugin::pluggable_to_plugin::GuestExtensionState
    for ExtensionState
{
    fn new() -> Self {
        ExtensionState {
            index: RefCell::new(0),
        }
    }

    fn index(&self) -> u32 {
        *self.index.borrow()
    }

    fn set_index(&self, i: u32) {
        *self.index.borrow_mut() = i;
    }
}

bindings::export!(Pluggable with_types_in bindings);
