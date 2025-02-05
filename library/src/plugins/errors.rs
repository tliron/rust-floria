use super::bindings::exports::floria::plugins::dispatch;

use {kutil::cli::depict::*, std::io, thiserror::*, wasmtime::component::*};

//
// PluginError
//

/// Plugin error.
#[derive(Debug, Error)]
pub enum PluginError {
    /// Not found.
    #[error("not found: {0}")]
    NotFound(String),

    /// Load WASM.
    #[error("load: {0}")]
    LoadWasm(wasmtime::Error),

    /// Link WASM.
    #[error("link: {0}")]
    LinkWasm(wasmtime::Error),

    /// Instantiate WASM.
    #[cfg(feature = "plugins")]
    #[error("instantiate: {0}")]
    InstantiateWasm(wasmtime::Error),

    /// Call WASM.
    #[error("call: {0}")]
    CallWasm(wasmtime::Error),

    /// WASM resource.
    #[error("resource: {0}")]
    WasmResource(ResourceTableError),

    /// Dispatch.
    #[error("dispatch: {0}")]
    Dispatch(dispatch::Error),
}

impl Depict for PluginError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Dispatch(dispatch) => dispatch.depict(writer, context),

            _ => {
                context.separate(writer)?;
                context.theme.write_error(writer, self)
            }
        }
    }
}
