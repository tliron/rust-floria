use super::super::super::dispatch_bindings::*;

use std::{cmp::*, fmt, hash::*};

impl AnyCall {
    /// To call.
    pub fn to_call(&self) -> &Call {
        self.get()
    }

    /// To call.
    pub fn to_call_mut(&mut self) -> &mut Call {
        self.get_mut()
    }
}

//
// Call
//

/// Call.
#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Call {
    /// Name.
    pub name: String,

    /// Arguments.
    pub arguments: Vec<Any>,
}

impl GuestAnyCall for Call {
    fn new(name: String, arguments: Vec<Any>) -> Self {
        Self { name, arguments }
    }

    fn get(&self) -> (String, Vec<Any>) {
        (self.name.clone(), self.arguments.clone())
    }
}

impl fmt::Display for Call {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arguments: Vec<_> = self.arguments.iter().map(|argument| argument.to_string()).collect();
        write!(formatter, "{}({})", self.name, arguments.join(","))
    }
}
