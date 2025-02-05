use super::call::*;

use {
    compris::{annotate::*, normal::*},
    kutil::cli::depict::*,
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

    /// Evaluate the expression.
    #[cfg(feature = "plugins")]
    pub fn evaluate<StoreT, AnnotatedT>(
        &self,
        site: &super::super::plugins::Site,
        library: &mut super::super::plugins::Library<StoreT>,
    ) -> Result<Variant<AnnotatedT>, super::super::FloriaError>
    where
        AnnotatedT: Annotated + Default,
        StoreT: Clone + Send + super::super::Store,
    {
        match self {
            Self::Literal(literal) => Ok(literal.clone().into_annotated()),
            Self::Call(call) => call.evaluate(site, library),
        }
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self::Literal(Default::default())
    }
}

impl Depict for Expression {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Literal(literal) => literal.depict(writer, &context.child().with_format(DepictionFormat::Compact)),
            Self::Call(call) => call.depict(writer, context),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_plain_depiction().map_err(|_error| fmt::Error)?, formatter)
    }
}

// Conversions

impl From<Call> for Expression {
    fn from(call: Call) -> Self {
        Self::Call(call)
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

impl<AnnotatedT> Into<Variant<AnnotatedT>> for &Expression
where
    AnnotatedT: Annotated + Default,
{
    fn into(self) -> Variant<AnnotatedT> {
        let mut map = Map::default();

        match self {
            Expression::Literal(literal) => {
                map.into_insert("$literal", literal.clone().into_annotated());
            }

            Expression::Call(call) => {
                map.into_insert("$call", call);
            }
        }

        map.into()
    }
}
