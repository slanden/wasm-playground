mod plugin_host;
use plugin_host::{PluginHost, State};
use plugin_host_api::example::plugin_host::host_interface::HostPluginList;
use plugin_host_api::exports::example::plugin_host::pluggable_interface::PluginList;
// use plugin_host_api::PluginList;
use plugin_host_api::{PluginHost as ParsePlugin, example::plugin_host::host_interface::Host};
// use plugin_host_api::exports::example::plugin_host::pluggable_interface::PluginList
use plugin_host_api::example::plugin_host::host_interface::ExtensionState;

// impl plugin_set::HostPluginList for PluginInstances {
//     fn new(&mut self,) -> wasmtime::component::Resource<PluginList> {
//         todo!()
//     }

//     fn drop(&mut self,rep:wasmtime::component::Resource<PluginList>) -> wasmtime::Result<()> {
//         todo!()
//     }
// }

fn main() {
    let mut host = match PluginHost::<ParsePlugin>::new() {
        Err(e) => {
            eprintln!("{e}");
            return;
        }
        Ok(x) => x,
    };
    if let Err(e) = ParsePlugin::add_to_linker(&mut host.linker, |s| s) {
        eprintln!("{e}");
        return;
    }
    if let Err(e) = host.register_plugins("../.plugins", |store, component, linker| {
        ParsePlugin::instantiate(store, component, linker)
    }) {
        eprintln!("{e}");
        return;
    }

    let pluggable = match host.register_component(
        "target/wasm32-wasip2/debug/pluggable.wasm",
        |store, component, linker| ParsePlugin::instantiate(store, component, linker),
    ) {
        Err(e) => {
            eprintln!("{e}");
            return;
        }
        Ok(p) => p,
    };
    // let plugin_list = wasmtime::component::Resource::new_borrow(plugin_set::PluginList);
    // pluggable
    //     .instance
    //     .example_plugin_host_pluggable_interface()
    //     .call_run(
    //         &mut pluggable.store,
    //         &host
    //             .plugins
    //             .iter()
    //             .map(|p| p.instance)
    //             .collect::<Vec<ParsePlugin>>(),
    //         b"[pdml]",
    //     );

    // let index = host
    //     .plugins
    //     .get_mut(0)
    //     .and_then(|c| {
    //         println!("Component 0");

    //         c.instance
    //             .example_plugin_host_plugin_interface()
    //             .call_add(&mut c.store, 1, 2)
    //             .ok();

    //         let res = match c
    //             .instance
    //             .example_plugin_host_plugin_interface()
    //             .call_parse(&mut c.store, b"([]")
    //         {
    //             Ok(state) => state,
    //             _ => return None,
    //         };

    //         let i = c
    //             .instance
    //             .example_plugin_host_plugin_interface()
    //             .extension_state()
    //             .call_index(&mut c.store, res)
    //             .unwrap();

    //         return Some(i);
    //     })
    //     .unwrap();

    // println!("index is {}", index);
}

impl<P: Send> Host for State<P> {
    fn host_test(&mut self) -> bool {
        self.host.host_test()
    }
}
impl<P: Send> HostPluginList for State<P> {
    fn new(&mut self) -> wasmtime::component::Resource<PluginList> {
        self.host.new()
    }

    #[doc = " Length of the plugin list"]
    fn len(&mut self, self_: wasmtime::component::Resource<PluginList>) -> u32 {
        self.host.len(self_)
    }

    #[doc = " Run the plugin at `index`"]
    fn run_plugin(
        &mut self,
        self_: wasmtime::component::Resource<PluginList>,
        index: u32,
        input: wasmtime::component::__internal::Vec<u8>,
    ) -> wasmtime::component::Resource<ExtensionState> {
        self.host.run_plugin(self_, index, input)
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<PluginList>) -> wasmtime::Result<()> {
        self.host.drop(rep)
    }
}

impl<P: Send> Host for PluginHost<P> {
    fn host_test(&mut self) -> bool {
        true
    }
}
impl<P: Send> HostPluginList for PluginHost<P> {
    fn new(&mut self) -> wasmtime::component::Resource<PluginList> {
        self.plugins
    }

    #[doc = " Length of the plugin list"]
    fn len(&mut self, self_: wasmtime::component::Resource<PluginList>) -> u32 {
        todo!()
    }

    #[doc = " Run the plugin at `index`"]
    fn run_plugin(
        &mut self,
        self_: wasmtime::component::Resource<PluginList>,
        index: u32,
        input: wasmtime::component::__internal::Vec<u8>,
    ) -> wasmtime::component::Resource<ExtensionState> {
        todo!()
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<PluginList>) -> wasmtime::Result<()> {
        todo!()
    }
}
// impl<P: Send> plugin_set::HostPluginList for PluginHost<P> {
//     fn drop(
//         &mut self,
//         rep: wasmtime::component::Resource<plugin_set::PluginList>,
//     ) -> wasmtime::Result<()> {
//         Ok(())
//     }
// }
