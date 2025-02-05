use super::bindings::exports::floria::plugins::dispatch;

use {
    kutil::{cli::depict::*, std::iter::*},
    std::io,
    thiserror::*,
    wasmtime::component::*,
};

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

//
// dispatch::Error
//

impl dispatch::Error {
    /// Constructor.
    pub fn new(name: String, message: String, arguments: Vec<String>, site: dispatch::Site) -> Self {
        Self { name, message, arguments, site }
    }
}

impl Depict for dispatch::Error {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;

        match &self.site.property_name {
            Some(property_name) => context.theme.write_meta(writer, property_name)?,
            None => context.theme.write_meta(writer, "no property")?,
        }

        context.indent_into_branch(writer, false)?;
        context.theme.write_name(writer, &self.name)?;
        context.theme.write_delimiter(writer, '(')?;

        for (argument, last) in IterateWithLast::new(&self.arguments) {
            context.theme.write_string(writer, argument)?;
            if !last {
                context.theme.write_delimiter(writer, ',')?;
            }
        }

        context.theme.write_delimiter(writer, ')')?;

        context.indent_into_branch(writer, true)?;
        context.theme.write_error(writer, &self.message)
    }
}
