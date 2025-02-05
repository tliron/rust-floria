use super::super::store::*;

use wasmtime::Engine;

//
// Environment
//

/// Plugin environment.
pub struct Environment<StoreT>
where
    StoreT: StoreClient,
{
    /// Wasmtime engine.
    pub engine: Engine,

    /// Store.
    pub store: StoreT,
}

impl<StoreT> Environment<StoreT>
where
    StoreT: StoreClient,
{
    /// Constructor.
    pub fn new(store: StoreT) -> Self {
        Self { engine: Engine::default(), store }
    }
}
