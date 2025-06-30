#![allow(warnings)]
mod plugin_host;
use plugin_host::{PluginHost, State};
use plugin_host_api::PluginHost as PluginWorld;
use plugin_host_api::example::plugin_host::{host_interface, plugin_interface};

fn main() {
    let mut host = match PluginHost::<PluginWorld>::new() {
        Err(e) => {
            eprintln!("{e}");
            return;
        }
        Ok(x) => x,
    };
    if let Err(e) = PluginWorld::add_to_linker(&mut host.linker, |s| s) {
        eprintln!("{e}");
        return;
    }
    if let Err(e) = host.register_plugins("../.plugins", |store, component, linker| {
        PluginWorld::instantiate(store, component, linker)
    }) {
        eprintln!("{e}");
        return;
    }

    if std::fs::exists("../target/wasm32-wasip2/debug/pluggable.wasm").unwrap() {
        println!("pluggable.wasm exists.");
    }
    let mut pluggable = match host.register_component(
        "../target/wasm32-wasip2/debug/pluggable.wasm",
        |store, component, linker| PluginWorld::instantiate(store, component, linker),
    ) {
        Err(e) => {
            eprintln!("Error registering component: {e}");
            return;
        }
        Ok(p) => p,
    };

    // let res = pluggable
    //     .instance
    //     .example_plugin_host_pluggable_interface()
    //     .call_run(&mut pluggable.store, b"");
    // println!("pluggable.run() = {:?}", res);
}
use wasmtime::component::Resource;
impl host_interface::Host for State<PluginWorld> {
    fn plugins_len(&mut self) -> u32 {
        todo!()
    }

    fn run_plugin(
        &mut self,
        index: u32,
        input: Vec<u8>,
    ) -> Resource<plugin_interface::ExtensionState> {
        todo!()
    }
}
impl plugin_interface::Host for State<PluginWorld> {
    fn add(&mut self, a: u32, b: u32) -> u32 {
        todo!()
    }

    fn parse(&mut self, bytes: Vec<u8>) -> Resource<plugin_interface::ExtensionState> {
        todo!()
    }
}
impl plugin_interface::HostExtensionState for State<PluginWorld> {
    fn new(&mut self) -> Resource<plugin_interface::ExtensionState> {
        todo!()
    }

    fn index(&mut self, self_: Resource<plugin_interface::ExtensionState>) -> u32 {
        todo!()
    }

    fn set_index(&mut self, self_: Resource<plugin_interface::ExtensionState>, i: u32) -> () {
        todo!()
    }

    fn drop(&mut self, rep: Resource<plugin_interface::ExtensionState>) -> wasmtime::Result<()> {
        todo!()
    }
}
