pub use super::super::{super::data::*, bindings::exports::floria::plugins::dispatcher::Site};

impl Site {
    /// Constructor.
    pub fn new(id: ID, property_name: String) -> Self {
        Self { kind: id.kind.to_string(), id: id.id, property_name }
    }
}
