use wasmtime::*;

//
// Environment
//

/// Wasm environment for plugins.
///
/// Cloning is cheap and clones always refer to the same shared state.
#[derive(Debug, Default, Clone)]
pub struct Environment {
    /// Wasmtime engine.
    pub engine: Engine,
}
