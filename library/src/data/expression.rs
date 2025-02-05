use super::call::*;

use {
    compris::{annotate::*, normal::*},
    kutil_cli::debug::*,
    std::{collections::*, fmt, io},
};

//
// Expression
//

/// Expression.
#[derive(Clone, Debug)]
pub enum Expression<AnnotatedT> {
    /// Literal.
    Literal(Variant<AnnotatedT>),

    /// Call.
    Call(Call<AnnotatedT>),
}

impl<AnnotatedT> Expression<AnnotatedT> {
    /// True if literal undefined.
    pub fn is_undefined(&self) -> bool {
        if let Self::Literal(Variant::Undefined) = self {
            return true;
        }
        return false;
    }

    /// To [Variant].
    pub fn to_variant(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Clone + Default,
    {
        let mut map = BTreeMap::new();

        match self {
            Self::Literal(variant) => {
                map.insert("literal".into(), variant.clone());
            }
            Self::Call(call) => {
                map.insert("call".into(), call.to_variant());
            }
        }

        map.into()
    }

    /// Evaluate the expression.
    #[cfg(feature = "plugins")]
    pub fn evaluate<StoreT>(
        &self,
        site: &super::super::plugins::Site,
        library: &mut super::super::plugins::Library<StoreT, AnnotatedT>,
        plugin_name: &str,
    ) -> Result<Variant<AnnotatedT>, super::super::FloriaError>
    where
        AnnotatedT: Clone + Default,
        StoreT: super::super::Store<AnnotatedT>,
    {
        match self {
            Self::Literal(literal) => Ok(literal.clone()),
            Self::Call(call) => call.evaluate(site, library, plugin_name),
        }
    }
}

impl<AnnotatedT> Annotated for Expression<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn has_annotations() -> bool {
        AnnotatedT::has_annotations()
    }

    fn get_annotations(&self) -> Option<&Annotations> {
        match self {
            Self::Literal(literal) => literal.get_annotations(),
            Self::Call(call) => call.get_annotations(),
        }
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::Literal(literal) => literal.get_annotations_mut(),
            Self::Call(call) => call.get_annotations_mut(),
        }
    }

    fn set_annotations(&mut self, annotations: Annotations) {
        match self {
            Self::Literal(literal) => literal.set_annotations(annotations),
            Self::Call(call) => call.set_annotations(annotations),
        }
    }
}

impl<AnnotatedT> Default for Expression<AnnotatedT> {
    fn default() -> Self {
        Self::Literal(Variant::default())
    }
}

impl<AnnotatedT> Debuggable for Expression<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Literal(variant) => {
                variant.write_debug_for(writer, &context.child().with_format(DebugFormat::Compact))
            }
            Self::Call(call) => call.write_debug_for(writer, context),
        }
    }
}

impl<AnnotatedT> fmt::Display for Expression<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(variant) => fmt::Display::fmt(variant, formatter),
            Self::Call(call) => fmt::Display::fmt(call, formatter),
        }
    }
}

impl<AnnotatedT> From<Variant<AnnotatedT>> for Expression<AnnotatedT> {
    fn from(variant: Variant<AnnotatedT>) -> Self {
        Self::Literal(variant)
    }
}

impl<AnnotatedT> From<Call<AnnotatedT>> for Expression<AnnotatedT> {
    fn from(call: Call<AnnotatedT>) -> Self {
        Self::Call(call)
    }
}
