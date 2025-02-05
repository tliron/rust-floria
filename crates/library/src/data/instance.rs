use super::{super::store::*, id::*, kind::*, metadata::*, namespace::*, property::*};

use {compris::normal::*, std::collections::*};

//
// Instance
//

/// Instance.
#[derive(Clone, Debug)]
pub struct Instance {
    /// ID.
    pub id: ID,

    /// Origin template ID.
    pub origin_template_id: Option<ID>,

    /// Metadata.
    pub metadata: Metadata,

    /// Type IDs.
    pub type_ids: Vec<ID>,

    /// Properties.
    pub properties: BTreeMap<String, Property>,
}

impl Instance {
    /// Constructor.
    pub fn new_for(kind: Kind, namespace: Namespace, id: String, origin_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(kind, namespace, id), origin_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, origin_template_id: Option<ID>) -> Self {
        Self { id, origin_template_id, metadata: Metadata::new(), type_ids: Vec::new(), properties: BTreeMap::new() }
    }

    /// To [Value].
    pub fn to_value<'own, StoreT>(
        &self,
        map: &mut BTreeMap<Value, Value>,
        store: &'own StoreT,
    ) -> Result<(), StoreError>
    where
        StoreT: StoreClient,
    {
        map.insert("kind".into(), self.id.kind.to_string().into());
        map.insert("id".into(), self.id.to_string().into());
        if let Some(origin_template_id) = &self.origin_template_id {
            map.insert("origin_template_id".into(), origin_template_id.to_string().into());
        }
        map.insert("metadata".into(), self.metadata.clone().into());

        if !self.type_ids.is_empty() {
            let mut types = Vec::with_capacity(self.type_ids.len());
            for type_id in &self.type_ids {
                match store.get_type(type_id)? {
                    Some(type_) => types.push(type_.to_value()),
                    None => {}
                }
            }
            map.insert("types".into(), types.into());
        }

        if !self.properties.is_empty() {
            let mut properties = BTreeMap::new();
            for (property_name, property) in &self.properties {
                properties.insert(property_name.clone().into(), property.to_value());
            }
            map.insert("properties".into(), properties.into());
        }

        Ok(())
    }

    /// Update.
    #[cfg(feature = "plugins")]
    pub fn update<StoreT, ErrorRecipientT>(
        &mut self,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), super::super::ImperativeError>
    where
        StoreT: super::super::StoreClient,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::ImperativeError>,
    {
        for (property_name, property) in self.properties.iter_mut() {
            let site = super::super::plugins::Site::new(self.id.clone(), property_name.clone());
            property.update(&site, library, plugin_name, errors)?;
        }
        Ok(())
    }
}
