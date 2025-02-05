use super::{data::*, store::*};

use {
    ahash::*,
    kutil_cli::debug::*,
    kutil_std::{error::*, iter::*},
    std::io,
    thiserror::*,
};

//
// ImperativeError
//

/// Imperative error.
#[derive(Error, Debug)]
pub enum ImperativeError {
    /// Store.
    #[error("store: {0}")]
    Store(#[from] StoreError),

    /// Plugin.
    #[cfg(feature = "plugins")]
    #[error("plugin: {0}")]
    Plugin(#[from] super::plugins::PluginError),
}

impl ImperativeError {
    /// ID.
    pub fn get_id(&self) -> Option<ID> {
        match self {
            ImperativeError::Store(_) => None,
            #[cfg(feature = "plugins")]
            ImperativeError::Plugin(plugin) => match plugin {
                super::plugins::PluginError::Dispatch(dispatch) => Some(dispatch.get_id()),
                _ => None,
            },
        }
    }
}

impl Debuggable for ImperativeError {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Store(store) => store.write_debug_for(writer, context),
            #[cfg(feature = "plugins")]
            Self::Plugin(plugin) => plugin.write_debug_for(writer, context),
        }
    }
}

//
// ImperativeErrors
//

/// Imperative errors.
pub trait ImperativeErrors {
    /// To [Debuggable].
    fn to_debuggable(&self, heading: &str) -> DebuggableImperativeErrors;
}

impl ImperativeErrors for Errors<ImperativeError> {
    fn to_debuggable(&self, heading: &str) -> DebuggableImperativeErrors {
        DebuggableImperativeErrors { heading: heading.into(), errors: self }
    }
}

//
// DebuggableImperativeErrors
//

/// Debuggable imperative errors.
pub struct DebuggableImperativeErrors<'own> {
    heading: String,
    errors: &'own Errors<ImperativeError>,
}

impl<'own> Debuggable for DebuggableImperativeErrors<'own> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        utils::write_debug_heading(&self.heading, writer, context, |writer, context| -> io::Result<()> {
            let mut table = HashMap::<_, Vec<_>>::new();
            for error in self.errors {
                let id = error.get_id();

                match table.get_mut(&id) {
                    Some(list) => list.push(error),
                    None => {
                        let mut list = Vec::new();
                        list.push(error);
                        table.insert(id, list);
                    }
                }
            }

            for ((id, list), first) in IterateWithFirst::new(table) {
                context.separate_or_indent(writer, first)?;

                match id {
                    Some(id) => {
                        id.kind.write_debug_for(writer, context)?;
                        write!(writer, " ")?;
                        id.write_debug_for(writer, context)?;
                    }
                    None => {
                        context.theme.write_meta(writer, "general")?;
                    }
                }

                for (error, last) in IterateWithLast::new(list) {
                    context.indent_into_branch(writer, last)?;
                    error.write_debug_for(
                        writer,
                        &context.child().with_separator(true).increase_indentation_branch(last),
                    )?;
                }
            }

            Ok(())
        })
    }
}
