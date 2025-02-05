use super::super::data::*;

use {
    kutil::{cli::depict::*, std::iter::*},
    std::{fmt, io},
    thiserror::*,
};

//
// InvalidValueError
//

/// Invalid value error.
#[derive(Debug, Error)]
pub struct InvalidValueError {
    /// ID.
    pub id: ID,

    /// Path.
    pub path: Vec<String>,
}

impl InvalidValueError {
    /// Constructor.
    pub fn new(id: ID, path: Vec<String>) -> Self {
        Self { id, path }
    }
}

impl Depict for InvalidValueError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;

        if !self.path.is_empty() {
            for (segment, last) in IterateWithLast::new(&self.path) {
                context.theme.write_meta(writer, segment)?;
                if !last {
                    context.theme.write_delimiter(writer, '.')?;
                }
            }
        } else {
            context.theme.write_meta(writer, "no path")?;
        }

        context.indent_into_branch(writer, true)?;
        context.theme.write_error(writer, "invalid value")
    }
}

impl fmt::Display for InvalidValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.path.is_empty() {
            write!(formatter, "ID: {}, path: {}", self.id, self.path.join("."))
        } else {
            write!(formatter, "ID: {}", self.id)
        }
    }
}
