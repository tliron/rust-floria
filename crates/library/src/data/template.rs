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

    /// Type IDs.
    pub type_ids: Vec<ID>,

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
            type_ids: Vec::new(),
            property_templates: BTreeMap::new(),
            event_handlers: Vec::new(),
        }
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

        if !self.property_templates.is_empty() {
            let mut property_templates = BTreeMap::new();
            for (property_name, property_template) in &self.property_templates {
                property_templates.insert(property_name.clone().into(), property_template.to_value());
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
        instance.type_ids = self.type_ids.clone();
        instance.properties = self.property_templates.clone();

        Ok(instance)
    }
}
