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

    /// Group IDs.
    pub group_ids: Vec<ID>,

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
        Self { id, origin_template_id, metadata: Metadata::new(), group_ids: Vec::new(), properties: BTreeMap::new() }
    }

    /// To [Value].
    pub fn to_value<'own, StoreT>(
        &self,
        map: &mut BTreeMap<Value, Value>,
        embedded: bool,
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

        if !self.group_ids.is_empty() {
            if embedded {
                let mut groups = Vec::with_capacity(self.group_ids.len());
                for group_id in &self.group_ids {
                    if let Some(group) = store.get_group(group_id)? {
                        groups.push(group.to_value());
                    }
                }
                map.insert("groups".into(), groups.into());
            } else {
                let group_ids: Vec<Value> = self.group_ids.iter().map(|id| id.to_string().into()).collect();
                map.insert("group_ids".into(), group_ids.into());
            }
        }

        if !self.properties.is_empty() {
            let mut properties = BTreeMap::new();
            for (property_name, property) in &self.properties {
                properties.insert(property_name.clone().into(), property.to_value(embedded, store)?);
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
            let site = super::super::plugins::Site::new(self.id.clone(), Some(property_name.clone()));
            property.update(&site, library, plugin_name, errors)?;
        }
        Ok(())
    }
}
