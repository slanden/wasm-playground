#![allow(warnings)]
use std::sync::{Arc, Mutex};

pub mod implementer_bindings {
    use wit_bindgen::generate;

    generate!({
      path: "../wit/all.wit",
      world: "plugin",
      pub_export_macro: true,
      export_macro_name: "export",
    });
}

pub use implementer_bindings::export;
