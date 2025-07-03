pub mod bindings {
    use wit_bindgen::generate;

    generate!({
      path: [
        "../plugin_host_api/wit/world.wit",
        "./wit/pluggable.wit"
      ],
      world: "example:pluggable/pluggable",
      generate_all,
      pub_export_macro: true,
      export_macro_name: "export",
    });
}

use bindings::example::pluggable::host_interface;
use bindings::exports::example::pluggable::pluggable_interface::Guest;
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
            let state = host_interface::run_plugin(i, &input);
            println!(
                "pluggable: plugin {i} returned with an index value of {}",
                state.index()
            );
        }
        true
    }
}

bindings::export!(Pluggable with_types_in bindings);
