use super::{super::super::store::*, plugin::*};

use std::sync::*;

/// Common reference type for [DispatchPlugin].
pub type DispatchPluginRef<StoreT> = Arc<Mutex<DispatchPlugin<StoreT>>>;

impl<StoreT> Into<DispatchPluginRef<StoreT>> for DispatchPlugin<StoreT>
where
    StoreT: Store,
{
    fn into(self) -> DispatchPluginRef<StoreT> {
        DispatchPluginRef::new(self.into())
    }
}
