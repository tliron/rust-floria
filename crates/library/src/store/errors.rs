use {
    kutil_cli::debug::*,
    std::{io, sync::*},
    thiserror::*,
};

//
// StoreError
//

/// Store error.
#[derive(Error, Debug)]
pub enum StoreError {
    /// ID.
    #[error("id: {0}")]
    ID(String),

    /// Concurrency.
    #[error("concurrency: {0}")]
    Concurrency(String),
}

impl Debuggable for StoreError {
    fn write_debug_for<WriteT>(&self, _writer: &mut WriteT, _context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        todo!()
    }
}

impl<GuardT> From<PoisonError<GuardT>> for StoreError {
    fn from(value: PoisonError<GuardT>) -> Self {
        Self::Concurrency(value.to_string())
    }
}
