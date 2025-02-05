use {
    kutil_cli::debug::*,
    std::{io, sync::*},
    thiserror::*,
};

//
// StoreError
//

/// Store error.
#[derive(Debug, Error)]
pub enum StoreError {
    /// ID.
    #[error("ID: {0}")]
    ID(String),

    /// Concurrency.
    #[error("concurrency: {0}")]
    Concurrency(String),
}

impl Debuggable for StoreError {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::ID(id) => write!(writer, "ID: {}", context.theme.error(id)),
            Self::Concurrency(concurrency) => write!(writer, "concurrency: {}", context.theme.error(concurrency)),
        }
    }
}

impl<GuardT> From<PoisonError<GuardT>> for StoreError {
    fn from(error: PoisonError<GuardT>) -> Self {
        Self::Concurrency(error.to_string())
    }
}
