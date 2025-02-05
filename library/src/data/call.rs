use super::expression::*;

use {
    bytestring::*,
    compris::{annotate::*, normal::*},
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
    pub name: ByteString,

    /// Arguments.
    pub arguments: Vec<Expression>,
}

impl Call {
    /// Constructor.
    pub fn new(name: ByteString, arguments: Vec<Expression>) -> Self {
        Self { name, arguments }
    }

    /// To [Variant].
    pub fn to_variant<AnnotatedT>(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut map = BTreeMap::default();

        map.insert("name".into(), self.name.clone().into());

        if !self.arguments.is_empty() {
            let mut arguments = Vec::with_capacity(self.arguments.len());
            for argument in &self.arguments {
                arguments.push(argument.to_variant());
            }
            map.insert("arguments".into(), arguments.into());
        }

        map.into()
    }

    /// Evaluate.
    #[cfg(feature = "plugins")]
    pub fn evaluate<StoreT, AnnotatedT>(
        &self,
        site: &super::super::plugins::Site,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
    ) -> Result<compris::normal::Variant<AnnotatedT>, super::super::FloriaError>
    where
        StoreT: super::super::Store,
        AnnotatedT: Annotated + Default,
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
