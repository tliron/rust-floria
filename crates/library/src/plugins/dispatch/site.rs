use super::super::{super::data::*, bindings::exports::floria::plugins::dispatch};

use {kutil_cli::debug::*, std::io};

impl dispatch::Site {
    /// Constructor.
    pub fn new(id: ID, property_name: Option<String>) -> Self {
        Self { id: id.into(), property_name }
    }
}

impl Debuggable for dispatch::Site {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        let id: ID = self.id.clone().into();
        id.kind.write_debug_for(writer, context)?;
        id.write_debug_for(writer, &context.child().with_separator(true))?;
        match &self.property_name {
            Some(property_name) => {
                context.separate(writer)?;
                context.theme.write_name(writer, property_name)
            }

            None => Ok(()),
        }
    }
}
