use super::super::store::Store;

use {std::marker::*, wasmtime::*};

//
// Environment
//

/// Plugin environment.
pub struct Environment<StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    /// Wasmtime engine.
    pub engine: Engine,

    /// Store.
    pub store: StoreT,

    annotations: PhantomData<AnnotatedT>,
}

impl<StoreT, AnnotatedT> Environment<StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    /// Constructor.
    pub fn new(store: StoreT) -> Self {
        Self { engine: Engine::default(), store, annotations: PhantomData }
    }
}
