#![allow(warnings)]
mod plugin_state;
use std::sync::{Arc, Mutex};

use plugin_host_api::host_pluggable_bindings::{
    Pluggable as PluggableWorld,
    example::pluggable::{self, host_interface},
};
use plugin_host_api::host_plugin_bindings::Plugin as PluginWorld;
use plugin_state::{ComponentEngine, ComponentInstance, ComponentState, HostState};

fn main() {
    let mut host = match ComponentEngine::new() {
        Err(e) => {
            eprintln!("{e}");
            return;
        }
        Ok(x) => x,
    };

    let mut host_state = Arc::new(Mutex::new(HostState {
        plugins: Vec::new(),
    }));
    // if let Err(e) = PluginWorld::add_to_linker(&mut host.linker, |s| s) {
    //     eprintln!("{e}");
    //     return;
    // }
    host_state.lock().unwrap().plugins = match register_plugins("../.plugins", &host, &host_state) {
        Err(e) => {
            eprintln!("{e}");
            return;
        }
        Ok(x) => x,
    };

    if std::fs::exists("../target/wasm32-wasip2/debug/pluggable.wasm").unwrap() {
        println!("pluggable.wasm exists.");
    }
    // let mut pluggable = match host.register_component(
    //     "../target/wasm32-wasip2/debug/pluggable.wasm",
    //     |store, component, linker| PluggableWorld::instantiate(store, component, linker),
    // ) {
    //     Err(e) => {
    //         eprintln!("Error registering component: {e}");
    //         return;
    //     }
    //     Ok(p) => p,
    // };

    if let Err(e) = PluggableWorld::add_to_linker(&mut host.linker, |s| s) {
        eprintln!("{e}");
        return;
    }

    let mut pluggable = register_pluggable(
        "../target/wasm32-wasip2/debug/pluggable.wasm",
        &host,
        &host_state,
    )
    .unwrap();

    let res = pluggable
        .instance
        .example_pluggable_pluggable_interface()
        .call_run(&mut pluggable.store, b"b");

    println!("host: pluggable.run() = {res:?}");
}
fn register_pluggable(
    path: &str,
    host: &ComponentEngine,
    host_state: &Arc<Mutex<HostState>>,
) -> wasmtime::Result<ComponentInstance<PluggableWorld>> {
    let component = wasmtime::component::Component::from_file(&host.engine, path)?;
    let mut store = wasmtime::Store::new(&host.engine, ComponentState::new(&host_state));
    let instance = PluggableWorld::instantiate(&mut store, &component, &host.linker)?;
    Ok(ComponentInstance { store, instance })
}
fn register_plugins(
    dir: &str,
    host: &ComponentEngine,
    host_state: &Arc<Mutex<HostState>>,
) -> wasmtime::Result<Vec<ComponentInstance<PluginWorld>>> {
    let mut plugins = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let component = wasmtime::component::Component::from_file(&host.engine, entry?.path())?;
        let mut store = wasmtime::Store::new(&host.engine, ComponentState::new(&host_state));
        let instance = PluginWorld::instantiate(&mut store, &component, &host.linker)?;
        plugins.push(ComponentInstance { store, instance });
    }
    Ok(plugins)
}

use wasmtime::component::Resource;
impl host_interface::Host for ComponentState {
    fn plugins_len(&mut self) -> u32 {
        self.host.lock().unwrap().plugins.len() as u32
    }

    fn run_plugin(
        &mut self,
        index: u32,
        input: Vec<u8>,
    ) -> Resource<pluggable::plugin_interface::ExtensionState> {
        let mut p = self.host.lock().unwrap();
        let mut p = p.plugins.get_mut(index as usize).unwrap();
        let answer = p
            .instance
            .example_plugin_plugin_interface()
            .call_add(&mut p.store, 3, 6);
        println!("host: plugin.add returned {answer:?}");
        // Just to see if the rest of the function worked
        Resource::new_own(1)
    }
}

impl pluggable::plugin_interface::Host for ComponentState {
    fn add(&mut self, a: u32, b: u32) -> u32 {
        todo!()
    }

    fn parse(&mut self, bytes: Vec<u8>) -> Resource<pluggable::plugin_interface::ExtensionState> {
        todo!()
    }
}
impl pluggable::plugin_interface::HostExtensionState for ComponentState {
    fn new(&mut self) -> Resource<pluggable::plugin_interface::ExtensionState> {
        todo!()
    }

    fn index(&mut self, self_: Resource<pluggable::plugin_interface::ExtensionState>) -> u32 {
        todo!()
    }

    fn set_index(
        &mut self,
        self_: Resource<pluggable::plugin_interface::ExtensionState>,
        i: u32,
    ) -> () {
        todo!()
    }

    fn drop(
        &mut self,
        rep: Resource<pluggable::plugin_interface::ExtensionState>,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}
