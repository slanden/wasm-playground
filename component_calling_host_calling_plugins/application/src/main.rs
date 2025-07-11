#![allow(warnings)]
mod plugin_state;
use plugin_host_api::{
    host_pluggable_bindings::{
        self, Pluggable as PluggableWorld, example::plugin::host_interface::Host,
    },
    host_plugin_bindings::Plugin as PluginWorld,
};
use plugin_state::{ComponentEngine, ComponentInstance, ComponentState, HostState};
use {
    std::sync::{Arc, Mutex},
    wasmtime::component::{HasSelf, Resource, ResourceAny, ResourceTable},
};

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
        table: ResourceTable::new(),
    }));

    // This had to be before instantiation, and even though
    // there are two worlds, they both try to add the same
    // interface (`host-interface`), so only call one.
    // Prefer to keep the one that actually imports the
    // interface directly.
    if let Err(e) = PluggableWorld::add_to_linker::<_, HasSelf<_>>(&mut host.linker, |s| s) {
        eprintln!("{e}");
        return;
    }

    if std::fs::exists("../target/wasm32-wasip2/debug/pluggable.wasm").unwrap() {
        println!("pluggable.wasm exists.");
    }

    let mut pluggable = match register_pluggable(
        "../target/wasm32-wasip2/debug/pluggable.wasm",
        &host,
        &host_state,
    ) {
        Err(e) => {
            eprintln!("Error registering component: {e}");
            return;
        }
        Ok(p) => p,
    };

    host_state.lock().unwrap().plugins = match register_plugins("../.plugins", &host, &host_state) {
        Err(e) => {
            eprintln!("{e}");
            return;
        }
        Ok(x) => x,
    };

    let res = pluggable
        .instance
        .example_plugin_pluggable_interface()
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
        dbg!(0);
        let component = wasmtime::component::Component::from_file(&host.engine, entry?.path())?;
        dbg!(1);
        let mut store = wasmtime::Store::new(&host.engine, ComponentState::new(&host_state));
        dbg!(2);
        let instance = PluginWorld::instantiate(&mut store, &component, &host.linker)?;
        dbg!(3);
        plugins.push(ComponentInstance { store, instance });
    }
    Ok(plugins)
}

impl host_pluggable_bindings::example::plugin::host_interface::Host for ComponentState {
    fn plugins_len(&mut self) -> u32 {
        self.host.lock().unwrap().plugins.len() as u32
    }

    fn run_plugin(
        &mut self,
        index: u32,
        input: Vec<u8>,
        state: wasmtime::component::Resource<
            plugin_host_api::host_pluggable_bindings::ExtensionState,
        >,
    ) -> () {
        let mut p = self.host.lock().unwrap();
        let mut p = p.plugins.get_mut(index as usize).unwrap();
        p.instance
            .example_plugin_plugin_interface()
            // .call_add(&mut p.store, 3, 6);
            .call_parse(&mut p.store, b"", state)
            .unwrap();
        println!("host: plugin.parse returned");
    }
}
impl host_pluggable_bindings::example::plugin::pluggable_to_plugin::HostExtensionState
    for ComponentState
{
    fn new(
        &mut self,
    ) -> wasmtime::component::Resource<plugin_host_api::host_pluggable_bindings::ExtensionState>
    {
        self.table
            .push(plugin_host_api::host_pluggable_bindings::ExtensionState::new())
            .unwrap()
    }

    fn index(
        &mut self,
        self_: wasmtime::component::Resource<
            plugin_host_api::host_pluggable_bindings::ExtensionState,
        >,
    ) -> u32 {
        *self.table.get(&self_).unwrap().index.borrow() as u32
    }

    fn set_index(
        &mut self,
        self_: wasmtime::component::Resource<
            plugin_host_api::host_pluggable_bindings::ExtensionState,
        >,
        i: u32,
    ) -> () {
        *self.table.get_mut(&self_).unwrap().index.borrow_mut() = i;
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<
            plugin_host_api::host_pluggable_bindings::ExtensionState,
        >,
    ) -> wasmtime::Result<()> {
        Ok(self.table.delete(rep).and(Ok(()))?)
    }
}
impl host_pluggable_bindings::example::plugin::pluggable_to_plugin::Host for ComponentState {}
