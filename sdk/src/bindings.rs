#![allow(missing_docs)]

wit_bindgen::generate!({
    path: "../assets/wit/floria-plugins.wit",
    default_bindings_module: "::floria_plugin_sdk::bindings",
    export_macro_name: "export_dispatcher",
    pub_export_macro: true,
});
