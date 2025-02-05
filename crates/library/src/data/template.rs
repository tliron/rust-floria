use super::{super::store::*, event_handler::*, id::*, instance::*, kind::*, metadata::*, namespace::*, property::*};

use {compris::normal::*, kutil_cli::debug::*, std::collections::*};

//
// Template
//

/// Template.
#[derive(Clone, Debug, Debuggable)]
pub struct Template {
    /// ID.
    pub id: ID,

    /// Metadata.
    pub metadata: Metadata,

    /// Group IDs.
    pub group_ids: Vec<ID>,

    /// Property templates.
    pub property_templates: BTreeMap<String, Property>,

    /// Event handlers.
    pub event_handlers: Vec<EventHandler>,
}

impl Template {
    /// Constructor.
    pub fn new(id: ID) -> Self {
        Self {
            id,
            metadata: Metadata::new(),
            group_ids: Vec::new(),
            property_templates: BTreeMap::new(),
            event_handlers: Vec::new(),
        }
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
        map.insert("metadata".into(), self.metadata.clone().into());

        if !self.group_ids.is_empty() {
            if embedded {
                let mut groups = Vec::with_capacity(self.group_ids.len());
                for group_id in &self.group_ids {
                    match store.get_group(group_id)? {
                        Some(group) => groups.push(group.to_value()),
                        None => {}
                    }
                }
                map.insert("groups".into(), groups.into());
            } else {
                let group_ids: Vec<Value> = self.group_ids.iter().map(|i| i.to_string().into()).collect();
                map.insert("group_ids".into(), group_ids.into());
            }
        }

        if !self.property_templates.is_empty() {
            let mut property_templates = BTreeMap::new();
            for (property_name, property_template) in &self.property_templates {
                property_templates.insert(property_name.clone().into(), property_template.to_value(embedded, store)?);
            }
            map.insert("property_templates".into(), property_templates.into());
        }

        Ok(())
    }

    /// Instantiate.
    pub fn instantiate<StoreT>(&self, kind: Kind, namespace: &Namespace, store: &StoreT) -> Result<Instance, StoreError>
    where
        StoreT: StoreClient,
    {
        let mut id = ID::new(kind, namespace.clone());
        store.create_id(&mut id)?;

        let mut instance = Instance::new_with(id, Some(self.id.clone()));
        instance.metadata = self.metadata.clone();
        instance.group_ids = self.group_ids.clone();
        instance.properties = self.property_templates.clone();

        Ok(instance)
    }
}
