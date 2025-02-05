use super::store::*;

use std::sync::*;

//
// StoreRef
//

/// Common reference type for [Store].
pub type StoreRef<'own> = Arc<Box<&'own dyn Store>>;

//
// StoreToRef
//

/// Create a [StoreRef].
pub trait StoreToRef<'own> {
    /// Create a [StoreRef].
    fn to_ref(&'own self) -> StoreRef<'own>;
}

impl<'own, StoreT> StoreToRef<'own> for StoreT
where
    StoreT: Store,
{
    fn to_ref(&'own self) -> StoreRef<'own> {
        StoreRef::new(Box::new(self))
    }
}
