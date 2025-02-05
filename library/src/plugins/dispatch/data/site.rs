use super::super::super::{super::data::*, bindings::exports::floria::plugins::dispatch};

use {
    kutil::{cli::depict::*, std::iter::*},
    std::{fmt, io},
};

impl dispatch::Site {
    /// Constructor.
    pub fn new(id: ID, path: Vec<String>) -> Self {
        Self { id: id.into(), path }
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

        Ok(())
    }
}

impl fmt::Display for dispatch::Site {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id: ID = self.id.clone().into();
        fmt::Display::fmt(&id, formatter)?;
        for segment in &self.path {
            write!(formatter, ".{}", segment)?;
        }
        Ok(())
    }
}
