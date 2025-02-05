use super::{super::data::*, bindings::exports::floria::plugins::dispatcher};

use {kutil_cli::debug::*, kutil_std::iter::*, std::io, thiserror::*, wasmtime::component::*};

//
// PluginError
//

/// Plugin error.
#[derive(Error, Debug)]
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
    Dispatch(dispatcher::Error),
}

impl Debuggable for PluginError {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Dispatch(dispatch) => dispatch.write_debug_for(writer, context),

            _ => {
                context.separate(writer)?;
                context.theme.write_error(writer, self)
            }
        }
    }
}

//
// dispatcher::Error
//

impl dispatcher::Error {
    /// Constructor.
    pub fn new(name: String, message: String, arguments: Vec<String>, site: dispatcher::Site) -> Self {
        Self { name, message, arguments, site }
    }

    /// ID.
    pub fn get_id(&self) -> ID {
        self.site.get_id()
    }
}

impl Debuggable for dispatcher::Error {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_meta(writer, &self.site.property_name)?;

        context.indent_into_branch(writer, false)?;
        context.theme.write_name(writer, &self.name)?;
        context.theme.write_delimiter(writer, "(")?;

        for (argument, last) in IterateWithLast::new(&self.arguments) {
            context.theme.write_string(writer, argument)?;
            if !last {
                context.theme.write_delimiter(writer, ",")?;
            }
        }

        context.theme.write_delimiter(writer, ")")?;

        context.indent_into_branch(writer, true)?;
        context.theme.write_error(writer, &self.message)
    }
}

impl dispatcher::Site {
    /// Kind.
    pub fn get_kind(&self) -> Kind {
        Kind::try_from(self.kind.as_str()).unwrap_or_default()
    }

    /// ID.
    pub fn get_id(&self) -> ID {
        ID::parse(self.get_kind(), &self.id)
    }
}

impl Debuggable for dispatcher::Site {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let id = self.get_id();

        context.separate(writer)?;
        id.kind.write_debug_for(writer, context)?;

        let context = context.child().with_separator(true);
        id.write_debug_for(writer, &context)?;

        context.separate(writer)?;
        context.theme.write_name(writer, &self.property_name)
    }
}
