// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
Floria Plugin SDK.
*/

/// WIT bindings.
pub mod bindings;

/// Data.
pub mod data;

/// Plugin host.
pub mod host;

#[allow(unused_imports)]
pub use bindings::{
    export_dispatcher, exports::floria::plugins::dispatch as dispatch_bindings,
    floria::plugins::floria as floria_bindings,
};
