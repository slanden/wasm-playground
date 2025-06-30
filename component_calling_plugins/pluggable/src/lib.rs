pub mod bindings {
    use wit_bindgen::generate;

    generate!({
      path: [
        "../plugin_host_api/wit/world.wit",
        "../plugin_api/wit/world.wit",
        "./wit/pluggable.wit"
      ],
      world: "example:pluggable/pluggable",
      // with: {
      //   "example:plugin/plugin-interface": generate
      // },
      generate_all,
      pub_export_macro: true,
      export_macro_name: "export",
    });
}

pub struct PluginList {
    plugins: Vec<u32>,
}
