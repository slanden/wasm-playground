//! Setup code for a plugin host, without project-specific
//! impls

use {
    std::fs,
    wasmtime::{
        Config, Engine, Result, Store,
        component::{Component, Linker, ResourceTable},
    },
    wasmtime_wasi::p2::{IoView, WasiCtx, WasiView},
};

pub struct State<P: Send> {
    ctx: WasiCtx,
    table: ResourceTable,
    // Not necessary, but maybe useful to separate logic
    pub host: PluginHost<P>,
}
impl<P: Send> State<P> {
    fn new() -> Result<Self> {
        Ok(Self {
            ctx: WasiCtx::builder().inherit_stdio().build(),
            table: ResourceTable::new(),
            host: PluginHost::new()?,
        })
    }
}
impl<P: Send> WasiView for State<P> {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
impl<P: Send> IoView for State<P> {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

pub struct PluginState<P: Send> {
    pub store: Store<State<P>>,
    pub instance: P,
}

pub struct PluginHost<P: Send> {
    pub plugins: Vec<PluginState<P>>,
    engine: Engine,
    pub linker: Linker<State<P>>,
}

impl<P: Send> PluginHost<P> {
    pub fn new() -> Result<Self> {
        let mut config = Config::default();
        config.wasm_component_model(true);
        config.debug_info(true);
        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

        Ok(Self {
            plugins: Vec::new(),
            engine,
            linker,
        })
    }
    pub fn register_component<F>(
        &mut self,
        path: &str,
        mut instantiate: F,
    ) -> Result<PluginState<P>>
    where
        F: FnMut(&mut Store<State<P>>, &Component, &Linker<State<P>>) -> Result<P>,
    {
        let component = Component::from_file(&self.engine, path)?;
        dbg!("1");
        let mut store = Store::new(&self.engine, State::new()?);
        dbg!("2");
        let instance = instantiate(&mut store, &component, &self.linker)?;

        Ok(PluginState { store, instance })
    }
    pub fn register_plugins<F>(&mut self, dir: &str, mut instantiate: F) -> Result<()>
    where
        F: FnMut(&mut Store<State<P>>, &Component, &Linker<State<P>>) -> Result<P>,
    {
        for entry in fs::read_dir(dir)? {
            let component = Component::from_file(&self.engine, entry?.path())?;
            let mut store = Store::new(&self.engine, State::new()?);
            let instance = instantiate(&mut store, &component, &self.linker)?;
            self.plugins.push(PluginState { store, instance });
        }
        Ok(())
    }
}
