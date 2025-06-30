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

use bindings::exports::example::pluggable::pluggable_interface::Guest;
struct Pluggable;

impl Guest for Pluggable {
    fn run(input: Vec<u8>) -> bool {
        println!("Hello from `pluggable.run`");
        true
    }
}

bindings::export!(Pluggable with_types_in bindings);
