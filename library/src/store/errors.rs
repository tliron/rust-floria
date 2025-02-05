use {
    kutil::cli::depict::*,
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

impl Depict for StoreError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
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
