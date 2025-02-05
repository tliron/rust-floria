use super::super::{super::data::*, bindings::exports::floria::plugins::dispatch};

use {kutil::cli::depict::*, std::io};

impl dispatch::Site {
    /// Constructor.
    pub fn new(id: ID, property_name: Option<String>) -> Self {
        Self { id: id.into(), property_name }
    }
}

impl Depict for dispatch::Site {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        let id: ID = self.id.clone().into();
        id.kind.depict(writer, context)?;
        id.depict(writer, &context.child().with_separator(true))?;
        match &self.property_name {
            Some(property_name) => {
                context.separate(writer)?;
                context.theme.write_name(writer, property_name)
            }

            None => Ok(()),
        }
    }
}
