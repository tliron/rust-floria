use super::{data::*, store::*};

use {
    kutil_cli::debug::*,
    kutil_std::{collections::*, error::*, iter::*},
    std::io,
    thiserror::*,
};

//
// FloriaError
//

/// Floria error.
#[derive(Debug, Error)]
pub enum FloriaError {
    /// Instantiation.
    #[error("instantiation: {0}")]
    Instantiation(String),

    /// Store.
    #[error("store: {0}")]
    Store(#[from] StoreError),

    /// Plugin.
    #[cfg(feature = "plugins")]
    #[error("plugin: {0}")]
    Plugin(#[from] super::plugins::PluginError),
}

impl FloriaError {
    /// ID.
    pub fn get_id(&self) -> Option<ID> {
        match self {
            Self::Instantiation(_) | Self::Store(_) => None,

            #[cfg(feature = "plugins")]
            Self::Plugin(plugin) => match plugin {
                super::plugins::PluginError::Dispatch(dispatch) => Some(dispatch.site.id.clone().into()),

                _ => None,
            },
        }
    }
}

impl Debuggable for FloriaError {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Instantiation(instantiation) => {
                context.separate(writer)?;
                write!(writer, "{}", instantiation)
            }

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

impl ImperativeErrors for Errors<FloriaError> {
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
    errors: &'own Errors<FloriaError>,
}

impl<'own> Debuggable for DebuggableImperativeErrors<'own> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        utils::write_debug_heading(&self.heading, writer, context, |writer, context| -> io::Result<()> {
            let mut table = FastHashMap::<_, Vec<_>>::new();
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
                    error.write_debug_for(writer, &context.child().increase_indentation_branch(last))?;
                }
            }

            Ok(())
        })
    }
}
