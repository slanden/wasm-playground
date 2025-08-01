use plugin_api::{
    ExtensionState, ExtensionStateResource, GuestExtensionState, Plugin, PluginInterface,
    host_interface::host_test, implementer_bindings::export,
};

struct MyPlugin;

impl Plugin for MyPlugin {
    fn greeting(name: String) -> String {
        println!("STDIO WORKS!");
        format!("Greetings {name}! I'm a WASI plugin!")
    }
}
impl PluginInterface for MyPlugin {
    type ExtensionState = ExtensionState;
    fn add(a: u32, b: u32) -> u32 {
        println!("Add works!");
        println!("{}+{}={}", a, b, a + b);
        a + b
    }
    fn parse(bytes: Vec<u8>) -> ExtensionStateResource {
        let state = ExtensionState::new();
        if !bytes.get(0).is_some_and(|b| *b == b'(') {
            return ExtensionStateResource::new(state);
        }
        state.set_index(12);

        println!("host_test() returned {}", host_test());
        ExtensionStateResource::new(state)
    }
}

export!(MyPlugin with_types_in plugin_api::implementer_bindings);
