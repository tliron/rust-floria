use super::expression::*;

use {
    compris::{annotate::*, normal::*},
    kutil::{
        cli::depict::*,
        std::{immutable::*, iter::*},
    },
    std::{fmt, io},
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

    /// Evaluate.
    #[cfg(feature = "plugins")]
    pub fn evaluate<StoreT, AnnotatedT>(
        &self,
        site: &super::super::plugins::Site,
        library: &mut super::super::plugins::Library<StoreT>,
    ) -> Result<compris::normal::Variant<AnnotatedT>, super::super::FloriaError>
    where
        StoreT: Clone + Send + super::super::Store,
        AnnotatedT: Annotated + Default,
    {
        tracing::debug!("evaluate: {}", self);

        let (plugin_name, name) = self.name.split_once(':').unwrap_or(("", &self.name));

        let mut arguments = Vec::with_capacity(self.arguments.len());
        for argument in &self.arguments {
            arguments.push(argument.evaluate(site, library)?);
        }

        let plugin = library.get_dispatch_plugin(plugin_name)?;
        let mut plugin = plugin.lock().map_err(super::super::plugins::PluginError::from)?;

        Ok(plugin.dispatch(name, arguments, site)?)
    }
}

impl Depict for Call {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_name(writer, &self.name)?;
        context.theme.write_delimiter(writer, '(')?;

        let child_context = &context.child().with_format(DepictionFormat::Compact).with_separator(false);
        for (argument, last) in IterateWithLast::new(&self.arguments) {
            argument.depict(writer, child_context)?;
            if !last {
                context.theme.write_delimiter(writer, ',')?;
            }
        }

        context.theme.write_delimiter(writer, ')')
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

// Conversions

impl<AnnotatedT> Into<Variant<AnnotatedT>> for &Call
where
    AnnotatedT: Annotated + Default,
{
    fn into(self) -> Variant<AnnotatedT> {
        let mut map = Map::default();

        map.into_insert("$name", self.name.clone());

        if !self.arguments.is_empty() {
            let arguments: List<AnnotatedT> = self.arguments.iter().map(|argument| argument.into()).collect();
            map.into_insert("$arguments", arguments);
        }

        map.into()
    }
}
