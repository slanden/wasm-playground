use plugin_api::export;
use plugin_api::implementer_bindings::{
    example::plugin::pluggable_to_plugin::ExtensionState as ExtensionStateResource,
    exports::example::plugin::plugin_interface::Guest,
};

struct MyPlugin;

impl Guest for MyPlugin {
    fn add(a: u32, b: u32) -> u32 {
        println!("custom_plugin: Add works!");
        println!("{}+{}={}", a, b, a + b);
        a + b
    }

    fn parse(bytes: Vec<u8>, state: &ExtensionStateResource) {
        println!("custom_plugin: Hello from `custom_plugin.parse`");
        // let state = ExtensionStateResource::new();
        // if !bytes.get(0).is_some_and(|b| *b == b'(') {
        //     return state;
        // }

        println!("plugin ext state: {:?}", state);
        state.set_index(18);
        println!("plugin ext state index: {:?}", state.index());
    }
}

export!(MyPlugin with_types_in plugin_api::implementer_bindings);
