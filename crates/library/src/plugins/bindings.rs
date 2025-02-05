#![allow(missing_docs)]

wasmtime::component::bindgen!({
    path: "../../assets/wit/floria-plugins.wit",
    with: {
        "floria:plugins/host/nested-list": super::host::List,
        "floria:plugins/host/nested-map": super::host::Map,
    },
    trappable_imports: true,
});
