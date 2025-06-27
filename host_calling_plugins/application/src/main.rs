mod plugin_host;
use plugin_host::{PluginHost, State};
use plugin_host_api::{ParsePlugin, example::parse_plugin::host_interface::Host};

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

    let index = host
        .plugins
        .get_mut(0)
        .and_then(|c| {
            println!("Component 0");

            c.instance
                .example_parse_plugin_plugin_interface()
                .call_add(&mut c.store, 1, 2)
                .ok();

            let res = match c
                .instance
                .example_parse_plugin_plugin_interface()
                .call_parse(&mut c.store, b"([]")
            {
                Ok(state) => state,
                _ => return None,
            };

            let i = c
                .instance
                .example_parse_plugin_plugin_interface()
                .extension_state()
                .call_index(&mut c.store, res)
                .unwrap();

            return Some(i);
        })
        .unwrap();

    println!("index is {}", index);
}

impl<P: Send> Host for State<P> {
    fn host_test(&mut self) -> bool {
        self.host.host_test()
    }
}

impl<P: Send> Host for PluginHost<P> {
    fn host_test(&mut self) -> bool {
        true
    }
}
