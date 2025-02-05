use super::expression::*;

use {
    compris::normal::*,
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{collections::*, fmt, io},
};

//
// Call
//

/// Call.
#[derive(Clone, Debug, Default)]
pub struct Call {
    /// Function name.
    pub name: String,

    /// Arguments.
    pub arguments: Vec<Expression>,
}

impl Call {
    /// Constructor.
    pub fn new(name: &str) -> Self {
        Self { name: name.into(), arguments: Vec::new() }
    }

    /// To [Value].
    pub fn to_value(&self) -> Value {
        let mut map = BTreeMap::new();

        map.insert("name".into(), self.name.clone().into());

        if !self.arguments.is_empty() {
            let mut arguments = Vec::with_capacity(self.arguments.len());
            for argument in &self.arguments {
                arguments.push(argument.to_value());
            }
            map.insert("arguments".into(), arguments.into());
        }

        map.into()
    }

    /// Evaluate.
    #[cfg(feature = "plugins")]
    pub fn evaluate<StoreT>(
        &self,
        site: &super::super::plugins::Site,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
    ) -> Result<compris::normal::Value, super::super::ImperativeError>
    where
        StoreT: super::super::StoreClient,
    {
        let mut arguments = Vec::with_capacity(self.arguments.len());
        for argument in &self.arguments {
            arguments.push(argument.evaluate(site, library, plugin_name)?);
        }

        Ok(library.get_dispatch_plugin(plugin_name)?.dispatch(&self.name, arguments, &site)?)
    }
}

impl Debuggable for Call {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_name(writer, &self.name)?;
        context.theme.write_delimiter(writer, "(")?;

        let child_context = &context.child().with_format(DebugFormat::Compact).with_separator(false);
        for (argument, last) in IterateWithLast::new(&self.arguments) {
            argument.write_debug_for(writer, child_context)?;
            if !last {
                context.theme.write_delimiter(writer, ",")?;
            }
        }

        context.theme.write_delimiter(writer, ")")
    }
}

impl fmt::Display for Call {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}(", self.name)?;

        for (argument, last) in IterateWithLast::new(&self.arguments) {
            fmt::Display::fmt(argument, formatter)?;
            if !last {
                write!(formatter, ",")?;
            }
        }

        write!(formatter, ")")
    }
}
