use super::super::bindings::exports::floria::plugins::dispatch;

use {
    kutil::{cli::depict::*, std::iter::*},
    std::io,
};

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
