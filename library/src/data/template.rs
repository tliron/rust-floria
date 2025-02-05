use super::{super::store::*, event_handler::*, id::*, instance::*, kind::*, metadata::*, property::*, scope::*};

use {
    compris::{annotate::*, normal::*},
    kutil::std::zerocopy::*,
    std::collections::*,
};

//
// Template
//

/// Template.
#[derive(Clone, Debug)]
pub struct Template {
    /// ID.
    pub id: ID,

    /// Metadata.
    pub metadata: Metadata,

    /// Group IDs.
    pub group_ids: Vec<ID>,

    /// Property templates.
    pub property_templates: BTreeMap<ByteString, Property>,

    /// Event handlers.
    pub event_handlers: Vec<EventHandler>,
}

impl Template {
    /// Constructor.
    pub fn new(id: ID) -> Self {
        Self {
            id,
            metadata: Default::default(),
            group_ids: Default::default(),
            property_templates: Default::default(),
            event_handlers: Default::default(),
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
        map.into_insert("metadata", self.metadata.clone().into_annotated());

        if !self.group_ids.is_empty() {
            if embedded {
                let mut groups = List::new_with_capacity(self.group_ids.len());
                for group_id in &self.group_ids {
                    match store.get_group(group_id)? {
                        Some(group) => groups.inner.push(group.to_variant()),
                        None => {}
                    }
                }
                map.into_insert("groups", groups);
            } else {
                let group_ids: List<_> = self.group_ids.iter().map(|id| id.to_string().into()).collect();
                map.into_insert("group_ids", group_ids);
            }
        }

        if !self.property_templates.is_empty() {
            let mut property_templates = Map::default();
            for (property_name, property_template) in &self.property_templates {
                property_templates.into_insert(property_name.clone(), property_template.to_variant(embedded, store)?);
            }
            map.into_insert("property_templates", property_templates);
        }

        Ok(())
    }

    /// Instantiate.
    pub fn instantiate<StoreT>(&self, kind: Kind, scope: &Scope, store: &StoreT) -> Result<Instance, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(kind, scope.clone());
        store.create_id(&mut id)?;

        let mut instance = Instance::new_with(id, Some(self.id.clone()));
        instance.metadata = self.metadata.clone();
        instance.group_ids = self.group_ids.clone();
        instance.properties = self.property_templates.clone();

        Ok(instance)
    }
}
