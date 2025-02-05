use super::{data::*, dispatch_bindings::*};

impl Error {
    /// Constructor.
    pub fn new(name: String, arguments: &Vec<Any>, site: Site, message: String) -> Self {
        Self { name, arguments: normal_vec_to_strings(arguments), site, message }
    }
}
