#![warn(missing_docs)]

/*!
Plugin SDK.
*/

mod errors;
mod macros;

/// WIT bindings.
pub mod bindings;

/// Data.
pub mod data;

/// Plugin host.
pub mod host;

/// Normal value types.
pub mod normal;

#[allow(unused_imports)]
pub use {
    bindings::{
        export_dispatcher, exports::floria::plugins::dispatcher as dispatcher_bindings,
        floria::plugins::host as host_bindings,
    },
    errors::*,
};
