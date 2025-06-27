use wit_bindgen::generate;

generate!({
  path: "./wit/pluggable.wit",
  world: "pluggable"
  pub_export_macro: true,
  export_macro_name: "export",
});

use bindings::Guest;

struct Pluggable;

impl Guest for Pluggable {
    fn run(input: Vec<u8>) -> bool {
        let count = pluggable::plugin_set::count();
        let mut results = Vec::new();
        for i in 0..count {
            if let Some(plugin) = pluggable::plugin_set::get(i) {
                results.push(plugin.parse(input.clone()));
            }
        }
        true
    }
}

bindings::export!(Pluggable);
