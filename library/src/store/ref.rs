use super::store::*;

use std::sync::*;

//
// StoreRef
//

/// Common reference type for [Store].
pub type StoreRef<'own> = Arc<Box<&'own dyn Store>>;

//
// ToStoreRef
//

/// To store reference.
pub trait ToStoreRef<'own> {
    /// To store reference.
    fn to_ref(&'own self) -> StoreRef<'own>;
}

impl<'own, StoreT> ToStoreRef<'own> for StoreT
where
    StoreT: Store,
{
    fn to_ref(&'own self) -> StoreRef<'own> {
        StoreRef::new(Box::new(self))
    }
}
