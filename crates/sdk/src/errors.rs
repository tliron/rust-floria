use super::{dispatcher_bindings::*, normal::*};

impl Error {
    /// Constructor.
    pub fn new(message: &str, name: &str, arguments: &Vec<Value>, site: &Site) -> Self {
        let arguments = normal_vec_to_strings(arguments);
        Self { name: name.into(), arguments, site: site.clone(), message: message.into() }
    }
}
