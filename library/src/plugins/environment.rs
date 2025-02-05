use super::super::store::Store;

use wasmtime::*;

//
// Environment
//

/// Plugin environment.
pub struct Environment<StoreT>
where
    StoreT: Store,
{
    /// Wasmtime engine.
    pub engine: Engine,

    /// Store.
    pub store: StoreT,
}

impl<StoreT> Environment<StoreT>
where
    StoreT: Store,
{
    /// Constructor.
    pub fn new(store: StoreT) -> Self {
        Self { engine: Engine::default(), store }
    }
}
