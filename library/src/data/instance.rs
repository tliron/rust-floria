use super::{super::store::*, id::*, kind::*, metadata::*, property::*, scope::*};

use {
    compris::{annotate::*, normal::*},
    kutil::std::zerocopy::*,
    std::collections::*,
};

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
    pub properties: BTreeMap<ByteString, Property>,
}

impl Instance {
    /// Constructor.
    pub fn new_for(kind: Kind, scope: Scope, id: ByteString, origin_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(kind, scope, id), origin_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, origin_template_id: Option<ID>) -> Self {
        Self {
            id,
            origin_template_id,
            metadata: Default::default(),
            group_ids: Default::default(),
            properties: Default::default(),
        }
    }

    /// To [Variant].
    pub fn to_variant<'own, StoreT, AnnotatedT>(
        &self,
        map: &mut Map<AnnotatedT>,
        embedded: bool,
        store: &'own StoreT,
    ) -> Result<(), StoreError>
    where
        AnnotatedT: Annotated + Clone + Default,
        StoreT: Store,
    {
        map.into_insert("kind", self.id.kind.to_string());
        map.into_insert("id", self.id.to_string());
        if let Some(origin_template_id) = &self.origin_template_id {
            map.into_insert("origin_template_id", origin_template_id.to_string());
        }
        map.into_insert("metadata", self.metadata.clone().into_annotated());

        if !self.group_ids.is_empty() {
            if embedded {
                let mut groups = List::new_with_capacity(self.group_ids.len());
                for group_id in &self.group_ids {
                    if let Some(group) = store.get_group(group_id)? {
                        groups.inner.push(group.to_variant());
                    }
                }
                map.into_insert("groups", groups);
            } else {
                let group_ids: List<_> = self.group_ids.iter().map(|id| id.to_string().into()).collect();
                map.into_insert("group_ids", group_ids);
            }
        }

        if !self.properties.is_empty() {
            let mut properties = Map::default();
            for (property_name, property) in &self.properties {
                properties.into_insert(property_name.clone(), property.to_variant(embedded, store)?);
            }
            map.into_insert("properties", properties);
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
    ) -> Result<(), super::super::FloriaError>
    where
        StoreT: super::super::Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        for (property_name, property) in self.properties.iter_mut() {
            let site = super::super::plugins::Site::new(self.id.clone(), Some(property_name.clone().into()));
            property.update(&site, library, plugin_name, errors)?;
        }
        Ok(())
    }
}
