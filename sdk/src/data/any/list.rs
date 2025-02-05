use super::super::super::dispatch_bindings::*;

use std::{
    cmp::*,
    fmt::{self, Write},
    hash::*,
};

impl AnyList {
    /// To list.
    pub fn to_list(&self) -> &List {
        self.get()
    }

    /// To list.
    pub fn to_list_mut(&mut self) -> &mut List {
        self.get_mut()
    }
}

//
// List
//

/// List.
#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct List {
    /// Inner.
    pub inner: Vec<Any>,
}

impl GuestAnyList for List {
    fn new(list: Vec<Any>) -> Self {
        Self::from(list)
    }

    fn get(&self) -> Vec<Any> {
        self.clone().inner
    }

    fn length(&self) -> u64 {
        self.inner.len() as u64
    }
}

impl From<Vec<Any>> for List {
    fn from(inner: Vec<Any>) -> Self {
        Self { inner }
    }
}

impl FromIterator<Any> for List {
    fn from_iter<IntoIteratorT>(iter: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = Any>,
    {
        Self::new(Vec::from_iter(iter))
    }
}

impl fmt::Display for List {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_char('[')?;

        let mut iterator = self.inner.iter().peekable();
        while let Some(item) = iterator.next() {
            fmt::Display::fmt(item, formatter)?;
            if iterator.peek().is_some() {
                formatter.write_char(',')?;
            }
        }

        formatter.write_char(']')
    }
}
