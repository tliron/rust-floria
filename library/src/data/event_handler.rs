use super::call::*;

use kutil::std::immutable::*;

//
// EventHandler
//

/// Event handler.
///
/// Equivalent to TOSCA operation or notification.
#[derive(Clone, Debug)]
pub struct EventHandler {
    /// Event ID.
    pub event_id: ByteString,

    /// Call.
    pub call: Call,
}
