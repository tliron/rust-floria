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

#[allow(unused_imports)]
pub use {
    bindings::{
        export_dispatcher, exports::floria::plugins::dispatch as dispatch_bindings,
        floria::plugins::floria as floria_bindings,
    },
    errors::*,
};
