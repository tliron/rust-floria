use super::{super::store::*, directory::*, id::*, kind::*, metadata::*, property::*};

use {
    compris::{annotate::*, normal::*},
    kutil::std::immutable::*,
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

    /// Class IDs.
    pub class_ids: Vec<ID>,

    /// Properties.
    pub properties: BTreeMap<ByteString, Property>,
}

impl Instance {
    /// Constructor.
    pub fn new_for(kind: Kind, directory: Directory, id: ByteString, origin_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(kind, directory, id), origin_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, origin_template_id: Option<ID>) -> Self {
        Self {
            id,
            origin_template_id,
            metadata: Default::default(),
            class_ids: Default::default(),
            properties: Default::default(),
        }
    }

    /// To Compris variant.
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

        if !self.class_ids.is_empty() {
            if embedded {
                let mut classes = List::new_with_capacity(self.class_ids.len());
                for class_id in &self.class_ids {
                    if let Some(class) = store.get_class(class_id)? {
                        classes.inner.push(class.to_variant());
                    }
                }
                map.into_insert("classes", classes);
            } else {
                let class_ids: List<_> = self.class_ids.iter().map(|id| id.to_string().into()).collect();
                map.into_insert("class_ids", class_ids);
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
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, super::super::FloriaError>
    where
        StoreT: Clone + Send + super::super::Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        let mut updated = false;

        for (property_name, property) in self.properties.iter_mut() {
            if property.update(&self.id, &property_name, library, errors)? {
                updated = true;
            }
        }

        Ok(updated)
    }

    /// Validate.
    #[cfg(feature = "plugins")]
    pub fn validate<StoreT, ErrorRecipientT>(
        &mut self,
        library: &mut super::super::plugins::Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, super::super::FloriaError>
    where
        StoreT: Clone + Send + super::super::Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        let mut valid = true;

        for (property_name, property) in self.properties.iter_mut() {
            if !property.validate(&self.id, &property_name, library, errors)? {
                valid = false;
            }
        }

        Ok(valid)
    }
}
