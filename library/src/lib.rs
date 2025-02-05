// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
Floria is a data system for cloud orchestration.

It consists of a data model (data-driven) integrated with a plugin execution mechanism
(event-driven), which has first-class support for [Wasm](https://webassembly.org/).

For a Floria-based orchestrator, see [Khutulun](https://github.com/tliron/rust-khutulun).

For a [TOSCA](https://www.oasis-open.org/committees/tosca/) frontend for Floria, see
[Puccini](https://github.com/tliron/rust-puccini).

For more information and usage examples see the
[home page](https://github.com/tliron/floria).
*/

mod data;
mod errors;
mod store;

/// Plugins.
#[cfg(feature = "plugins")]
pub mod plugins;

#[allow(unused_imports)]
pub use {data::*, errors::*, store::*};
