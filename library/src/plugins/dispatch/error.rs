use super::super::bindings::exports::floria::plugins::dispatch;

use {
    kutil::{cli::depict::*, std::iter::*},
    std::{error::*, fmt, io},
};

//
// DispatchError
//

/// Dispatch error.
#[derive(Debug)]
pub struct DispatchError {
    /// Message.
    pub message: String,

    /// Plugin name.
    pub plugin_name: String,

    /// Function name.
    pub name: String,

    /// Function arguments.
    pub arguments: Vec<String>,

    /// Call site.
    pub site: dispatch::Site,
}

impl DispatchError {
    /// Constructor.
    pub fn new(
        message: String,
        plugin_name: String,
        name: String,
        arguments: Vec<String>,
        site: dispatch::Site,
    ) -> Self {
        Self { message, plugin_name, name, arguments, site }
    }
}

impl Depict for DispatchError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;

        if !self.site.path.is_empty() {
            for (segment, last) in IterateWithLast::new(&self.site.path) {
                context.theme.write_meta(writer, segment)?;
                if !last {
                    context.theme.write_delimiter(writer, '.')?;
                }
            }
        } else {
            context.theme.write_meta(writer, "no path")?;
        }

        context.indent_into_branch(writer, false)?;
        context.theme.write_name(writer, &self.plugin_name)?;
        context.theme.write_delimiter(writer, ':')?;
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

impl fmt::Display for DispatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} during {}:{}({}) at {}",
            self.message,
            self.plugin_name,
            self.name,
            self.arguments.join(","),
            &self.site
        )
    }
}

impl Error for DispatchError {}
