//! Setup code for a plugin, without project-specific
//! impls

use {
    plugin_host_api::{host_pluggable_bindings::Pluggable, host_plugin_bindings::Plugin},
    std::sync::{Arc, Mutex},
    wasmtime::{
        Config, Engine, Result, Store,
        component::{Component, Instance, Linker, ResourceAny, ResourceTable},
    },
    wasmtime_wasi::p2::{IoView, WasiCtx, WasiView},
};

// Should be unique to each component, or maybe even each
// instance
pub struct ComponentState {
    ctx: WasiCtx,
    pub table: ResourceTable,
    // Arc<Mutex was necessary to allow a component
    // (Pluggable) to call a host function that relies on
    // host data
    pub host: Arc<Mutex<HostState>>,
}
impl ComponentState {
    pub fn new(host: &Arc<Mutex<HostState>>) -> Self {
        Self {
            ctx: WasiCtx::builder().inherit_stdio().build(),
            table: ResourceTable::new(),
            host: host.clone(),
        }
    }
}
impl WasiView for ComponentState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
impl IoView for ComponentState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

pub struct ComponentInstance<P> {
    // Stored because every function needs it passed in
    pub store: Store<ComponentState>,
    pub instance: P,
}

// Okay to reuse for multiple component worlds as long as
// the state inside the linker carefully avoids being
// specific to a single component world
pub struct ComponentEngine {
    pub engine: Engine,
    pub linker: Linker<ComponentState>,
}

pub struct HostState {
    /// The list of plugins meant for a Pluggable to iterate
    /// over
    pub plugins: Vec<ComponentInstance<Plugin>>,
    pub table: ResourceTable,
}
impl HostState {
    pub fn new_extension_state(
        &mut self,
    ) -> wasmtime::component::Resource<plugin_host_api::host_pluggable_bindings::example::plugin::pluggable_to_plugin::ExtensionState>{
        dbg!(0);
        let s = self
            .table
            .push(plugin_host_api::ExtensionState::new())
            .unwrap();
        dbg!(1);
        s
    }
}

impl ComponentEngine {
    pub fn new() -> Result<Self> {
        let mut config = Config::default();
        config.wasm_component_model(true);
        config.debug_info(true);
        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

        Ok(Self { engine, linker })
    }
}
