use super::{super::normal::*, id::*};

//
// Entity
//

/// Entity.
pub struct Entity {
    /// ID.
    pub id: ID,

    /// Value.
    pub value: Value,
}

impl Entity {
    /// Constructor.
    pub fn new(id: ID, value: Value) -> Self {
        Self { id, value }
    }
}
