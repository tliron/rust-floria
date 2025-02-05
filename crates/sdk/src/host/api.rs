use super::super::{host_bindings, normal};

/// Log.
#[macro_export]
macro_rules! log {
    ( $source:expr, $($arg:tt)* ) => (
        $crate::host_bindings::log($source, &::std::format! ( $($arg)* ) )
    );
}

// These wrappers expose normal types, internally converting to and from host types

/// Get node.
pub fn get_node(id: &str) -> Result<normal::Value, String> {
    host_bindings::get_node(id).map(|n| n.into())
}

/// Get relationship.
pub fn get_relationship(id: &str) -> Result<normal::Value, String> {
    host_bindings::get_relationship(id).map(|n| n.into())
}
