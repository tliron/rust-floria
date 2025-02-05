use super::super::{data, dispatch_bindings, floria_bindings};

/// Log.
#[macro_export]
macro_rules! log {
    ( $source:expr, $($arg:tt)* ) => (
        $crate::floria_bindings::log($source, &::std::format! ( $($arg)* ) )
    );
}

// These wrappers expose dispatch types, internally converting to and from host types

/// Get entity.
pub fn get_entity(id: &dispatch_bindings::Id) -> Result<data::Value, String> {
    floria_bindings::get_entity(&id.clone().into()).map(|n| n.into())
}
