use super::{data::*, store::*};

use {
    kutil::{
        cli::debug::*,
        std::{collections::*, error::*, iter::*},
    },
    std::io,
    thiserror::*,
};

//
// FloriaError
//

/// Floria error.
#[derive(Debug, Debuggable, Error)]
pub enum FloriaError {
    /// Instantiation.
    #[error("instantiation: {0}")]
    Instantiation(String),

    /// Store.
    #[error("store: {0}")]
    #[debuggable(as(debuggable))]
    Store(#[from] StoreError),

    /// Plugin.
    #[cfg(feature = "plugins")]
    #[error("plugin: {0}")]
    #[debuggable(as(debuggable))]
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

//
// FloriaErrors
//

/// Floria errors.
pub trait FloriaErrors {
    /// To [Debuggable].
    fn to_debuggable(&self, heading: &str) -> DebuggableFloriaErrors<'_>;
}

impl FloriaErrors for Errors<FloriaError> {
    fn to_debuggable(&self, heading: &str) -> DebuggableFloriaErrors<'_> {
        DebuggableFloriaErrors { heading: heading.into(), errors: self }
    }
}

//
// DebuggableFloriaErrors
//

/// Debuggable [FloriaErrors].
pub struct DebuggableFloriaErrors<'own> {
    heading: String,
    errors: &'own Errors<FloriaError>,
}

impl<'own> Debuggable for DebuggableFloriaErrors<'own> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        utils::write_debug_heading(&self.heading, writer, context, |writer, context| -> io::Result<()> {
            let mut table = FastHashMap::<_, Vec<_>>::default();
            for error in self.errors {
                let id = error.get_id();

                match table.get_mut(&id) {
                    Some(list) => list.push(error),
                    None => {
                        let mut list = Vec::default();
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
