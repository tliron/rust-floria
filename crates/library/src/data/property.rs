use super::call::*;

use {compris::normal::*, kutil_cli::debug::*, std::collections::*};

//
// Property
//

/// Property.
///
/// Equivalent to TOSCA property or attribute.
#[derive(Clone, Debug, Debuggable)]
pub struct Property {
    /// Value.
    #[debuggable(option, as(debuggable))]
    pub value: Option<Value>,

    /// Updater.
    #[debuggable(option, as(debuggable))]
    pub updater: Option<Call>,

    /// Validator.
    #[debuggable(option, as(debuggable))]
    pub validator: Option<Call>,

    /// Read-only.
    #[debuggable(style(bare))]
    pub read_only: bool,
}

impl Property {
    /// Constructor.
    pub fn new(value: Option<Value>, updater: Option<Call>, validator: Option<Call>, read_only: bool) -> Self {
        Self { value, updater, validator, read_only }
    }

    /// To [Value].
    pub fn to_value(&self) -> Value {
        let mut map = BTreeMap::new();

        if let Some(value) = &self.value {
            map.insert("value".into(), value.clone());
        }
        if let Some(updater) = &self.updater {
            map.insert("updater".into(), updater.to_value());
        }
        if let Some(validator) = &self.validator {
            map.insert("validator".into(), validator.to_value());
        }
        map.insert("read_only".into(), self.read_only.into());

        map.into()
    }

    /// Update.
    #[cfg(feature = "plugins")]
    pub fn update<StoreT, ErrorRecipientT>(
        &mut self,
        site: &super::super::plugins::Site,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), super::super::ImperativeError>
    where
        StoreT: super::super::StoreClient,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::ImperativeError>,
    {
        use kutil_std::error::*;
        if let Some(updater) = &self.updater {
            self.value = updater.evaluate(site, library, plugin_name).map(|v| Some(v)).give_or(None, errors)?;
        };
        Ok(())
    }
}
