#![allow(missing_docs)]

wasmtime::component::bindgen!({
    path: "../assets/wit/floria-plugins.wit",
    with: {
        "floria:plugins/floria/nested-list": super::host::List,
        "floria:plugins/floria/nested-map": super::host::Map,
    },
    imports: { default: trappable },
});

// Used to be:
// trappable_imports: true,
