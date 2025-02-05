#![allow(missing_docs)]

wasmtime::component::bindgen!({
    path: "../assets/wit/floria-plugins.wit",
    with: {
        "floria:plugins/floria/any-list": super::host::List,
        "floria:plugins/floria/any-map": super::host::Map,
        "floria:plugins/floria/any-call": super::host::Call,
    },
    imports: { default: trappable },
});

// Used to be:
// trappable_imports: true,
