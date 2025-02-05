use super::{super::store::*, event_handler::*, id::*, instance::*, kind::*, metadata::*, namespace::*, property::*};

use {bytestring::*, compris::normal::*, std::collections::*};

//
// Template
//

/// Template.
#[derive(Clone, Debug)]
pub struct Template<AnnotatedT> {
    /// ID.
    pub id: ID,

    /// Metadata.
    pub metadata: Metadata<AnnotatedT>,

    /// Group IDs.
    pub group_ids: Vec<ID>,

    /// Property templates.
    pub property_templates: BTreeMap<ByteString, Property<AnnotatedT>>,

    /// Event handlers.
    pub event_handlers: Vec<EventHandler<AnnotatedT>>,
}

impl<AnnotatedT> Template<AnnotatedT> {
    /// Constructor.
    pub fn new(id: ID) -> Self
    where
        AnnotatedT: Default,
    {
        Self {
            id,
            metadata: Metadata::default(),
            group_ids: Vec::new(),
            property_templates: BTreeMap::new(),
            event_handlers: Vec::new(),
        }
    }

    /// To [Variant].
    pub fn to_variant<'own, StoreT>(
        &self,
        map: &mut BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>,
        embedded: bool,
        store: &'own StoreT,
    ) -> Result<(), StoreError>
    where
        AnnotatedT: Clone + Default,
        StoreT: Store<AnnotatedT>,
    {
        map.insert("kind".into(), self.id.kind.to_string().into());
        map.insert("id".into(), self.id.to_string().into());
        map.insert("metadata".into(), self.metadata.clone().into());

        if !self.group_ids.is_empty() {
            if embedded {
                let mut groups = Vec::with_capacity(self.group_ids.len());
                for group_id in &self.group_ids {
                    match store.get_group(group_id)? {
                        Some(group) => groups.push(group.to_variant()),
                        None => {}
                    }
                }
                map.insert("groups".into(), groups.into());
            } else {
                let group_ids: Vec<Variant<_>> = self.group_ids.iter().map(|id| id.to_string().into()).collect();
                map.insert("group_ids".into(), group_ids.into());
            }
        }

        if !self.property_templates.is_empty() {
            let mut property_templates = BTreeMap::new();
            for (property_name, property_template) in &self.property_templates {
                property_templates.insert(property_name.clone().into(), property_template.to_variant(embedded, store)?);
            }
            map.insert("property_templates".into(), property_templates.into());
        }

        Ok(())
    }

    /// Instantiate.
    pub fn instantiate<StoreT>(
        &self,
        kind: Kind,
        namespace: &Namespace,
        store: &StoreT,
    ) -> Result<Instance<AnnotatedT>, StoreError>
    where
        AnnotatedT: Clone + Default,
        StoreT: Store<AnnotatedT>,
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
