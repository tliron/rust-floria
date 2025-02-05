use super::super::data::*;

use {
    kutil::cli::depict::*,
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

    /// Property name.
    pub property_name: Option<String>,
}

impl InvalidValueError {
    /// Constructor.
    pub fn new(id: ID, property_name: Option<String>) -> Self {
        Self { id, property_name }
    }
}

impl Depict for InvalidValueError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;

        match &self.property_name {
            Some(property_name) => context.theme.write_meta(writer, property_name)?,
            None => context.theme.write_meta(writer, "no property")?,
        }

        context.indent_into_branch(writer, true)?;
        context.theme.write_error(writer, "invalid value")
    }
}

impl fmt::Display for InvalidValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.property_name {
            Some(property_name) => write!(formatter, "ID: {}, property: {}", self.id, property_name),
            None => write!(formatter, "ID: {}", self.id),
        }
    }
}
