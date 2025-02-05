use super::call::*;

use bytestring::*;

//
// EventHandler
//

/// Event handler.
///
/// Equivalent to TOSCA operation or notification.
#[derive(Clone, Debug)]
pub struct EventHandler<AnnotatedT> {
    /// Event ID.
    pub event_id: ByteString,

    /// Call.
    pub call: Call<AnnotatedT>,
}
