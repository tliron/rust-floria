use super::call::*;

use {
    compris::normal::*,
    kutil_cli::debug::*,
    std::{collections::*, fmt, io},
};

//
// Expression
//

/// Expression.
#[derive(Clone, Debug)]
pub enum Expression {
    /// Literal.
    Literal(Value),

    /// Call.
    Call(Call),
}

impl Expression {
    /// True if literal nothing.
    pub fn is_nothing(&self) -> bool {
        if let Self::Literal(Value::Nothing) = self {
            return true;
        }
        return false;
    }

    /// To [Value].
    pub fn to_value(&self) -> Value {
        let mut map = BTreeMap::new();

        match self {
            Self::Literal(value) => {
                map.insert("literal".into(), value.clone());
            }
            Self::Call(call) => {
                map.insert("call".into(), call.to_value());
            }
        }

        map.into()
    }

    /// Evaluate the expression.
    #[cfg(feature = "plugins")]
    pub fn evaluate<StoreT>(
        &self,
        site: &super::super::plugins::Site,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
    ) -> Result<Value, super::super::ImperativeError>
    where
        StoreT: super::super::StoreClient,
    {
        match self {
            Self::Literal(literal) => Ok(literal.clone()),
            Self::Call(call) => call.evaluate(site, library, plugin_name),
        }
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self::Literal(Value::default())
    }
}

impl Debuggable for Expression {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Literal(value) => value.write_debug_for(writer, &context.child().with_format(DebugFormat::Compact)),
            Self::Call(call) => call.write_debug_for(writer, context),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(value) => fmt::Display::fmt(value, formatter),
            Self::Call(call) => fmt::Display::fmt(call, formatter),
        }
    }
}

impl From<Value> for Expression {
    fn from(value: Value) -> Self {
        Self::Literal(value)
    }
}

impl From<Call> for Expression {
    fn from(call: Call) -> Self {
        Self::Call(call)
    }
}
