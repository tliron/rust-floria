use super::call::*;

use {
    compris::{annotate::*, normal::*},
    kutil_cli::debug::*,
    std::{fmt, io},
};

//
// Expression
//

/// Expression.
#[derive(Clone, Debug)]
pub enum Expression {
    /// Literal.
    Literal(Variant<WithoutAnnotations>),

    /// Call.
    Call(Call),
}

impl Expression {
    /// True if literal undefined.
    pub fn is_undefined(&self) -> bool {
        if let Self::Literal(Variant::Undefined) = self {
            return true;
        }
        return false;
    }

    /// To [Variant].
    pub fn to_variant<AnnotatedT>(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut map = Map::default();

        match self {
            Self::Literal(literal) => {
                map.into_insert("literal", literal.clone().into_annotated());
            }

            Self::Call(call) => {
                map.into_insert("call", call.to_variant());
            }
        }

        map.into()
    }

    /// Evaluate the expression.
    #[cfg(feature = "plugins")]
    pub fn evaluate<StoreT, AnnotatedT>(
        &self,
        site: &super::super::plugins::Site,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
    ) -> Result<Variant<AnnotatedT>, super::super::FloriaError>
    where
        AnnotatedT: Annotated + Default,
        StoreT: super::super::Store,
    {
        match self {
            Self::Literal(literal) => Ok(literal.clone().into_annotated()),
            Self::Call(call) => call.evaluate(site, library, plugin_name),
        }
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self::Literal(Variant::default())
    }
}

impl Debuggable for Expression {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Literal(literal) => {
                literal.write_debug_for(writer, &context.child().with_format(DebugFormat::Compact))
            }

            Self::Call(call) => call.write_debug_for(writer, context),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => fmt::Display::fmt(literal, formatter),
            Self::Call(call) => fmt::Display::fmt(call, formatter),
        }
    }
}

impl<AnnotatedT> From<Variant<AnnotatedT>> for Expression
where
    AnnotatedT: Annotated,
{
    fn from(variant: Variant<AnnotatedT>) -> Self {
        Self::Literal(variant.into_annotated())
    }
}

impl From<Call> for Expression {
    fn from(call: Call) -> Self {
        Self::Call(call)
    }
}
