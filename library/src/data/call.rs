use super::expression::*;

use {
    compris::{impl_annotated, normal::*},
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{collections::*, fmt, io},
};

//
// Call
//

/// Call.
#[derive(Clone, Debug, Default)]
pub struct Call<AnnotatedT> {
    /// Function name.
    pub name: Text<AnnotatedT>,

    /// Arguments.
    pub arguments: Vec<Expression<AnnotatedT>>,
}

impl<AnnotatedT> Call<AnnotatedT> {
    /// Constructor.
    pub fn new(name: Text<AnnotatedT>) -> Self {
        Self { name, arguments: Vec::new() }
    }

    /// To [Variant].
    pub fn to_variant(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Clone + Default,
    {
        let mut map = BTreeMap::new();

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
    pub fn evaluate<StoreT>(
        &self,
        site: &super::super::plugins::Site,
        library: &mut super::super::plugins::Library<StoreT, AnnotatedT>,
        plugin_name: &str,
    ) -> Result<compris::normal::Variant<AnnotatedT>, super::super::FloriaError>
    where
        AnnotatedT: Clone + Default,
        StoreT: super::super::Store<AnnotatedT>,
    {
        let mut arguments = Vec::with_capacity(self.arguments.len());
        for argument in &self.arguments {
            arguments.push(argument.evaluate(site, library, plugin_name)?);
        }

        Ok(library.get_dispatch_plugin(plugin_name)?.dispatch(self.name.as_str(), arguments, &site)?)
    }
}

impl_annotated!(Call, name);

impl<AnnotatedT> Debuggable for Call<AnnotatedT> {
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

impl<AnnotatedT> fmt::Display for Call<AnnotatedT> {
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
