#![allow(missing_docs)]

wasmtime::component::bindgen!({
    path: "../assets/wit/floria-plugins.wit",
    with: {
        "floria:plugins/floria/nested-list": super::host::List,
        "floria:plugins/floria/nested-map": super::host::Map,
    },
    trappable_imports: true,
});
